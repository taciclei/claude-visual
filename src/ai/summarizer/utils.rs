//! Utility functions for summarization

/// Estimate token count for text
pub(crate) fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: ~4 characters per token
    text.len() / 4
}
