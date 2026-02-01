//! Core summarizer implementation

use super::types::*;

/// Conversation summarizer
pub struct Summarizer {
    /// Configuration
    pub(super) config: SummarizationConfig,
    /// Previous summaries (oldest first)
    pub(super) summaries: Vec<ConversationSummary>,
    /// Current messages (not yet summarized)
    pub(super) messages: Vec<ConversationMessage>,
    /// Total token count
    pub(super) total_tokens: usize,
}

impl Default for Summarizer {
    fn default() -> Self {
        Self::new(SummarizationConfig::default())
    }
}

impl Summarizer {
    /// Create a new summarizer
    pub fn new(config: SummarizationConfig) -> Self {
        Self {
            config,
            summaries: Vec::new(),
            messages: Vec::new(),
            total_tokens: 0,
        }
    }

    /// Add a message
    pub fn add_message(&mut self, message: ConversationMessage) {
        self.total_tokens += message.token_count;
        self.messages.push(message);
    }

    /// Check if summarization is needed
    pub fn needs_summarization(&self) -> bool {
        self.total_tokens > self.config.token_threshold
            && self.messages.len() >= self.config.min_messages
    }

    /// Get messages that should be summarized (excluding recent ones)
    pub fn messages_to_summarize(&self) -> &[ConversationMessage] {
        if self.messages.len() <= self.config.preserve_recent {
            &[]
        } else {
            let end = self.messages.len() - self.config.preserve_recent;
            &self.messages[..end]
        }
    }

    /// Generate a summary prompt for AI
    pub fn generate_summary_prompt(&self) -> Option<String> {
        let to_summarize = self.messages_to_summarize();
        if to_summarize.is_empty() {
            return None;
        }

        let mut prompt = String::from(
            "Please summarize the following conversation, focusing on:\n\
             1. Key topics and decisions made\n\
             2. Important technical details and code changes\n\
             3. Current state and any pending tasks\n\
             4. Context needed for continuing the conversation\n\n\
             Keep the summary concise but comprehensive.\n\n\
             Conversation to summarize:\n\n"
        );

        for msg in to_summarize {
            let role = match msg.role {
                MessageRole::User => "User",
                MessageRole::Assistant => "Assistant",
                MessageRole::System => "System",
            };

            prompt.push_str(&format!("**{}**: {}\n\n", role, msg.content));
        }

        Some(prompt)
    }

    /// Apply a summary (replace summarized messages)
    pub fn apply_summary(&mut self, summary: ConversationSummary) {
        // Calculate how many messages to remove
        let to_remove = if self.messages.len() > self.config.preserve_recent {
            self.messages.len() - self.config.preserve_recent
        } else {
            0
        };

        if to_remove > 0 {
            // Calculate tokens being removed
            let removed_tokens: usize = self.messages[..to_remove]
                .iter()
                .map(|m| m.token_count)
                .sum();

            // Remove old messages
            self.messages.drain(..to_remove);

            // Update token count
            self.total_tokens = self.total_tokens.saturating_sub(removed_tokens);
            self.total_tokens += summary.summary_tokens;

            // Add summary
            self.summaries.push(summary);
        }
    }

    /// Get all summaries
    pub fn summaries(&self) -> &[ConversationSummary] {
        &self.summaries
    }

    /// Get current messages
    pub fn messages(&self) -> &[ConversationMessage] {
        &self.messages
    }

    /// Get total token count (summaries + messages)
    pub fn total_tokens(&self) -> usize {
        self.total_tokens
    }

    /// Get token count from summaries only
    pub fn summary_tokens(&self) -> usize {
        self.summaries.iter().map(|s| s.summary_tokens).sum()
    }

    /// Get token count from messages only
    pub fn message_tokens(&self) -> usize {
        self.messages.iter().map(|m| m.token_count).sum()
    }

    /// Format all context for AI prompt (summaries + messages)
    pub fn format_for_prompt(&self) -> String {
        let mut output = String::new();

        // Include all summaries
        for summary in &self.summaries {
            output.push_str(&summary.format_for_prompt());
        }

        // Include current messages
        for msg in &self.messages {
            let role = match msg.role {
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::System => "system",
            };
            output.push_str(&format!("[{}]: {}\n\n", role, msg.content));
        }

        output
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.summaries.clear();
        self.messages.clear();
        self.total_tokens = 0;
    }
}
