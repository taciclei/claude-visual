//! Database Connection Pooling
//!
//! Provides connection pooling for SQLite to improve performance
//! and reduce connection overhead in concurrent scenarios.

mod config;
mod connection;
mod core;
mod guard;
mod shared;
mod stats;
#[cfg(test)]
mod tests;

// Public re-exports
pub use config::PoolConfig;
pub use core::DatabasePool;
pub use guard::PooledConnectionGuard;
pub use shared::{create_shared_pool, SharedPool};
pub use stats::{ConnectionStats, PoolStats};
