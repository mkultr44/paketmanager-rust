
use anyhow::{Context, Result};
use chrono::{Local};
use hermes_core::{Package, PackageStatus, Zone, normalize_tracking};
use parking_lot::Mutex;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .with_context(|| format!("opening db at {}", path.as_ref().display()))?;
        Self::setup(&conn)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    fn setup(conn: &Connection) -> Result<()> {
        conn.pragma_update(None, "journal_mode", &"WAL")?;
        conn.pragma_update(None, "synchronous", &"NORMAL")?;
        conn.pragma_update(None, "busy_timeout", &"5000")?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS packages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tracking TEXT NOT NULL UNIQUE,
                zone TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_packages_zone ON packages(zone);
            CREATE INDEX IF NOT EXISTS idx_packages_created ON packages(created_at);
        "#)?;
        Ok(())
    }

    pub fn add_in(&self, tracking_raw: &str, zone: Zone) -> Result<Package> {
        let tracking = normalize_tracking(tracking_raw);
        let now = Local::now();
        let conn = self.conn.lock();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO packages (tracking, zone, status, created_at) VALUES (?1, ?2, 'in', ?3)",
            params![tracking, zone.to_string(), now.to_rfc3339()],
        )?;
        tx.commit()?;
        Ok(Package { id: self.find_id(&tracking)?, tracking, zone, status: PackageStatus::In, created_at: now })
    }

    pub fn mark_out(&self, tracking_raw: &str) -> Result<()> {
        let tracking = normalize_tracking(tracking_raw);
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE packages SET status='out' WHERE tracking=?1",
            params![tracking],
        )?;
        Ok(())
    }

    pub fn find_id(&self, tracking: &str) -> Result<i64> {
        let conn = self.conn.lock();
        let id: Option<i64> = conn
            .query_row("SELECT id FROM packages WHERE tracking=?1", params![tracking], |r| r.get(0))
            .optional()?;
        Ok(id.unwrap_or_default())
    }

    pub fn count_in_zone(&self, zone: Zone) -> Result<u64> {
        let conn = self.conn.lock();
        let n: u64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE zone=?1 AND status='in'",
            params![zone.to_string()],
            |r| r.get::<_, u64>(0),
        )?;
        Ok(n)
    }

    pub fn total_in(&self) -> Result<u64> {
        let conn = self.conn.lock();
        let n: u64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE status='in'",
            [],
            |r| r.get::<_, u64>(0),
        )?;
        Ok(n)
    }
}
