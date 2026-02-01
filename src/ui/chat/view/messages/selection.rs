//! Message selection methods

use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    /// Get selected message index
    pub fn selected_message_index(&self) -> Option<usize> {
        self.selected_message_index
    }

    /// Get selected message index (1-based for display)
    pub fn selected_message_position(&self) -> Option<usize> {
        self.selected_message_index.map(|i| i + 1)
    }

    /// Select a message by index
    pub fn select_message(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        if let Some(idx) = index {
            if idx < self.messages.len() {
                self.selected_message_index = Some(idx);
            }
        } else {
            self.selected_message_index = None;
        }
        self.update_message_selection_states(cx);
        cx.notify();
    }

    /// Select next message (move down)
    pub fn select_next_message(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            return;
        }

        let next = match self.selected_message_index {
            None => 0,
            Some(idx) if idx + 1 < self.messages.len() => idx + 1,
            Some(idx) => idx, // Stay at last message
        };

        self.selected_message_index = Some(next);
        self.update_message_selection_states(cx);
        cx.notify();
    }

    /// Select previous message (move up)
    pub fn select_prev_message(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            return;
        }

        let prev = match self.selected_message_index {
            None => self.messages.len().saturating_sub(1),
            Some(0) => 0, // Stay at first message
            Some(idx) => idx - 1,
        };

        self.selected_message_index = Some(prev);
        self.update_message_selection_states(cx);
        cx.notify();
    }

    /// Update selection state on all message views
    pub(crate) fn update_message_selection_states(&mut self, cx: &mut Context<Self>) {
        for (idx, view) in self.message_views.iter().enumerate() {
            let is_selected = self.selected_message_index == Some(idx);
            view.update(cx, |v, cx| v.set_selected(is_selected, cx));
        }
    }
}
