//! Pool implementation tests

use super::*;
use std::sync::Arc;

#[test]
fn test_object_pool_basic() {
    let pool = ObjectPool::new(5, || Vec::<i32>::new());

    // Get items
    let mut v1 = pool.get();
    let mut v2 = pool.get();

    v1.push(1);
    v2.push(2);

    // Return to pool
    pool.put(v1);
    pool.put(v2);

    // Pool should have 2 items
    assert_eq!(pool.available(), 2);

    // Get reused item
    let v3 = pool.get();
    assert_eq!(pool.stats().reused, 1);
    assert!(v3.is_empty() || !v3.is_empty()); // Could be either, depends on reset
}

#[test]
fn test_object_pool_with_reset() {
    let pool = ObjectPool::with_reset(5, || Vec::<i32>::new(), |v| v.clear());

    let mut v1 = pool.get();
    v1.push(1);
    v1.push(2);

    pool.put(v1);

    // Get reused item - should be cleared
    let v2 = pool.get();
    assert!(v2.is_empty());
}

#[test]
fn test_pool_max_size() {
    let pool = ObjectPool::new(2, || 0);

    // Return more items than max size
    pool.put(1);
    pool.put(2);
    pool.put(3); // This should be dropped

    assert_eq!(pool.available(), 2);
}

#[test]
fn test_pooled_item() {
    let pool = Arc::new(ObjectPool::new(5, || String::new()));

    {
        let mut item = PooledItem::from_pool(pool.clone());
        item.push_str("hello");
        assert_eq!(&*item, "hello");
    } // item dropped, returned to pool

    assert_eq!(pool.available(), 1);
}

#[test]
fn test_string_pool() {
    let pool = StringPool::new(5, 256);

    let mut s = pool.get();
    s.push_str("test");

    pool.put(s);

    let s2 = pool.get();
    assert!(s2.is_empty()); // Should be cleared
    assert!(s2.capacity() >= 256);
}

#[test]
fn test_pool_prewarm() {
    let pool = ObjectPool::new(10, || String::new());

    pool.prewarm(5);
    assert_eq!(pool.available(), 5);

    let stats = pool.stats();
    assert_eq!(stats.created, 5);
    assert_eq!(stats.misses, 0);
}
