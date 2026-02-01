//! Input history handlers

use gpui::*;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::NotificationType;

impl ChatView {
    /// Add input to history
    pub fn add_to_input_history(&mut self, input: String) {
        if input.trim().is_empty() {
            return;
        }

        // Don't add duplicates of the last entry
        if self.input_history.history.last() == Some(&input) {
            return;
        }

        self.input_history.history.push(input);

        // Trim to max size
        while self.input_history.history.len() > self.input_history.max_size {
            self.input_history.history.remove(0);
        }

        // Reset position
        self.input_history.position = -1;
        self.input_history.temp.clear();
    }

    /// Navigate to previous input in history (up arrow)
    pub fn input_history_previous(&mut self, cx: &mut Context<Self>) {
        if self.input_history.history.is_empty() {
            return;
        }

        // Save current input if we're just starting to navigate
        if self.input_history.position == -1 {
            self.input_history.temp = self.input.read(cx).text().to_string();
        }

        // Move up in history
        let new_pos = self.input_history.position + 1;
        if new_pos < self.input_history.history.len() as i32 {
            self.input_history.position = new_pos;
            let history_index = self.input_history.history.len() - 1 - new_pos as usize;
            if let Some(historical_input) = self.input_history.history.get(history_index).cloned() {
                self.input.update(cx, |input, cx| {
                    input.set_text(historical_input, cx);
                });
            }
            cx.notify();
        }
    }

    /// Navigate to next input in history (down arrow)
    pub fn input_history_next(&mut self, cx: &mut Context<Self>) {
        if self.input_history.position <= -1 {
            return;
        }

        self.input_history.position -= 1;

        if self.input_history.position == -1 {
            // Restore original input
            let temp = self.input_history.temp.clone();
            self.input.update(cx, |input, cx| {
                input.set_text(temp, cx);
            });
        } else {
            let history_index = self.input_history.history.len() - 1 - self.input_history.position as usize;
            if let Some(historical_input) = self.input_history.history.get(history_index).cloned() {
                self.input.update(cx, |input, cx| {
                    input.set_text(historical_input, cx);
                });
            }
        }
        cx.notify();
    }

    /// Get input history count
    pub fn input_history_count(&self) -> usize {
        self.input_history.history.len()
    }

    /// Clear input history
    pub fn clear_input_history(&mut self, cx: &mut Context<Self>) {
        self.input_history.history.clear();
        self.input_history.position = -1;
        self.input_history.temp.clear();
        self.show_notification("Input history cleared", NotificationType::Info, cx);
    }
}
