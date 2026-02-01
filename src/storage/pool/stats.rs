//! Pool and connection statistics

use std::time::Duration;

/// Connection pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total connections created
    pub connections_created: usize,
    /// Total connections closed
    pub connections_closed: usize,
    /// Current active connections (checked out)
    pub active_connections: usize,
    /// Current idle connections (available)
    pub idle_connections: usize,
    /// Total number of acquire requests
    pub acquire_requests: usize,
    /// Number of times a new connection was created because pool was empty
    pub pool_misses: usize,
    /// Number of times a connection was reused from pool
    pub pool_hits: usize,
    /// Number of acquire timeouts
    pub timeouts: usize,
    /// Number of health check failures
    pub health_check_failures: usize,
}

impl PoolStats {
    /// Get hit rate percentage
    pub fn hit_rate(&self) -> f32 {
        let total = self.pool_hits + self.pool_misses;
        if total == 0 {
            0.0
        } else {
            (self.pool_hits as f32 / total as f32) * 100.0
        }
    }

    /// Get total connections in pool
    pub fn total_connections(&self) -> usize {
        self.active_connections + self.idle_connections
    }
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    /// How long this connection has existed
    pub age: Duration,
    /// How long since last use before current checkout
    pub idle_time: Duration,
    /// Number of times this connection has been used
    pub use_count: usize,
}
