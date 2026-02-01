//! Message bookmark methods

use gpui::*;
use std::collections::HashSet;

use crate::claude::message::ClaudeMessage;
use crate::ui::chat::message::MessageView;

use super::super::core::ChatView;
use super::super::types::NotificationType;

impl ChatView {
    /// Toggle bookmark on a message
    pub fn toggle_bookmark(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.bookmarked_messages.contains(&index) {
            self.bookmarked_messages.remove(&index);
            self.show_notification("Bookmark removed", NotificationType::Info, cx);
        } else {
            self.bookmarked_messages.insert(index);
            self.show_notification("Message bookmarked", NotificationType::Success, cx);
        }
        cx.notify();
    }

    /// Check if a message is bookmarked
    pub fn is_bookmarked(&self, index: usize) -> bool {
        self.bookmarked_messages.contains(&index)
    }

    /// Jump to next bookmarked message
    pub fn jump_to_next_bookmark(&mut self, cx: &mut Context<Self>) {
        if self.bookmarked_messages.is_empty() {
            self.show_notification("No bookmarked messages", NotificationType::Info, cx);
            return;
        }

        let mut bookmarks: Vec<usize> = self.bookmarked_messages.iter().copied().collect();
        bookmarks.sort();

        let current = self.selected_message_index.or(
            self.highlighted_message.map(|(idx, _)| idx)
        ).unwrap_or(0);

        let next = bookmarks.iter()
            .find(|&&idx| idx > current)
            .or_else(|| bookmarks.first())
            .copied();

        if let Some(idx) = next {
            self.selected_message_index = Some(idx);
            self.highlighted_message = Some((idx, std::time::Instant::now()));
            self.update_message_selection_states(cx);
            cx.notify();
        }
    }

    /// Jump to previous bookmarked message
    pub fn jump_to_prev_bookmark(&mut self, cx: &mut Context<Self>) {
        if self.bookmarked_messages.is_empty() {
            self.show_notification("No bookmarked messages", NotificationType::Info, cx);
            return;
        }

        let mut bookmarks: Vec<usize> = self.bookmarked_messages.iter().copied().collect();
        bookmarks.sort();
        bookmarks.reverse();

        let current = self.selected_message_index.or(
            self.highlighted_message.map(|(idx, _)| idx)
        ).unwrap_or(self.messages.len());

        let prev = bookmarks.iter()
            .find(|&&idx| idx < current)
            .or_else(|| bookmarks.first())
            .copied();

        if let Some(idx) = prev {
            self.selected_message_index = Some(idx);
            self.highlighted_message = Some((idx, std::time::Instant::now()));
            self.update_message_selection_states(cx);
            cx.notify();
        }
    }

    /// Get all bookmarked message indices
    pub fn get_bookmarked_indices(&self) -> Vec<usize> {
        let mut bookmarks: Vec<usize> = self.bookmarked_messages.iter().copied().collect();
        bookmarks.sort();
        bookmarks
    }

    /// Get bookmarked messages with their content
    pub fn get_bookmarked_messages(&self) -> Vec<(usize, &ClaudeMessage)> {
        self.get_bookmarked_indices()
            .into_iter()
            .filter_map(|idx| self.messages.get(idx).map(|msg| (idx, msg)))
            .collect()
    }

    /// Get bookmarked message count
    pub fn bookmarked_count(&self) -> usize {
        self.bookmarked_messages.len()
    }

    /// Toggle showing only bookmarked messages
    pub fn toggle_bookmarked_only(&mut self, cx: &mut Context<Self>) {
        self.show_bookmarked_only = !self.show_bookmarked_only;
        cx.notify();
    }

    /// Toggle bookmarked filter
    pub fn toggle_bookmarked_filter(&mut self, cx: &mut Context<Self>) {
        self.show_bookmarked_only = !self.show_bookmarked_only;
        cx.notify();
    }

    /// Check if bookmarked filter is active
    pub fn is_bookmarked_filter_active(&self) -> bool {
        self.show_bookmarked_only
    }

    /// Get count of bookmarked messages
    pub fn bookmarked_message_count(&self, cx: &Context<Self>) -> usize {
        self.message_views
            .iter()
            .filter(|view| view.read(cx).is_bookmarked())
            .count()
    }
}
