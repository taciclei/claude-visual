//! Configuration for conversation summarization

use serde::{Deserialize, Serialize};

/// Configuration for auto-summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizationConfig {
    /// Token threshold to trigger summarization (e.g., 80% of context window)
    pub token_threshold: usize,
    /// Minimum number of messages before summarization is considered
    pub min_messages: usize,
    /// Number of recent messages to preserve (not summarized)
    pub preserve_recent: usize,
    /// Whether to include code blocks in summary
    pub include_code: bool,
    /// Maximum summary length in tokens
    pub max_summary_tokens: usize,
}

impl Default for SummarizationConfig {
    fn default() -> Self {
        Self {
            token_threshold: 80_000, // 80k tokens (80% of 100k)
            min_messages: 10,
            preserve_recent: 4, // Keep last 4 messages
            include_code: true,
            max_summary_tokens: 2000,
        }
    }
}

impl SummarizationConfig {
    /// Create config for smaller context windows (e.g., GPT-4)
    pub fn for_small_context() -> Self {
        Self {
            token_threshold: 6000,
            min_messages: 6,
            preserve_recent: 2,
            include_code: false,
            max_summary_tokens: 1000,
        }
    }

    /// Create config for large context windows (e.g., Claude 200k)
    pub fn for_large_context() -> Self {
        Self {
            token_threshold: 160_000,
            min_messages: 20,
            preserve_recent: 6,
            include_code: true,
            max_summary_tokens: 4000,
        }
    }
}
