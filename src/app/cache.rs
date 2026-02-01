//! LRU Cache for Memory Management
//!
//! Provides efficient caching with automatic eviction of least recently used items.
//! Used for image caching, syntax highlighting cache, and other memory-intensive data.

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// Entry in the LRU cache
struct CacheEntry<V> {
    value: V,
    last_access: Instant,
    size_bytes: usize,
}

/// LRU (Least Recently Used) cache with size limits
pub struct LruCache<K: Hash + Eq + Clone, V> {
    /// Cached entries
    entries: HashMap<K, CacheEntry<V>>,
    /// Maximum number of entries
    max_entries: usize,
    /// Maximum total size in bytes
    max_size_bytes: usize,
    /// Current total size in bytes
    current_size_bytes: usize,
    /// Time-to-live for entries (optional)
    ttl: Option<Duration>,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    /// Create a new LRU cache with entry limit
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            max_size_bytes: usize::MAX,
            current_size_bytes: 0,
            ttl: None,
        }
    }

    /// Create a new LRU cache with size limit
    pub fn with_size_limit(max_size_bytes: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries: usize::MAX,
            max_size_bytes,
            current_size_bytes: 0,
            ttl: None,
        }
    }

    /// Set time-to-live for entries
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Get a value from the cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // Check TTL if configured
        if let Some(ttl) = self.ttl {
            if let Some(entry) = self.entries.get(key) {
                if entry.last_access.elapsed() > ttl {
                    self.remove(key);
                    return None;
                }
            }
        }

        // Update last access time and return value
        if let Some(entry) = self.entries.get_mut(key) {
            entry.last_access = Instant::now();
            Some(&entry.value)
        } else {
            None
        }
    }

    /// Insert a value into the cache
    pub fn insert(&mut self, key: K, value: V, size_bytes: usize) {
        // Remove existing entry if present
        if self.entries.contains_key(&key) {
            self.remove(&key);
        }

        // Evict entries if necessary
        while self.entries.len() >= self.max_entries
            || self.current_size_bytes + size_bytes > self.max_size_bytes
        {
            if !self.evict_oldest() {
                break;
            }
        }

        // Insert new entry
        self.entries.insert(
            key,
            CacheEntry {
                value,
                last_access: Instant::now(),
                size_bytes,
            },
        );
        self.current_size_bytes += size_bytes;
    }

    /// Insert a value with default size estimate
    pub fn insert_default(&mut self, key: K, value: V) {
        // Estimate 1KB per entry by default
        self.insert(key, value, 1024);
    }

    /// Remove an entry from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.entries.remove(key) {
            self.current_size_bytes = self.current_size_bytes.saturating_sub(entry.size_bytes);
            Some(entry.value)
        } else {
            None
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_size_bytes = 0;
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the current size in bytes
    pub fn size_bytes(&self) -> usize {
        self.current_size_bytes
    }

    /// Evict the oldest entry
    fn evict_oldest(&mut self) -> bool {
        let oldest_key = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_access)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            self.remove(&key);
            true
        } else {
            false
        }
    }

    /// Evict expired entries (if TTL is set)
    pub fn evict_expired(&mut self) {
        if let Some(ttl) = self.ttl {
            let expired: Vec<K> = self
                .entries
                .iter()
                .filter(|(_, entry)| entry.last_access.elapsed() > ttl)
                .map(|(key, _)| key.clone())
                .collect();

            for key in expired {
                self.remove(&key);
            }
        }
    }
}

impl<K: Hash + Eq + Clone, V> Default for LruCache<K, V> {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Syntax highlighting cache
pub type SyntaxCache = LruCache<String, Vec<(std::ops::Range<usize>, String)>>;

/// Create a new syntax cache with reasonable defaults
pub fn create_syntax_cache() -> SyntaxCache {
    LruCache::with_size_limit(10 * 1024 * 1024) // 10MB limit
        .with_ttl(Duration::from_secs(300)) // 5 minute TTL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_eviction() {
        let mut cache: LruCache<i32, String> = LruCache::new(3);

        cache.insert_default(1, "one".to_string());
        cache.insert_default(2, "two".to_string());
        cache.insert_default(3, "three".to_string());
        assert_eq!(cache.len(), 3);

        // Access 1 to make it recent
        cache.get(&1);

        // Insert 4, should evict 2 (oldest)
        cache.insert_default(4, "four".to_string());
        assert_eq!(cache.len(), 3);
        assert!(cache.get(&2).is_none());
        assert!(cache.get(&1).is_some());
    }

    #[test]
    fn test_size_limit() {
        let mut cache: LruCache<i32, Vec<u8>> = LruCache::with_size_limit(100);

        cache.insert(1, vec![0; 40], 40);
        cache.insert(2, vec![0; 40], 40);
        assert_eq!(cache.size_bytes(), 80);

        // This should evict entry 1
        cache.insert(3, vec![0; 40], 40);
        assert!(cache.size_bytes() <= 100);
    }
}
