//! Internal pooled connection type

use std::time::{Duration, Instant};

use rusqlite::Connection;

/// A pooled connection with metadata
pub(crate) struct PooledConnection {
    /// The SQLite connection
    pub(crate) conn: Connection,
    /// When this connection was created
    pub(crate) created_at: Instant,
    /// Last time this connection was used
    pub(crate) last_used: Instant,
    /// Number of times this connection has been used
    pub(crate) use_count: usize,
}

impl PooledConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        let now = Instant::now();
        Self {
            conn,
            created_at: now,
            last_used: now,
            use_count: 0,
        }
    }

    pub(crate) fn is_healthy(&self) -> bool {
        // Simple health check - try to execute a query
        self.conn.execute_batch("SELECT 1").is_ok()
    }

    pub(crate) fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub(crate) fn idle_time(&self) -> Duration {
        self.last_used.elapsed()
    }
}
