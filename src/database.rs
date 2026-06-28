//! Fast SQLite database with connection pooling

use rusqlite::{Connection, params, Result as SqlResult};
use std::sync::{Arc, Mutex};
use crate::pipeline::{ProductIdea, PipelineStage, Priority};
use crate::scheduler::ScheduledTask;
use crate::license_manager::License;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> SqlResult<Self> {
        let conn = Connection::open("dpf_data.db")?;
        
        // Enable WAL mode for better concurrency
        conn.execute("PRAGMA journal_mode=WAL;", [])?;
        conn.execute("PRAGMA synchronous=NORMAL;", [])?;
        conn.execute("PRAGMA cache_size=10000;", [])?;
        conn.execute("PRAGMA temp_store=memory;", [])?;
        
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        
        db.init_tables()?;
        
        Ok(db)
    }
    
    fn init_tables(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        
        // Pipeline ideas table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ideas (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                stage TEXT NOT NULL,
                product_type TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                priority TEXT NOT NULL,
                tags TEXT,
                estimated_value REAL,
                actual_value REAL,
                notes TEXT,
                platform TEXT
            )",
            [],
        )?;
        
        // Scheduled tasks table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS scheduled_tasks (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                task_type TEXT NOT NULL,
                schedule TEXT NOT NULL,
                next_run TEXT,
                last_run TEXT,
                status TEXT NOT NULL,
                data TEXT
            )",
            [],
        )?;
        
        // License table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS licenses (
                key TEXT PRIMARY KEY,
                tier TEXT NOT NULL,
                max_devices INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                expires_at TEXT,
                status TEXT NOT NULL,
                activated_devices TEXT,
                metadata TEXT
            )",
            [],
        )?;
        
        // Products table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                product_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                status TEXT NOT NULL,
                file_path TEXT,
                metadata TEXT
            )",
            [],
        )?;
        
        // Create indexes for fast queries
        conn.execute("CREATE INDEX IF NOT EXISTS idx_ideas_stage ON ideas(stage);", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_ideas_priority ON ideas(priority);", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_tasks_status ON scheduled_tasks(status);", [])?;
        
        Ok(())
    }
    
    // Pipeline operations
    pub fn load_ideas(&self) -> SqlResult<Vec<ProductIdea>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, stage, product_type, 
                    created_at, updated_at, priority, tags, 
                    estimated_value, actual_value, notes, platform 
             FROM ideas ORDER BY updated_at DESC"
        )?;
        
        let ideas = stmt.query_map([], |row| {
            Ok(ProductIdea {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                stage: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or(PipelineStage::Idea),
                product_type: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                priority: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or(Priority::Medium),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                estimated_value: row.get(9)?,
                actual_value: row.get(10)?,
                notes: row.get(11)?,
                platform: serde_json::from_str(&row.get::<_, String>(12)?).unwrap_or_default(),
            })
        })?;
        
        ideas.collect()
    }
    
    pub fn save_idea(&self, idea: &ProductIdea) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO ideas 
             (id, title, description, stage, product_type, created_at, updated_at, 
              priority, tags, estimated_value, actual_value, notes, platform)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                idea.id,
                idea.title,
                idea.description,
                serde_json::to_string(&idea.stage).unwrap(),
                idea.product_type,
                idea.created_at.to_rfc3339(),
                idea.updated_at.to_rfc3339(),
                serde_json::to_string(&idea.priority).unwrap(),
                serde_json::to_string(&idea.tags).unwrap(),
                idea.estimated_value,
                idea.actual_value,
                idea.notes,
                serde_json::to_string(&idea.platform).unwrap(),
            ],
        )?;
        Ok(())
    }
    
    pub fn delete_idea(&self, id: usize) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM ideas WHERE id = ?1", params![id])?;
        Ok(())
    }
    
    // License operations
    pub fn save_license(&self, license: &License) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO licenses 
             (key, tier, max_devices, created_at, expires_at, status, activated_devices, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                license.key,
                serde_json::to_string(&license.tier).unwrap(),
                license.max_devices,
                license.created_at.to_rfc3339(),
                license.expires_at.map(|d| d.to_rfc3339()),
                serde_json::to_string(&license.status).unwrap(),
                serde_json::to_string(&license.activated_devices).unwrap(),
                license.metadata.clone(),
            ],
        )?;
        Ok(())
    }
    
    pub fn get_license(&self, key: &str) -> SqlResult<Option<License>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT key, tier, max_devices, created_at, expires_at, 
                    status, activated_devices, metadata 
             FROM licenses WHERE key = ?1"
        )?;
        
        let mut rows = stmt.query(params![key])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(License {
                key: row.get(0)?,
                tier: serde_json::from_str(&row.get::<_, String>(1)?).unwrap(),
                max_devices: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                expires_at: row.get::<_, Option<String>>(4)?
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&chrono::Utc)),
                status: serde_json::from_str(&row.get::<_, String>(5)?).unwrap(),
                activated_devices: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                metadata: row.get(7)?,
            }))
        } else {
            Ok(None)
        }
    }
    
    // Scheduled tasks operations
    pub fn load_scheduled_tasks(&self) -> SqlResult<Vec<crate::scheduler::ScheduledTask>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, task_type, schedule, next_run, last_run, status, data, enabled 
             FROM scheduled_tasks ORDER BY next_run"
        )?;
        
        let tasks = stmt.query_map([], |row| {
            Ok(crate::scheduler::ScheduledTask {
                id: row.get(0)?,
                name: row.get(1)?,
                task_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap(),
                schedule: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
                next_run: row.get::<_, Option<String>>(4)?
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&chrono::Utc)),
                last_run: row.get::<_, Option<String>>(5)?
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&chrono::Utc)),
                status: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or(crate::scheduler::TaskStatus::Pending),
                data: row.get(7)?,
                enabled: row.get(8)?,
            })
        })?;
        
        tasks.collect()
    }
    
    pub fn save_scheduled_task(&self, task: &crate::scheduler::ScheduledTask) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO scheduled_tasks 
             (id, name, task_type, schedule, next_run, last_run, status, data, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                task.id,
                task.name,
                serde_json::to_string(&task.task_type).unwrap(),
                serde_json::to_string(&task.schedule).unwrap(),
                task.next_run.map(|d| d.to_rfc3339()),
                task.last_run.map(|d| d.to_rfc3339()),
                serde_json::to_string(&task.status).unwrap(),
                task.data,
                task.enabled,
            ],
        )?;
        Ok(())
    }
}
