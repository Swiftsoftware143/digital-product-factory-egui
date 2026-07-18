//! Database extension — Analytics and Publishing tables

use rusqlite::{Connection, params, Result as SqlResult};

/// Initialize analytics + publishing tables (standalone function)
pub fn init_business_tables(conn: &Connection) -> SqlResult<()> {
    // Sales records
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sales_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            platform TEXT NOT NULL,
            units_sold INTEGER NOT NULL DEFAULT 1,
            revenue REAL NOT NULL,
            fee REAL NOT NULL DEFAULT 0.0,
            net_revenue REAL NOT NULL,
            sale_date TEXT NOT NULL,
            recorded_at TEXT NOT NULL,
            notes TEXT DEFAULT ''
        )",
        [],
    )?;

    // Publish logs
    conn.execute(
        "CREATE TABLE IF NOT EXISTS publish_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            platform TEXT NOT NULL,
            listing_url TEXT,
            listing_id TEXT,
            status TEXT NOT NULL DEFAULT 'published',
            error_message TEXT,
            published_at TEXT NOT NULL
        )",
        [],
    )?;

    // Indexes for fast queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sales_product ON sales_records(product_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sales_platform ON sales_records(platform)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_publish_product ON publish_logs(product_id)",
        [],
    )?;

    Ok(())
}
