//! Key event handlers for chat input

use gpui::*;

use super::{ChatInput, ChatInputEvent};
use crate::ui::vim::VimMode;

impl ChatInput {
    /// Handle key input
    pub(super) fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        tracing::info!(
            "ChatInput::handle_key_down: key={}, disabled={}",
            event.keystroke.key,
            self.is_disabled
        );
        if self.is_disabled {
            tracing::warn!("Key ignored - input is disabled");
            return;
        }

        let key = &event.keystroke.key;

        // Check if vim mode is active and handle key through vim state
        if let Some(vim_state) = &self.vim_state {
            let vim_mode = vim_state.read(cx).mode();
            let vim_enabled = vim_state.read(cx).is_enabled();

            if vim_enabled {
                // Build key string with modifiers for vim
                let vim_key = if event.keystroke.modifiers.control {
                    format!("ctrl-{}", key)
                } else if event.keystroke.modifiers.alt {
                    format!("alt-{}", key)
                } else {
                    key.clone()
                };

                // Route key through vim state
                let action = vim_state.update(cx, |vs, cx| vs.handle_key(&vim_key, window, cx));

                if let Some(action) = action {
                    self.apply_vim_action(action, cx);
                    return;
                }

                // In normal/visual mode, don't pass keys through
                if vim_mode != VimMode::Insert {
                    return;
                }
            }
        }

        // Regular key handling (insert mode or vim disabled)
        match key.as_str() {
            "enter" => {
                if self.show_command_autocomplete {
                    // Select current command from autocomplete
                    self.insert_selected_command(cx);
                } else if self.show_file_autocomplete {
                    // Select current file from autocomplete
                    self.insert_selected_file(cx);
                } else if event.keystroke.modifiers.shift {
                    // Shift+Enter: insert newline
                    self.insert_char('\n', cx);
                } else {
                    // Enter: submit
                    self.submit(cx);
                }
            }
            "tab" => {
                if self.show_command_autocomplete {
                    // Tab completes the selected command
                    self.insert_selected_command(cx);
                } else if self.show_file_autocomplete {
                    // Tab completes the selected file
                    self.insert_selected_file(cx);
                }
            }
            "up" => {
                if self.show_command_autocomplete {
                    self.select_previous_command(cx);
                } else if self.show_file_autocomplete {
                    self.select_previous_file(cx);
                } else if self.text.is_empty() || !self.text.contains('\n') {
                    // Navigate to previous history entry (only if single line or empty)
                    self.navigate_history_up(cx);
                }
            }
            "down" => {
                if self.show_command_autocomplete {
                    self.select_next_command(cx);
                } else if self.show_file_autocomplete {
                    self.select_next_file(cx);
                } else if self.history_index.is_some() {
                    // Navigate to next history entry (only when in history mode)
                    self.navigate_history_down(cx);
                }
            }
            "backspace" => {
                self.delete_char_before(cx);
                self.update_mentions_with_autocomplete(cx);
                self.update_command_autocomplete();
                cx.notify();
            }
            "delete" => {
                self.delete_char_at_cursor(cx);
                self.update_mentions_with_autocomplete(cx);
                self.update_command_autocomplete();
                cx.notify();
            }
            "left" => {
                self.move_cursor_left(1, cx);
            }
            "right" => {
                self.move_cursor_right(1, cx);
            }
            "home" => {
                self.cursor_position = 0;
                cx.notify();
            }
            "end" => {
                self.cursor_position = self.text.len();
                cx.notify();
            }
            "escape" => {
                if self.show_command_autocomplete {
                    // Close command autocomplete
                    self.show_command_autocomplete = false;
                    cx.notify();
                } else if self.show_file_autocomplete {
                    // Close file autocomplete
                    self.close_file_autocomplete(cx);
                } else if self.show_templates {
                    // Close templates dropdown
                    self.show_templates = false;
                    cx.notify();
                } else if let Some(vim_state) = &self.vim_state {
                    // If vim mode exists, escape should switch to normal mode
                    vim_state.update(cx, |vs, cx| vs.set_mode(VimMode::Normal, cx));
                }
            }
            "u" if event.keystroke.modifiers.control => {
                // Ctrl+U: Clear line (Unix-style)
                self.clear(cx);
            }
            "k" if event.keystroke.modifiers.control => {
                // Ctrl+K: Kill to end of line
                let pos = self.cursor_position;
                self.text.truncate(pos);
                cx.notify();
            }
            "a" if event.keystroke.modifiers.control => {
                // Ctrl+A: Go to beginning
                self.cursor_position = 0;
                cx.notify();
            }
            "e" if event.keystroke.modifiers.control => {
                // Ctrl+E: Go to end
                self.cursor_position = self.text.len();
                cx.notify();
            }
            "w" if event.keystroke.modifiers.control => {
                // Ctrl+W: Delete word before cursor
                self.delete_word_before(cx);
            }
            "l" if event.keystroke.modifiers.control => {
                // Ctrl+L: Clear conversation (like terminal clear)
                cx.emit(ChatInputEvent::ClearConversation);
            }
            "r" if event.keystroke.modifiers.control => {
                // Ctrl+R: Reverse history search
                cx.emit(ChatInputEvent::OpenHistorySearch);
            }
            _ => {
                // Handle character input via key_char
                if let Some(key_char) = &event.keystroke.key_char {
                    // Don't insert if modifier keys are held (except shift)
                    if !event.keystroke.modifiers.control
                        && !event.keystroke.modifiers.alt
                        && !event.keystroke.modifiers.platform
                    {
                        self.handle_input(key_char, window, cx);
                    }
                }
            }
        }
    }

    /// Handle text input (for regular characters)
    pub(super) fn handle_input(
        &mut self,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_disabled {
            return;
        }

        // Only accept text input in insert mode when vim is enabled
        if let Some(vim_state) = &self.vim_state {
            let vim_mode = vim_state.read(cx).mode();
            let vim_enabled = vim_state.read(cx).is_enabled();

            if vim_enabled && vim_mode != VimMode::Insert {
                // In normal/visual mode, text input goes through vim handler
                return;
            }
        }

        // Insert text at cursor position
        for ch in text.chars() {
            self.text.insert(self.cursor_position, ch);
            self.cursor_position += 1;
        }
        self.update_mentions_with_autocomplete(cx);
        self.update_command_autocomplete();
        cx.notify();
    }
}
