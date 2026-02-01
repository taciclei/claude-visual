//! Input history management

use gpui::*;
use super::ChatInput;

impl ChatInput {
    /// Navigate to previous history entry (up arrow)
    pub(super) fn navigate_history_up(&mut self, cx: &mut Context<Self>) {
        if self.input_history.is_empty() {
            return;
        }

        match self.history_index {
            None => {
                // First time entering history - save current input
                self.saved_current_input = Some(self.text.clone());
                self.history_index = Some(0);
                self.text = self.input_history[0].clone();
            }
            Some(idx) => {
                // Go to older history entry
                if idx + 1 < self.input_history.len() {
                    self.history_index = Some(idx + 1);
                    self.text = self.input_history[idx + 1].clone();
                }
            }
        }
        self.cursor_position = self.text.len();
        cx.notify();
    }

    /// Navigate to next history entry (down arrow)
    pub(super) fn navigate_history_down(&mut self, cx: &mut Context<Self>) {
        match self.history_index {
            Some(0) => {
                // At most recent history entry - restore current input
                self.history_index = None;
                self.text = self.saved_current_input.take().unwrap_or_default();
            }
            Some(idx) => {
                // Go to more recent history entry
                self.history_index = Some(idx - 1);
                self.text = self.input_history[idx - 1].clone();
            }
            None => {
                // Not in history mode, do nothing
            }
        }
        self.cursor_position = self.text.len();
        cx.notify();
    }

    /// Get input history count
    pub fn history_count(&self) -> usize {
        self.input_history.len()
    }

    /// Check if currently browsing history
    pub fn is_browsing_history(&self) -> bool {
        self.history_index.is_some()
    }

    /// Get current history position (1-indexed for display)
    pub fn history_position(&self) -> Option<usize> {
        self.history_index.map(|idx| idx + 1)
    }

    /// Get formatted history indicator like "[2/15 ↑↓]"
    pub fn history_indicator(&self) -> Option<String> {
        if let Some(pos) = self.history_position() {
            let total = self.input_history.len();
            Some(format!("[{}/{} ↑↓]", pos, total))
        } else {
            None
        }
    }

    /// Clear input history
    pub fn clear_history(&mut self) {
        self.input_history.clear();
        self.history_index = None;
        self.saved_current_input = None;
    }
}
