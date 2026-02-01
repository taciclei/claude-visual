//! Connection guard for RAII management

use rusqlite::Connection;

use super::connection::PooledConnection;
use super::core::DatabasePool;
use super::stats::ConnectionStats;

/// RAII guard for a pooled connection
pub struct PooledConnectionGuard<'a> {
    pub(crate) pool: &'a DatabasePool,
    pub(crate) conn: Option<PooledConnection>,
}

impl<'a> PooledConnectionGuard<'a> {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        &self.conn.as_ref().unwrap().conn
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn.as_mut().unwrap().conn
    }

    /// Get connection statistics
    pub fn stats(&self) -> ConnectionStats {
        let pooled = self.conn.as_ref().unwrap();
        ConnectionStats {
            age: pooled.age(),
            idle_time: pooled.idle_time(),
            use_count: pooled.use_count,
        }
    }
}

impl Drop for PooledConnectionGuard<'_> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            self.pool.release(conn);
        }
    }
}

impl std::ops::Deref for PooledConnectionGuard<'_> {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection()
    }
}
