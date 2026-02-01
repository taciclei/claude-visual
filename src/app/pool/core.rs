//! Core object pooling implementation

use super::types::{PoolStats, PoolStatsInner};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Generic object pool for reusable items
pub struct ObjectPool<T> {
    /// Available items
    pub(crate) items: Mutex<VecDeque<T>>,
    /// Maximum pool size
    pub(crate) max_size: usize,
    /// Factory function for creating new items
    pub(crate) factory: Box<dyn Fn() -> T + Send + Sync>,
    /// Reset function called when returning items to pool
    pub(crate) reset: Option<Box<dyn Fn(&mut T) + Send + Sync>>,
    /// Statistics
    pub(crate) stats: PoolStatsInner,
}

impl<T> ObjectPool<T> {
    /// Create a new pool with a factory function
    pub fn new(max_size: usize, factory: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self {
            items: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
            factory: Box::new(factory),
            reset: None,
            stats: PoolStatsInner::default(),
        }
    }

    /// Create a pool with a reset function
    pub fn with_reset(
        max_size: usize,
        factory: impl Fn() -> T + Send + Sync + 'static,
        reset: impl Fn(&mut T) + Send + Sync + 'static,
    ) -> Self {
        Self {
            items: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
            factory: Box::new(factory),
            reset: Some(Box::new(reset)),
            stats: PoolStatsInner::default(),
        }
    }

    /// Get an item from the pool or create a new one
    pub fn get(&self) -> T {
        let mut items = self.items.lock();

        if let Some(item) = items.pop_front() {
            self.stats.reused.fetch_add(1, Ordering::Relaxed);
            item
        } else {
            self.stats.misses.fetch_add(1, Ordering::Relaxed);
            self.stats.created.fetch_add(1, Ordering::Relaxed);
            (self.factory)()
        }
    }

    /// Return an item to the pool
    pub fn put(&self, mut item: T) {
        let mut items = self.items.lock();

        if items.len() < self.max_size {
            // Reset the item before returning to pool
            if let Some(ref reset) = self.reset {
                reset(&mut item);
            }
            items.push_back(item);
        }
        // Otherwise, drop the item (pool is full)
    }

    /// Get current pool statistics
    pub fn stats(&self) -> PoolStats {
        let items = self.items.lock();
        let available = items.len();
        self.stats.to_stats(available)
    }

    /// Pre-warm the pool with items
    pub fn prewarm(&self, count: usize) {
        let mut items = self.items.lock();
        let to_create = count.min(self.max_size - items.len());

        for _ in 0..to_create {
            self.stats.created.fetch_add(1, Ordering::Relaxed);
            items.push_back((self.factory)());
        }
    }

    /// Clear the pool
    pub fn clear(&self) {
        let mut items = self.items.lock();
        items.clear();
    }

    /// Get available count
    pub fn available(&self) -> usize {
        self.items.lock().len()
    }
}

/// Thread-safe pool wrapper with Arc
pub type SharedPool<T> = Arc<ObjectPool<T>>;

/// Create a shared pool
pub fn shared_pool<T>(
    max_size: usize,
    factory: impl Fn() -> T + Send + Sync + 'static,
) -> SharedPool<T> {
    Arc::new(ObjectPool::new(max_size, factory))
}

/// Create a shared pool with reset
pub fn shared_pool_with_reset<T>(
    max_size: usize,
    factory: impl Fn() -> T + Send + Sync + 'static,
    reset: impl Fn(&mut T) + Send + Sync + 'static,
) -> SharedPool<T> {
    Arc::new(ObjectPool::with_reset(max_size, factory, reset))
}
