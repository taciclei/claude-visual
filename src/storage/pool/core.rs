//! Core pool implementation

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::Result;
use rusqlite::Connection;

use super::config::PoolConfig;
use super::connection::PooledConnection;
use super::guard::PooledConnectionGuard;
use super::stats::PoolStats;

/// Inner pool state
pub(crate) struct PoolInner {
    /// Available connections
    pub(crate) connections: VecDeque<PooledConnection>,
    /// Number of active (checked out) connections
    pub(crate) active: usize,
    /// Pool statistics
    pub(crate) stats: PoolStats,
    /// Whether the pool is closed
    pub(crate) closed: bool,
}

/// Database connection pool
pub struct DatabasePool {
    /// Pool configuration
    pub(crate) config: PoolConfig,
    /// Database path
    pub(crate) db_path: PathBuf,
    /// Inner pool state (protected by mutex)
    pub(crate) inner: Mutex<PoolInner>,
    /// Total connections (atomic for fast reads)
    pub(crate) total_connections: AtomicUsize,
}

impl DatabasePool {
    /// Create a new connection pool
    pub fn new(db_path: PathBuf, config: PoolConfig) -> Result<Self> {
        let pool = Self {
            config,
            db_path,
            inner: Mutex::new(PoolInner {
                connections: VecDeque::new(),
                active: 0,
                stats: PoolStats::default(),
                closed: false,
            }),
            total_connections: AtomicUsize::new(0),
        };

        // Initialize minimum connections
        pool.initialize()?;

        Ok(pool)
    }

    /// Initialize the pool with minimum connections
    fn initialize(&self) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();

        for _ in 0..self.config.min_connections {
            let conn = self.create_connection()?;
            inner.connections.push_back(PooledConnection::new(conn));
            inner.stats.connections_created += 1;
            self.total_connections.fetch_add(1, Ordering::SeqCst);
        }

        inner.stats.idle_connections = inner.connections.len();
        Ok(())
    }

    /// Create a new database connection
    pub(crate) fn create_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;

        // Configure connection
        conn.busy_timeout(Duration::from_millis(self.config.busy_timeout_ms as u64))?;

        if self.config.enable_wal {
            conn.execute_batch("PRAGMA journal_mode=WAL")?;
        }

        // Additional performance settings
        conn.execute_batch(
            r#"
            PRAGMA synchronous=NORMAL;
            PRAGMA cache_size=10000;
            PRAGMA temp_store=MEMORY;
            "#,
        )?;

        Ok(conn)
    }

    /// Acquire a connection from the pool
    pub fn acquire(&self) -> Result<PooledConnectionGuard<'_>> {
        let start = Instant::now();

        loop {
            // Check if we can get a connection
            {
                let mut inner = self.inner.lock().unwrap();

                if inner.closed {
                    return Err(anyhow::anyhow!("Pool is closed"));
                }

                inner.stats.acquire_requests += 1;

                // Try to get an existing connection
                while let Some(mut pooled) = inner.connections.pop_front() {
                    // Check if connection is healthy
                    if pooled.is_healthy() {
                        pooled.last_used = Instant::now();
                        pooled.use_count += 1;
                        inner.active += 1;
                        inner.stats.pool_hits += 1;
                        inner.stats.idle_connections = inner.connections.len();
                        inner.stats.active_connections = inner.active;

                        return Ok(PooledConnectionGuard {
                            pool: self,
                            conn: Some(pooled),
                        });
                    } else {
                        // Connection is unhealthy, close it
                        inner.stats.health_check_failures += 1;
                        inner.stats.connections_closed += 1;
                        self.total_connections.fetch_sub(1, Ordering::SeqCst);
                    }
                }

                // No available connections, try to create one
                let total = self.total_connections.load(Ordering::SeqCst);
                if total < self.config.max_connections {
                    match self.create_connection() {
                        Ok(conn) => {
                            let mut pooled = PooledConnection::new(conn);
                            pooled.use_count = 1;
                            inner.active += 1;
                            inner.stats.pool_misses += 1;
                            inner.stats.connections_created += 1;
                            inner.stats.active_connections = inner.active;
                            self.total_connections.fetch_add(1, Ordering::SeqCst);

                            return Ok(PooledConnectionGuard {
                                pool: self,
                                conn: Some(pooled),
                            });
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }

            // Check timeout
            if start.elapsed() >= self.config.acquire_timeout {
                let mut inner = self.inner.lock().unwrap();
                inner.stats.timeouts += 1;
                return Err(anyhow::anyhow!("Connection acquire timeout"));
            }

            // Wait a bit before retrying
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Return a connection to the pool
    pub(crate) fn release(&self, mut pooled: PooledConnection) {
        let mut inner = self.inner.lock().unwrap();

        if inner.closed {
            // Pool is closed, don't return the connection
            inner.stats.connections_closed += 1;
            self.total_connections.fetch_sub(1, Ordering::SeqCst);
            return;
        }

        inner.active -= 1;
        pooled.last_used = Instant::now();

        // Check if we should keep this connection
        let total = inner.connections.len() + inner.active;
        if total < self.config.max_connections {
            inner.connections.push_back(pooled);
        } else {
            inner.stats.connections_closed += 1;
            self.total_connections.fetch_sub(1, Ordering::SeqCst);
        }

        inner.stats.idle_connections = inner.connections.len();
        inner.stats.active_connections = inner.active;
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let inner = self.inner.lock().unwrap();
        inner.stats.clone()
    }

    /// Get current pool size
    pub fn size(&self) -> usize {
        self.total_connections.load(Ordering::SeqCst)
    }

    /// Get number of idle connections
    pub fn idle(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.connections.len()
    }

    /// Get number of active connections
    pub fn active(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.active
    }

    /// Clean up idle connections
    pub fn cleanup(&self) {
        let mut inner = self.inner.lock().unwrap();

        // Remove idle connections that have exceeded idle timeout
        let mut i = 0;
        while i < inner.connections.len() {
            if inner.connections[i].idle_time() > self.config.idle_timeout {
                // Keep at least min_connections
                let total = inner.connections.len() + inner.active;
                if total > self.config.min_connections {
                    inner.connections.remove(i);
                    inner.stats.connections_closed += 1;
                    self.total_connections.fetch_sub(1, Ordering::SeqCst);
                    continue;
                }
            }
            i += 1;
        }

        inner.stats.idle_connections = inner.connections.len();
    }

    /// Close the pool and all connections
    pub fn close(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.closed = true;

        // Close all idle connections
        while inner.connections.pop_front().is_some() {
            inner.stats.connections_closed += 1;
            self.total_connections.fetch_sub(1, Ordering::SeqCst);
        }

        inner.stats.idle_connections = 0;
    }

    /// Check if the pool is closed
    pub fn is_closed(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.closed
    }
}
