//! Pool tests

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::storage::pool::{DatabasePool, PoolConfig};

    fn temp_db_path() -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        std::env::temp_dir().join(format!("claude_visual_test_{}.db", n))
    }

    #[test]
    fn test_pool_creation() {
        let path = temp_db_path();
        let config = PoolConfig {
            min_connections: 2,
            ..Default::default()
        };
        let pool = DatabasePool::new(path.clone(), config).unwrap();

        assert_eq!(pool.size(), 2);
        assert_eq!(pool.idle(), 2);
        assert_eq!(pool.active(), 0);

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_acquire_release() {
        let path = temp_db_path();
        let pool = DatabasePool::new(path.clone(), PoolConfig::default()).unwrap();

        {
            let conn = pool.acquire().unwrap();
            assert_eq!(pool.active(), 1);
            assert!(conn.connection().execute_batch("SELECT 1").is_ok());
        }

        // Connection should be returned to pool
        assert_eq!(pool.active(), 0);
        assert!(pool.idle() >= 1);

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_pool_stats() {
        let path = temp_db_path();
        let pool = DatabasePool::new(path.clone(), PoolConfig::default()).unwrap();

        // Acquire and release a few times
        for _ in 0..5 {
            let _conn = pool.acquire().unwrap();
        }

        let stats = pool.stats();
        assert_eq!(stats.acquire_requests, 5);
        assert!(stats.pool_hits > 0 || stats.pool_misses > 0);

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_pool_close() {
        let path = temp_db_path();
        let pool = DatabasePool::new(path.clone(), PoolConfig::default()).unwrap();

        pool.close();
        assert!(pool.is_closed());
        assert!(pool.acquire().is_err());

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_config_presets() {
        let high_concurrency = PoolConfig::high_concurrency();
        assert!(high_concurrency.max_connections > PoolConfig::default().max_connections);

        let low_memory = PoolConfig::low_memory();
        assert!(low_memory.max_connections < PoolConfig::default().max_connections);
    }
}
