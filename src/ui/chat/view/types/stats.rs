//! Conversation statistics

/// Conversation statistics
#[derive(Debug, Clone, Default)]
pub struct ConversationStats {
    /// Total number of messages
    pub message_count: usize,
    /// Number of user messages
    pub user_message_count: usize,
    /// Number of assistant messages
    pub assistant_message_count: usize,
    /// Number of tool uses
    pub tool_use_count: usize,
    /// Total word count
    pub word_count: usize,
    /// User word count
    pub user_word_count: usize,
    /// Assistant word count
    pub assistant_word_count: usize,
    /// Total character count
    pub char_count: usize,
    /// Estimated token count (rough approximation: words * 1.3)
    pub estimated_tokens: usize,
    /// Conversation duration in minutes
    pub duration_minutes: i64,
    /// Input tokens used
    pub input_tokens: u64,
    /// Output tokens used
    pub output_tokens: u64,
    /// Total cost in USD
    pub total_cost: f64,
    /// Number of pinned messages
    pub pinned_count: usize,
    /// Number of bookmarked messages
    pub bookmarked_count: usize,
    /// Number of tags
    pub tags_count: usize,
}

impl ConversationStats {
    /// Format duration as human-readable string
    pub fn format_duration(&self) -> String {
        if self.duration_minutes < 1 {
            "< 1 min".to_string()
        } else if self.duration_minutes < 60 {
            format!("{} min", self.duration_minutes)
        } else {
            let hours = self.duration_minutes / 60;
            let mins = self.duration_minutes % 60;
            if mins == 0 {
                format!("{} hr", hours)
            } else {
                format!("{} hr {} min", hours, mins)
            }
        }
    }

    /// Format token count with K/M suffix
    pub fn format_tokens(&self) -> String {
        if self.estimated_tokens >= 1_000_000 {
            format!("{:.1}M", self.estimated_tokens as f64 / 1_000_000.0)
        } else if self.estimated_tokens >= 1_000 {
            format!("{:.1}K", self.estimated_tokens as f64 / 1_000.0)
        } else {
            self.estimated_tokens.to_string()
        }
    }

    /// Format word count with K suffix
    pub fn format_words(&self) -> String {
        if self.word_count >= 1_000 {
            format!("{:.1}K", self.word_count as f64 / 1_000.0)
        } else {
            self.word_count.to_string()
        }
    }
}
