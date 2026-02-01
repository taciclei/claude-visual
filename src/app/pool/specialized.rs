//! Specialized pool implementations

use super::core::ObjectPool;
use super::types::PoolStats;

/// String buffer pool for text processing
pub struct StringPool {
    pool: ObjectPool<String>,
    default_capacity: usize,
}

impl StringPool {
    /// Create a new string pool
    pub fn new(max_size: usize, default_capacity: usize) -> Self {
        Self {
            pool: ObjectPool::with_reset(
                max_size,
                move || String::with_capacity(default_capacity),
                |s| s.clear(),
            ),
            default_capacity,
        }
    }

    /// Get a string buffer
    pub fn get(&self) -> String {
        self.pool.get()
    }

    /// Return a string buffer
    pub fn put(&self, s: String) {
        // Only return if capacity is reasonable (avoid holding huge strings)
        if s.capacity() <= self.default_capacity * 4 {
            self.pool.put(s);
        }
    }

    /// Get stats
    pub fn stats(&self) -> PoolStats {
        self.pool.stats()
    }
}

/// Vec buffer pool for list processing
pub struct VecPool<T> {
    pool: ObjectPool<Vec<T>>,
    default_capacity: usize,
}

impl<T: Default> VecPool<T> {
    /// Create a new vec pool
    pub fn new(max_size: usize, default_capacity: usize) -> Self {
        Self {
            pool: ObjectPool::with_reset(
                max_size,
                move || Vec::with_capacity(default_capacity),
                |v| v.clear(),
            ),
            default_capacity,
        }
    }

    /// Get a vec buffer
    pub fn get(&self) -> Vec<T> {
        self.pool.get()
    }

    /// Return a vec buffer
    pub fn put(&self, v: Vec<T>) {
        // Only return if capacity is reasonable
        if v.capacity() <= self.default_capacity * 4 {
            self.pool.put(v);
        }
    }

    /// Get stats
    pub fn stats(&self) -> PoolStats {
        self.pool.stats()
    }
}
