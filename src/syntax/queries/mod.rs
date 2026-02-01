//! Precompiled Tree-sitter Queries
//!
//! Optimizes query loading and caching for syntax highlighting.
//! Queries are lazily compiled on first use and cached for reuse.

mod cache;
mod config;
mod types;

// Re-export public types
pub use cache::{QueryCache, QueryCacheStats};
pub use config::LanguageConfig;
pub use types::{CompiledQuery, PrewarmResult, QueryError};

use std::sync::{Arc, OnceLock};

/// Global query cache instance
static QUERY_CACHE: OnceLock<QueryCache> = OnceLock::new();

/// Get or initialize the global query cache
pub fn query_cache() -> &'static QueryCache {
    QUERY_CACHE.get_or_init(QueryCache::new)
}

/// Prewarm the global query cache
pub fn prewarm_queries() -> Result<PrewarmResult, QueryError> {
    query_cache().prewarm()
}

/// Get a compiled query from the global cache
pub fn get_query(language: &str) -> Result<Arc<CompiledQuery>, QueryError> {
    query_cache().get(language)
}

/// Check if a language is supported
pub fn is_language_supported(language: &str) -> bool {
    query_cache().is_supported(language)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_cache_creation() {
        let cache = QueryCache::new();
        assert!(cache.is_supported("rust"));
        assert!(cache.is_supported("rs"));
        assert!(cache.is_supported("javascript"));
        assert!(cache.is_supported("js"));
        assert!(!cache.is_supported("unknown"));
    }

    #[test]
    fn test_query_compilation() {
        let cache = QueryCache::new();
        let query = cache.get("rust").unwrap();
        assert_eq!(query.language, "rust");
        assert!(query.capture_count > 0);
    }

    #[test]
    fn test_query_caching() {
        let cache = QueryCache::new();

        // First access - miss
        let _ = cache.get("rust").unwrap();

        // Second access - hit
        let _ = cache.get("rust").unwrap();

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_alias_resolution() {
        let cache = QueryCache::new();

        let rust1 = cache.get("rust").unwrap();
        let rust2 = cache.get("rs").unwrap();

        // Both should return the same compiled query
        assert_eq!(rust1.language, rust2.language);
    }

    #[test]
    fn test_unsupported_language() {
        let cache = QueryCache::new();
        let result = cache.get("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_supported_languages() {
        let cache = QueryCache::new();
        let languages = cache.supported_languages();

        assert!(languages.contains(&"rust"));
        assert!(languages.contains(&"javascript"));
        assert!(languages.contains(&"python"));
    }
}
