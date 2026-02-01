//! Conversation messages for summarization

use serde::{Deserialize, Serialize};

use crate::ai::summarizer::utils::estimate_tokens;

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// A conversation message for summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Message role (user/assistant/system)
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Whether message contains code
    pub has_code: bool,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Estimated token count
    pub token_count: usize,
}

impl ConversationMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        let content = content.into();
        let has_code = content.contains("```");
        let token_count = estimate_tokens(&content);

        Self {
            role: MessageRole::User,
            content,
            has_code,
            timestamp: chrono::Utc::now(),
            token_count,
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        let content = content.into();
        let has_code = content.contains("```");
        let token_count = estimate_tokens(&content);

        Self {
            role: MessageRole::Assistant,
            content,
            has_code,
            timestamp: chrono::Utc::now(),
            token_count,
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        let content = content.into();
        let token_count = estimate_tokens(&content);

        Self {
            role: MessageRole::System,
            content,
            has_code: false,
            timestamp: chrono::Utc::now(),
            token_count,
        }
    }
}
