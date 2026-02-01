//! Textarea event handlers

use gpui::*;
use crate::ui::components::textarea::types::*;
use super::state::Textarea;

impl Textarea {
    /// Handle key input
    pub(super) fn handle_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.readonly {
            return;
        }

        let modifiers = event.keystroke.modifiers;
        let key = &event.keystroke.key;

        match key.as_str() {
            "enter" => {
                if modifiers.platform || modifiers.control {
                    // Submit on Cmd+Enter or Ctrl+Enter
                    cx.emit(TextareaEvent::Submit(self.text.clone()));
                } else {
                    // Regular enter adds newline
                    self.text.push('\n');
                    cx.emit(TextareaEvent::Changed(self.text.clone()));
                }
            }
            "backspace" => {
                if !self.text.is_empty() {
                    self.text.pop();
                    cx.emit(TextareaEvent::Changed(self.text.clone()));
                }
            }
            _ => {}
        }

        cx.notify();
    }

    /// Handle text input
    pub(super) fn handle_input(&mut self, text: &str, _window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.readonly {
            return;
        }

        // Check max length
        if let Some(max) = self.max_length {
            let remaining = max.saturating_sub(self.text.len());
            if remaining == 0 {
                return;
            }
            let chars_to_add: String = text.chars().take(remaining).collect();
            self.text.push_str(&chars_to_add);
        } else {
            self.text.push_str(text);
        }

        cx.emit(TextareaEvent::Changed(self.text.clone()));
        cx.notify();
    }
}
