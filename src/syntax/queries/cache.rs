//! Query cache implementation

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::config::language_configs;
use super::types::{CompiledQuery, PrewarmResult, QueryError};
use super::LanguageConfig;

/// Query cache for compiled queries
pub struct QueryCache {
    /// Cached compiled queries by language name
    queries: RwLock<HashMap<String, Arc<CompiledQuery>>>,
    /// Language configurations
    configs: HashMap<String, &'static LanguageConfig>,
    /// Statistics
    stats: RwLock<QueryCacheStats>,
}

/// Query cache statistics
#[derive(Debug, Clone, Default)]
pub struct QueryCacheStats {
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses (new compilations)
    pub misses: usize,
    /// Total compilation time in microseconds
    pub total_compile_time_us: u64,
    /// Languages loaded
    pub languages_loaded: usize,
}

impl QueryCacheStats {
    /// Get hit rate percentage
    pub fn hit_rate(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f32 / total as f32) * 100.0
        }
    }
}

impl QueryCache {
    /// Create a new query cache with all supported languages
    pub fn new() -> Self {
        let mut configs = HashMap::new();

        // Register all languages
        for config in language_configs().iter() {
            configs.insert(config.name.to_string(), config);
            for alias in config.aliases {
                configs.insert(alias.to_string(), config);
            }
        }

        Self {
            queries: RwLock::new(HashMap::new()),
            configs,
            stats: RwLock::new(QueryCacheStats::default()),
        }
    }

    /// Get a compiled query for a language, compiling if needed
    pub fn get(&self, language: &str) -> Result<Arc<CompiledQuery>, QueryError> {
        let lang_lower = language.to_lowercase();

        // Fast path: check cache
        {
            let cache = self.queries.read().unwrap();
            if let Some(query) = cache.get(&lang_lower) {
                let mut stats = self.stats.write().unwrap();
                stats.hits += 1;
                return Ok(Arc::clone(query));
            }
        }

        // Slow path: compile query
        self.compile_and_cache(&lang_lower)
    }

    /// Compile and cache a query
    fn compile_and_cache(&self, language: &str) -> Result<Arc<CompiledQuery>, QueryError> {
        let config = self
            .configs
            .get(language)
            .ok_or_else(|| QueryError::UnsupportedLanguage(language.to_string()))?;

        let start = std::time::Instant::now();
        let compiled = CompiledQuery::new(config.name, &config.language, config.highlights)?;
        let compile_time = start.elapsed().as_micros() as u64;

        let compiled = Arc::new(compiled);

        // Cache the compiled query
        {
            let mut cache = self.queries.write().unwrap();
            cache.insert(language.to_string(), Arc::clone(&compiled));

            // Also cache by primary name if different
            if language != config.name {
                cache.insert(config.name.to_string(), Arc::clone(&compiled));
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.misses += 1;
            stats.total_compile_time_us += compile_time;
            stats.languages_loaded += 1;
        }

        Ok(compiled)
    }

    /// Prewarm cache by compiling all supported languages
    pub fn prewarm(&self) -> Result<PrewarmResult, QueryError> {
        let start = std::time::Instant::now();
        let mut compiled = 0;
        let mut errors = Vec::new();

        for config in language_configs().iter() {
            match self.get(config.name) {
                Ok(_) => compiled += 1,
                Err(e) => errors.push((config.name.to_string(), e)),
            }
        }

        Ok(PrewarmResult {
            languages_compiled: compiled,
            errors,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Get cache statistics
    pub fn stats(&self) -> QueryCacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Check if a language is supported
    pub fn is_supported(&self, language: &str) -> bool {
        self.configs.contains_key(&language.to_lowercase())
    }

    /// Get list of supported languages
    pub fn supported_languages(&self) -> Vec<&str> {
        language_configs().iter().map(|c| c.name).collect()
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.queries.write().unwrap();
        cache.clear();

        let mut stats = self.stats.write().unwrap();
        stats.languages_loaded = 0;
    }
}

impl Default for QueryCache {
    fn default() -> Self {
        Self::new()
    }
}
