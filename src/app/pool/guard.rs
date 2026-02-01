//! Pooled item guard with auto-return on drop

use super::core::ObjectPool;
use std::sync::Arc;

/// Pooled item guard that returns to pool on drop
pub struct PooledItem<T> {
    item: Option<T>,
    pool: Arc<ObjectPool<T>>,
}

impl<T> PooledItem<T> {
    /// Create from pool
    pub fn from_pool(pool: Arc<ObjectPool<T>>) -> Self {
        Self {
            item: Some(pool.get()),
            pool,
        }
    }

    /// Get reference to inner item
    pub fn get(&self) -> &T {
        self.item.as_ref().unwrap()
    }

    /// Get mutable reference to inner item
    pub fn get_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }

    /// Take ownership (prevents return to pool)
    pub fn take(mut self) -> T {
        self.item.take().unwrap()
    }
}

impl<T> Drop for PooledItem<T> {
    fn drop(&mut self) {
        if let Some(item) = self.item.take() {
            self.pool.put(item);
        }
    }
}

impl<T> std::ops::Deref for PooledItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> std::ops::DerefMut for PooledItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
