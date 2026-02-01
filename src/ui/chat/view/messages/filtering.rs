//! Message filtering methods

use gpui::*;

use crate::ui::chat::message::MessageView;

use super::super::core::ChatView;
use super::super::types::MessageFilter;

impl ChatView {
    /// Get current message filter
    pub fn message_filter(&self) -> MessageFilter {
        self.message_filter
    }

    /// Set message filter
    pub fn set_message_filter(&mut self, filter: MessageFilter, cx: &mut Context<Self>) {
        self.message_filter = filter;
        cx.notify();
    }

    /// Cycle to next filter option
    pub fn next_filter(&mut self, cx: &mut Context<Self>) {
        let options = MessageFilter::all_options();
        let current_idx = options.iter()
            .position(|f| *f == self.message_filter)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % options.len();
        self.message_filter = options[next_idx];
        cx.notify();
    }

    /// Get filtered message views
    pub fn filtered_message_views(&self, cx: &Context<Self>) -> Vec<Entity<MessageView>> {
        let show_bookmarked = self.show_bookmarked_only;
        self.message_views
            .iter()
            .zip(self.messages.iter())
            .filter(|(view, msg)| {
                // First apply role filter
                if !self.message_filter.includes_role(msg.role) {
                    return false;
                }
                // Then apply bookmark filter if enabled
                if show_bookmarked && !view.read(cx).is_bookmarked() {
                    return false;
                }
                true
            })
            .map(|(view, _)| view.clone())
            .collect()
    }

    /// Get filtered message views with indices for highlight tracking
    pub fn filtered_message_views_with_indices(&self, cx: &Context<Self>) -> Vec<(usize, Entity<MessageView>)> {
        let show_bookmarked = self.show_bookmarked_only;
        self.message_views
            .iter()
            .enumerate()
            .zip(self.messages.iter())
            .filter(|((_, view), msg)| {
                // First apply role filter
                if !self.message_filter.includes_role(msg.role) {
                    return false;
                }
                // Then apply bookmark filter if enabled
                if show_bookmarked && !view.read(cx).is_bookmarked() {
                    return false;
                }
                true
            })
            .map(|((idx, view), _)| (idx, view.clone()))
            .collect()
    }

    /// Get count of visible messages with current filter
    pub fn visible_message_count(&self, cx: &Context<Self>) -> usize {
        let show_bookmarked = self.show_bookmarked_only;
        self.message_views
            .iter()
            .zip(self.messages.iter())
            .filter(|(view, msg)| {
                if !self.message_filter.includes_role(msg.role) {
                    return false;
                }
                if show_bookmarked && !view.read(cx).is_bookmarked() {
                    return false;
                }
                true
            })
            .count()
    }

    /// Get count of messages for a specific filter
    pub fn message_count_for_filter(&self, filter: MessageFilter) -> usize {
        self.messages
            .iter()
            .filter(|msg| filter.includes_role(msg.role))
            .count()
    }
}
