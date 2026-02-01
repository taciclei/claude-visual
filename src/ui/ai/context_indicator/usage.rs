//! Context usage information and utilities

/// Context usage information
#[derive(Debug, Clone, Default)]
pub struct ContextUsage {
    /// Current token count
    pub current_tokens: usize,
    /// Maximum token limit
    pub max_tokens: usize,
    /// Number of messages in context
    pub message_count: usize,
    /// Number of attached files
    pub file_count: usize,
    /// Estimated cost in USD (optional)
    pub estimated_cost: Option<f64>,
}

impl ContextUsage {
    /// Calculate usage percentage
    pub fn percentage(&self) -> f32 {
        if self.max_tokens == 0 {
            0.0
        } else {
            (self.current_tokens as f32 / self.max_tokens as f32) * 100.0
        }
    }

    /// Check if context is nearly full (>80%)
    pub fn is_warning(&self) -> bool {
        self.percentage() > 80.0
    }

    /// Check if context is critical (>95%)
    pub fn is_critical(&self) -> bool {
        self.percentage() > 95.0
    }

    /// Format token count for display
    pub fn format_tokens(&self) -> String {
        if self.current_tokens >= 1_000_000 {
            format!("{:.1}M", self.current_tokens as f64 / 1_000_000.0)
        } else if self.current_tokens >= 1_000 {
            format!("{:.1}K", self.current_tokens as f64 / 1_000.0)
        } else {
            self.current_tokens.to_string()
        }
    }

    /// Format max tokens for display
    pub fn format_max_tokens(&self) -> String {
        if self.max_tokens >= 1_000_000 {
            format!("{:.0}M", self.max_tokens as f64 / 1_000_000.0)
        } else if self.max_tokens >= 1_000 {
            format!("{:.0}K", self.max_tokens as f64 / 1_000.0)
        } else {
            self.max_tokens.to_string()
        }
    }
}
