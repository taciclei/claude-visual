//! Conversation summary types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ai::summarizer::utils::estimate_tokens;

/// Summary of a conversation segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// Summary ID
    pub id: String,
    /// The summarized text
    pub content: String,
    /// Number of messages that were summarized
    pub message_count: usize,
    /// Token count of original messages
    pub original_tokens: usize,
    /// Token count of the summary
    pub summary_tokens: usize,
    /// Time range covered
    pub time_range: (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>),
    /// Key topics discussed
    pub topics: Vec<String>,
    /// Code-related if any
    pub code_context: Option<String>,
    /// When the summary was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ConversationSummary {
    /// Create a new summary
    pub fn new(
        content: impl Into<String>,
        message_count: usize,
        original_tokens: usize,
        time_range: (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>),
    ) -> Self {
        let content = content.into();
        let summary_tokens = estimate_tokens(&content);

        Self {
            id: Uuid::new_v4().to_string(),
            content,
            message_count,
            original_tokens,
            summary_tokens,
            time_range,
            topics: Vec::new(),
            code_context: None,
            created_at: chrono::Utc::now(),
        }
    }

    /// Add topics
    pub fn with_topics(mut self, topics: Vec<String>) -> Self {
        self.topics = topics;
        self
    }

    /// Add code context
    pub fn with_code_context(mut self, context: impl Into<String>) -> Self {
        self.code_context = Some(context.into());
        self
    }

    /// Format as context for AI prompt
    pub fn format_for_prompt(&self) -> String {
        let mut output = String::new();

        output.push_str("=== CONVERSATION SUMMARY ===\n");
        output.push_str(&format!(
            "(Summarizing {} messages, {} → {} tokens)\n\n",
            self.message_count,
            self.original_tokens,
            self.summary_tokens
        ));

        if !self.topics.is_empty() {
            output.push_str(&format!("Topics discussed: {}\n\n", self.topics.join(", ")));
        }

        output.push_str(&self.content);

        if let Some(ref code_ctx) = self.code_context {
            output.push_str("\n\nCode context:\n");
            output.push_str(code_ctx);
        }

        output.push_str("\n=== END SUMMARY ===\n\n");
        output
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f32 {
        if self.original_tokens == 0 {
            return 0.0;
        }
        1.0 - (self.summary_tokens as f32 / self.original_tokens as f32)
    }
}
