//! Message grouping methods

use gpui::*;

use crate::ui::chat::message::MessageView;

use super::super::core::ChatView;

impl ChatView {
    /// Get time group label for a timestamp (Today, Yesterday, date)
    fn time_group_label(timestamp: chrono::DateTime<chrono::Utc>) -> String {
        use chrono::Datelike;
        let now = chrono::Local::now();
        let local_time = timestamp.with_timezone(&chrono::Local);
        let today = now.date_naive();
        let msg_date = local_time.date_naive();

        if msg_date == today {
            "Today".to_string()
        } else if msg_date == today - chrono::Duration::days(1) {
            "Yesterday".to_string()
        } else if today - msg_date <= chrono::Duration::days(7) {
            // Within last week, show day name
            local_time.format("%A").to_string()
        } else if local_time.year() == now.year() {
            // This year
            local_time.format("%B %d").to_string()
        } else {
            // Different year
            local_time.format("%B %d, %Y").to_string()
        }
    }

    /// Get messages with their time groups for rendering separators
    pub fn messages_with_time_groups(&self, cx: &Context<Self>) -> Vec<(Option<String>, usize, Entity<MessageView>)> {
        let show_bookmarked = self.show_bookmarked_only;
        let mut result = Vec::new();
        let mut last_group: Option<String> = None;

        for ((idx, view), msg) in self.message_views.iter().enumerate().zip(self.messages.iter()) {
            // Apply filters
            if !self.message_filter.includes_role(msg.role) {
                continue;
            }
            if show_bookmarked && !view.read(cx).is_bookmarked() {
                continue;
            }

            // Check if we need a new group separator
            let group = Self::time_group_label(msg.timestamp);
            let show_separator = if let Some(ref last) = last_group {
                &group != last
            } else {
                true // First message always shows separator
            };

            if show_separator {
                last_group = Some(group.clone());
                result.push((Some(group), idx, view.clone()));
            } else {
                result.push((None, idx, view.clone()));
            }
        }

        result
    }
}
