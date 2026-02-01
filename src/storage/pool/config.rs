//! Pool configuration

use std::time::Duration;

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections to keep open
    pub min_connections: usize,
    /// Maximum number of connections allowed
    pub max_connections: usize,
    /// Maximum time to wait for a connection
    pub acquire_timeout: Duration,
    /// Maximum idle time before a connection is closed
    pub idle_timeout: Duration,
    /// Connection health check interval
    pub health_check_interval: Duration,
    /// Enable WAL mode for better concurrent access
    pub enable_wal: bool,
    /// Busy timeout in milliseconds
    pub busy_timeout_ms: u32,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 10,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            health_check_interval: Duration::from_secs(60),
            enable_wal: true,
            busy_timeout_ms: 5000,
        }
    }
}

impl PoolConfig {
    /// Configuration for high concurrency
    pub fn high_concurrency() -> Self {
        Self {
            min_connections: 2,
            max_connections: 20,
            acquire_timeout: Duration::from_secs(15),
            idle_timeout: Duration::from_secs(120),
            health_check_interval: Duration::from_secs(30),
            enable_wal: true,
            busy_timeout_ms: 10000,
        }
    }

    /// Configuration for low memory usage
    pub fn low_memory() -> Self {
        Self {
            min_connections: 1,
            max_connections: 3,
            acquire_timeout: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(120),
            enable_wal: true,
            busy_timeout_ms: 3000,
        }
    }
}
