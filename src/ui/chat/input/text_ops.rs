//! Text operation handlers for chat input

use gpui::*;

use super::{ChatInput, ChatInputEvent};

impl ChatInput {
    /// Submit the current message
    pub(super) fn submit(&mut self, cx: &mut Context<Self>) {
        tracing::info!(
            "ChatInput::submit called, disabled={}, text='{}'",
            self.is_disabled,
            self.text
        );
        if self.is_disabled {
            tracing::warn!("Submit blocked - input is disabled");
            return;
        }
        let text = self.text.trim().to_string();
        if !text.is_empty() {
            tracing::info!("Emitting ChatInputEvent::Submit with text: '{}'", text);
            // Add to input history (avoid duplicates at the front)
            if self.input_history.first() != Some(&text) {
                self.input_history.insert(0, text.clone());
                // Keep max 100 history entries
                if self.input_history.len() > 100 {
                    self.input_history.pop();
                }
            }
            // Reset history navigation
            self.history_index = None;
            self.saved_current_input = None;
            // Collect file mentions before clearing
            let file_mentions = self.file_mentions();
            if !file_mentions.is_empty() {
                cx.emit(ChatInputEvent::FilesAttached(file_mentions));
            }
            cx.emit(ChatInputEvent::Submit(text));
            self.text.clear();
            self.mentions.clear();
            self.partial_mention = None;
            self.cursor_position = 0;
            cx.notify();
        }
    }

    /// Insert a character at cursor position
    pub(super) fn insert_char(&mut self, ch: char, cx: &mut Context<Self>) {
        self.text.insert(self.cursor_position, ch);
        self.cursor_position += 1;
        self.update_mentions();
        cx.notify();
    }

    /// Delete character at cursor position
    pub(super) fn delete_char_at_cursor(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position < self.text.len() {
            self.text.remove(self.cursor_position);
            self.update_mentions();
            cx.notify();
        }
    }

    /// Delete character before cursor position
    pub(super) fn delete_char_before(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_position);
            self.update_mentions();
            cx.notify();
        }
    }

    /// Delete a range of text
    pub(super) fn delete_range(&mut self, start: usize, end: usize, cx: &mut Context<Self>) {
        let end = end.min(self.text.len());
        let start = start.min(end);
        self.text.drain(start..end);
        self.cursor_position = start;
        self.update_mentions();
        cx.notify();
    }

    /// Delete word before cursor (Ctrl+W)
    pub(super) fn delete_word_before(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position == 0 {
            return;
        }
        let text_before = &self.text[..self.cursor_position];
        // Skip trailing whitespace
        let end_pos = text_before.trim_end().len();
        if end_pos == 0 {
            // All whitespace, delete it all
            self.delete_range(0, self.cursor_position, cx);
            return;
        }
        // Find start of the word
        let start_pos = text_before[..end_pos]
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        self.delete_range(start_pos, self.cursor_position, cx);
    }
}
