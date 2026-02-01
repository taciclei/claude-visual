//! Summarization request types

use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::config::SummarizationConfig;
use super::message::{ConversationMessage, MessageRole};

/// Statistics about summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizationStats {
    /// Total messages in current session
    pub total_messages: usize,
    /// Messages that have been summarized
    pub summarized_messages: usize,
    /// Total tokens (summaries + messages)
    pub total_tokens: usize,
    /// Tokens used by summaries
    pub summary_tokens: usize,
    /// Tokens used by messages
    pub message_tokens: usize,
    /// Overall compression ratio
    pub compression_ratio: f32,
    /// Number of summaries created
    pub summary_count: usize,
}

/// Summarization request for async processing
#[derive(Debug, Clone)]
pub struct SummarizationRequest {
    /// Messages to summarize
    pub messages: Vec<ConversationMessage>,
    /// Configuration
    pub config: SummarizationConfig,
    /// Request ID
    pub request_id: String,
    /// When the request was created
    pub created_at: Instant,
}

impl SummarizationRequest {
    /// Create a new request
    pub fn new(messages: Vec<ConversationMessage>, config: SummarizationConfig) -> Self {
        Self {
            messages,
            config,
            request_id: uuid::Uuid::new_v4().to_string(),
            created_at: Instant::now(),
        }
    }

    /// Get the summary prompt
    pub fn prompt(&self) -> String {
        let mut prompt = String::from(
            "Please summarize the following conversation, focusing on:\n\
             1. Key topics and decisions made\n\
             2. Important technical details and code changes\n\
             3. Current state and any pending tasks\n\
             4. Context needed for continuing the conversation\n\n\
             Keep the summary concise but comprehensive. "
        );

        if !self.config.include_code {
            prompt.push_str("Summarize code changes conceptually without including full code blocks. ");
        }

        prompt.push_str(&format!(
            "Keep the summary under {} tokens.\n\n",
            self.config.max_summary_tokens
        ));

        prompt.push_str("Conversation to summarize:\n\n");

        for msg in &self.messages {
            let role = match msg.role {
                MessageRole::User => "User",
                MessageRole::Assistant => "Assistant",
                MessageRole::System => "System",
            };

            prompt.push_str(&format!("**{}**: {}\n\n", role, msg.content));
        }

        prompt
    }
}
