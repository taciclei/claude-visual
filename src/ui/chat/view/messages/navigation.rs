//! Message navigation methods

use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    /// Navigate back in message history
    pub fn navigate_back(&mut self, cx: &mut Context<Self>) {
        if self.navigation_history_position > 0 {
            self.navigation_history_position -= 1;
            if let Some(&message_index) = self.navigation_history.get(self.navigation_history_position) {
                self.selected_message_index = Some(message_index);
                self.highlighted_message = Some((message_index, std::time::Instant::now()));
                self.update_message_selection_states(cx);
                cx.notify();
            }
        }
    }

    /// Navigate forward in message history
    pub fn navigate_forward(&mut self, cx: &mut Context<Self>) {
        if self.navigation_history_position < self.navigation_history.len() {
            self.navigation_history_position += 1;
            if let Some(&message_index) = self.navigation_history.get(self.navigation_history_position) {
                self.selected_message_index = Some(message_index);
                self.highlighted_message = Some((message_index, std::time::Instant::now()));
                self.update_message_selection_states(cx);
                cx.notify();
            }
        }
    }

    /// Check if we can navigate back
    pub fn can_navigate_back(&self) -> bool {
        self.navigation_history_position > 0
    }

    /// Check if we can navigate forward
    pub fn can_navigate_forward(&self) -> bool {
        self.navigation_history_position < self.navigation_history.len()
    }

    /// Check if a message is currently highlighted (returns opacity 0.0-1.0 for animation)
    pub fn message_highlight_opacity(&self, index: usize) -> f32 {
        if let Some((highlighted_idx, timestamp)) = self.highlighted_message {
            if highlighted_idx == index {
                let elapsed = timestamp.elapsed().as_millis() as f32;
                // Highlight fades over 2 seconds
                let fade_duration = 2000.0;
                if elapsed < fade_duration {
                    return 1.0 - (elapsed / fade_duration);
                }
            }
        }
        0.0
    }
}
