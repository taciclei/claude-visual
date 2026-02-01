//! Message statistics methods

use crate::claude::message::MessageRole;

use super::super::core::ChatView;
use super::super::types::ConversationStats;

impl ChatView {
    /// Get conversation statistics
    pub fn get_conversation_stats(&self) -> ConversationStats {
        let message_count = self.messages.len();
        let user_message_count = self.messages.iter().filter(|m| m.role == MessageRole::User).count();
        let assistant_message_count = self.messages.iter().filter(|m| m.role == MessageRole::Assistant).count();
        let tool_use_count = self.messages.iter().filter(|m| m.role == MessageRole::ToolUse).count();

        // Calculate word counts
        let mut word_count = 0;
        let mut user_word_count = 0;
        let mut assistant_word_count = 0;
        let mut char_count = 0;

        for msg in &self.messages {
            let words = msg.content.split_whitespace().count();
            word_count += words;
            char_count += msg.content.len();
            match msg.role {
                MessageRole::User => user_word_count += words,
                MessageRole::Assistant => assistant_word_count += words,
                _ => {}
            }
        }

        let estimated_tokens = (word_count as f64 * 1.3) as usize;
        let pinned_count = self.pinned_count();
        let bookmarked_count = self.bookmarked_messages.len();
        let tags_count = self.conversation_tags.len();

        ConversationStats {
            message_count,
            user_message_count,
            assistant_message_count,
            tool_use_count,
            word_count,
            user_word_count,
            assistant_word_count,
            char_count,
            estimated_tokens,
            duration_minutes: 0, // Would need to track session start time
            input_tokens: self.stats.input_tokens,
            output_tokens: self.stats.output_tokens,
            total_cost: self.stats.cost,
            pinned_count,
            bookmarked_count,
            tags_count,
        }
    }

    /// Calculate conversation statistics
    pub fn calculate_stats(&self) -> ConversationStats {
        let mut stats = ConversationStats::default();

        for message in &self.messages {
            stats.message_count += 1;
            let words = message.content.split_whitespace().count();
            stats.word_count += words;
            stats.char_count += message.content.len();

            match message.role {
                MessageRole::User => {
                    stats.user_message_count += 1;
                    stats.user_word_count += words;
                }
                MessageRole::Assistant => {
                    stats.assistant_message_count += 1;
                    stats.assistant_word_count += words;
                }
                MessageRole::ToolUse => {
                    stats.tool_use_count += 1;
                }
                _ => {}
            }
        }

        // Estimate tokens (rough approximation: words * 1.3)
        stats.estimated_tokens = (stats.word_count as f64 * 1.3) as usize;

        // Calculate duration
        if let (Some(first), Some(last)) = (self.messages.first(), self.messages.last()) {
            let duration = last.timestamp - first.timestamp;
            stats.duration_minutes = duration.num_minutes();
        }

        stats
    }

    /// Count messages by role
    pub fn count_messages_by_role(&self, role: MessageRole) -> usize {
        self.messages.iter().filter(|m| m.role == role).count()
    }

    /// Generate a quick summary of conversation topics
    /// Extracts key topics from user messages for quick reference
    pub fn get_conversation_topics(&self) -> Vec<String> {
        let mut topics: Vec<String> = Vec::new();

        for message in &self.messages {
            if message.role == MessageRole::User {
                // Extract first meaningful line as a topic
                let first_line = message.content
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim();

                // Skip if it's a slash command or too short
                if first_line.starts_with('/') || first_line.len() < 10 {
                    continue;
                }

                // Truncate long lines
                let topic = if first_line.len() > 60 {
                    format!("{}...", &first_line[..57])
                } else {
                    first_line.to_string()
                };

                if !topics.contains(&topic) {
                    topics.push(topic);
                }
            }
        }

        // Return last 5 unique topics
        topics.into_iter().rev().take(5).collect()
    }

    /// Get tools used in this conversation
    pub fn get_tools_used(&self) -> Vec<String> {
        let mut tools: Vec<String> = Vec::new();

        for message in &self.messages {
            if message.role == MessageRole::ToolUse {
                // Extract tool name from content (format: "Using tool: ToolName")
                if let Some(tool_name) = message.content.strip_prefix("Using tool: ") {
                    let name = tool_name.lines().next().unwrap_or(tool_name).to_string();
                    if !tools.contains(&name) {
                        tools.push(name);
                    }
                }
            }
        }

        tools
    }

    /// Get a brief summary suitable for display
    pub fn get_brief_summary(&self) -> String {
        let stats = self.calculate_stats();
        let topics = self.get_conversation_topics();
        let tools = self.get_tools_used();

        let mut parts = Vec::new();

        // Message count
        parts.push(format!("{} messages", stats.message_count));

        // Duration
        if stats.duration_minutes > 0 {
            parts.push(stats.format_duration());
        }

        // Topics preview
        if !topics.is_empty() {
            parts.push(format!("{} topics", topics.len()));
        }

        // Tools used
        if !tools.is_empty() {
            parts.push(format!("{} tools", tools.len()));
        }

        parts.join(" · ")
    }

    /// Format token count with K/M suffix
    pub fn format_token_count(tokens: u64) -> String {
        if tokens >= 1_000_000 {
            format!("{:.1}M", tokens as f64 / 1_000_000.0)
        } else if tokens >= 1_000 {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        } else {
            tokens.to_string()
        }
    }
}
