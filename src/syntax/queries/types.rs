//! Core query types

use tree_sitter::{Language, Query};

/// Compiled query with metadata
pub struct CompiledQuery {
    /// The tree-sitter query
    pub query: Query,
    /// Language name
    pub language: String,
    /// Number of capture names
    pub capture_count: usize,
    /// Capture names for fast lookup
    pub capture_names: Vec<String>,
}

impl CompiledQuery {
    /// Create a new compiled query
    pub fn new(language: &str, lang: &Language, source: &str) -> Result<Self, QueryError> {
        let query = Query::new(lang, source).map_err(|e| QueryError::CompilationFailed {
            language: language.to_string(),
            message: e.to_string(),
        })?;

        let capture_names: Vec<String> = query.capture_names().iter().map(|s| s.to_string()).collect();
        let capture_count = capture_names.len();

        Ok(Self {
            query,
            language: language.to_string(),
            capture_count,
            capture_names,
        })
    }

    /// Get capture index by name
    pub fn capture_index(&self, name: &str) -> Option<usize> {
        self.capture_names.iter().position(|n| n == name)
    }
}

/// Query compilation error
#[derive(Debug, Clone, thiserror::Error)]
pub enum QueryError {
    #[error("Query compilation failed for {language}: {message}")]
    CompilationFailed { language: String, message: String },
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    #[error("Query not found: {0}")]
    QueryNotFound(String),
}

/// Result of prewarming the cache
#[derive(Debug)]
pub struct PrewarmResult {
    /// Number of languages compiled
    pub languages_compiled: usize,
    /// Errors encountered during compilation
    pub errors: Vec<(String, QueryError)>,
    /// Total duration in milliseconds
    pub duration_ms: u64,
}

impl PrewarmResult {
    /// Check if all languages were compiled successfully
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}
