//! Task scheduler for automation

use crate::database::Database;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: usize,
    pub name: String,
    pub task_type: TaskType,
    pub schedule: Schedule,
    pub next_run: Option<DateTime<Utc>>,
    pub last_run: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    GenerateProduct,
    PublishProduct,
    ResearchMarket,
    CreateBundle,
    PinterestPin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Schedule {
    Once(DateTime<Utc>),
    Daily { hour: u32, minute: u32 },
    Weekly { day: u32, hour: u32, minute: u32 },
    Interval { minutes: u64 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Paused,
}

pub struct Scheduler {
    db: Arc<Database>,
    runtime: Arc<Runtime>,
    tasks: Vec<ScheduledTask>,
}

impl Scheduler {
    pub fn new(db: &Arc<Database>, runtime: Arc<Runtime>) -> Self {
        let tasks = db.load_scheduled_tasks().unwrap_or_default();
        
        Self {
            db: db.clone(),
            runtime,
            tasks,
        }
    }
    
    pub fn schedule(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }
    
    pub fn tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }
}
