//! Shared pool utilities

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use super::config::PoolConfig;
use super::core::DatabasePool;

/// Thread-safe pool reference
pub type SharedPool = Arc<DatabasePool>;

/// Create a shared pool
pub fn create_shared_pool(db_path: PathBuf, config: PoolConfig) -> Result<SharedPool> {
    Ok(Arc::new(DatabasePool::new(db_path, config)?))
}
