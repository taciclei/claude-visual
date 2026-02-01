//! Tree-sitter based syntax highlighter

mod core;
mod global;
mod queries;
mod types;
mod utils;

// Re-export public API
pub use global::SyntaxHighlighter;
pub use types::HighlightedSpan;

// Re-export Highlighter for testing or direct use
pub use core::Highlighter;
