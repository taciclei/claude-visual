//! Syntax highlighting module using tree-sitter

pub mod highlighter;
pub mod queries;

pub use highlighter::{Highlighter, HighlightedSpan, SyntaxHighlighter};
pub use queries::{
    query_cache, prewarm_queries, get_query, is_language_supported,
    CompiledQuery, QueryCache, QueryCacheStats, QueryError, PrewarmResult,
};
