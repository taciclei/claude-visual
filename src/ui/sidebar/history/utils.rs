//! Utility functions for history sidebar

use super::core::HistorySidebar;

impl HistorySidebar {
    /// Format relative time
    pub(super) fn format_relative_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(*dt);

        if duration.num_minutes() < 1 {
            "Just now".to_string()
        } else if duration.num_hours() < 1 {
            format!("{} min ago", duration.num_minutes())
        } else if duration.num_days() < 1 {
            format!("{} hours ago", duration.num_hours())
        } else if duration.num_days() < 7 {
            format!("{} days ago", duration.num_days())
        } else {
            dt.format("%Y-%m-%d").to_string()
        }
    }

    /// Truncate text with ellipsis
    pub(super) fn truncate(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }
}
