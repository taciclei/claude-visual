//! Syntax highlighting module using tree-sitter

pub mod highlighter;
pub mod queries;

pub use highlighter::{HighlightedSpan, Highlighter, SyntaxHighlighter};
pub use queries::{
    get_query, is_language_supported, prewarm_queries, query_cache, CompiledQuery, PrewarmResult,
    QueryCache, QueryCacheStats, QueryError,
};
