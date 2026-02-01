//! Message pinning methods

use gpui::*;

use crate::claude::message::ClaudeMessage;

use super::super::core::ChatView;
use super::super::types::NotificationType;

impl ChatView {
    /// Pin/unpin a message
    pub fn toggle_pin(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.pinned_messages.contains(&index) {
            self.pinned_messages.remove(&index);
            self.show_notification("Message unpinned".to_string(), NotificationType::Info, cx);
        } else {
            self.pinned_messages.insert(index);
            self.show_notification("Message pinned".to_string(), NotificationType::Success, cx);
        }
        cx.notify();
    }

    /// Check if message is pinned
    pub fn is_pinned(&self, index: usize) -> bool {
        self.pinned_messages.contains(&index)
    }

    /// Get pinned message count
    pub fn pinned_count(&self) -> usize {
        self.pinned_messages.len()
    }

    /// Get sorted list of pinned message indices
    pub fn get_pinned_indices(&self) -> Vec<usize> {
        let mut indices: Vec<_> = self.pinned_messages.iter().copied().collect();
        indices.sort();
        indices
    }

    /// Get pinned messages with their content
    pub fn get_pinned_messages(&self) -> Vec<(usize, &ClaudeMessage)> {
        self.get_pinned_indices()
            .into_iter()
            .filter_map(|idx| self.messages.get(idx).map(|msg| (idx, msg)))
            .collect()
    }

    /// Toggle pinned messages panel
    pub fn toggle_pinned_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.pinned_panel = !self.panels.pinned_panel;
        cx.notify();
    }
}
