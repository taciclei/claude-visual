//! Vim mode integration

use super::ChatInput;
use crate::ui::vim::{VimAction, VimMode, VimState};
use gpui::*;

impl ChatInput {
    /// Check if vim state exists
    pub fn has_vim_state(&self) -> bool {
        self.vim_state.is_some()
    }

    /// Check if vim mode is active
    pub fn vim_mode(&self, cx: &Context<Self>) -> Option<VimMode> {
        self.vim_state.as_ref().and_then(|vs| {
            let state = vs.read(cx);
            if state.is_enabled() {
                Some(state.mode())
            } else {
                None
            }
        })
    }

    /// Toggle vim mode
    pub fn toggle_vim_mode(&mut self, cx: &mut Context<Self>) {
        if let Some(vim_state) = &self.vim_state {
            vim_state.update(cx, |vs, cx| vs.toggle(cx));
        } else {
            // Create vim state if it doesn't exist
            let state = self.app_state.clone();
            let vim = cx.new(|cx| {
                let mut vim = VimState::new(state, cx);
                vim.enable(cx);
                vim
            });
            self.vim_state = Some(vim);
        }
        cx.notify();
    }

    /// Apply a vim action to the text
    pub(super) fn apply_vim_action(&mut self, action: VimAction, cx: &mut Context<Self>) {
        match action {
            // Mode transitions
            VimAction::EnterInsertMode => {
                // Stay at current position
            }
            VimAction::EnterInsertModeAppend => {
                self.move_cursor_right(1, cx);
            }
            VimAction::EnterInsertModeLineStart => {
                self.cursor_position = self.find_line_start();
                cx.notify();
            }
            VimAction::EnterInsertModeLineEnd => {
                self.cursor_position = self.find_line_end();
                cx.notify();
            }
            VimAction::EnterInsertModeNewLineBelow => {
                self.cursor_position = self.find_line_end();
                self.insert_char('\n', cx);
            }
            VimAction::EnterInsertModeNewLineAbove => {
                self.cursor_position = self.find_line_start();
                self.insert_char('\n', cx);
                self.move_cursor_left(1, cx);
            }
            VimAction::ExitInsertMode => {
                // Move cursor left when exiting insert mode (vim behavior)
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    cx.notify();
                }
            }

            // Cursor movement
            VimAction::MoveLeft(n) => self.move_cursor_left(n, cx),
            VimAction::MoveRight(n) => self.move_cursor_right(n, cx),
            VimAction::MoveUp(_n) => {
                // In single-line context, move to start
                self.cursor_position = 0;
                cx.notify();
            }
            VimAction::MoveDown(_n) => {
                // In single-line context, move to end
                self.cursor_position = self.text.len();
                cx.notify();
            }
            VimAction::MoveWordForward(n) => {
                for _ in 0..n {
                    self.cursor_position = self.find_next_word_start();
                }
                cx.notify();
            }
            VimAction::MoveWordBackward(n) => {
                for _ in 0..n {
                    self.cursor_position = self.find_prev_word_start();
                }
                cx.notify();
            }
            VimAction::MoveWordEnd(n) => {
                for _ in 0..n {
                    self.cursor_position = self.find_word_end();
                }
                cx.notify();
            }
            VimAction::MoveLineStart => {
                self.cursor_position = self.find_line_start();
                cx.notify();
            }
            VimAction::MoveLineFirstNonBlank => {
                self.cursor_position = self.find_first_non_blank();
                cx.notify();
            }
            VimAction::MoveLineEnd => {
                self.cursor_position = self.find_line_end();
                cx.notify();
            }
            VimAction::MoveToTop => {
                self.cursor_position = 0;
                cx.notify();
            }
            VimAction::MoveToBottom => {
                self.cursor_position = self.text.len();
                cx.notify();
            }

            // Text manipulation
            VimAction::DeleteChar => {
                self.delete_char_at_cursor(cx);
            }
            VimAction::DeleteCharBefore => {
                self.delete_char_before(cx);
            }
            VimAction::DeleteLine => {
                // Delete current line
                let start = self.find_line_start();
                let end = self.find_line_end();
                self.delete_range(start, end + 1, cx);
            }
            VimAction::Delete => {
                // Delete selection or char
                self.delete_char_at_cursor(cx);
            }

            // Undo/Redo (placeholder - would need proper undo stack)
            VimAction::Undo | VimAction::Redo => {
                // TODO: Implement undo stack
            }

            _ => {}
        }
    }

    /// Move cursor left by n characters
    pub(super) fn move_cursor_left(&mut self, n: usize, cx: &mut Context<Self>) {
        self.cursor_position = self.cursor_position.saturating_sub(n);
        cx.notify();
    }

    /// Move cursor right by n characters
    pub(super) fn move_cursor_right(&mut self, n: usize, cx: &mut Context<Self>) {
        self.cursor_position = (self.cursor_position + n).min(self.text.len());
        cx.notify();
    }

    /// Find start of current line
    pub(super) fn find_line_start(&self) -> usize {
        self.text[..self.cursor_position]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0)
    }

    /// Find end of current line
    pub(super) fn find_line_end(&self) -> usize {
        self.text[self.cursor_position..]
            .find('\n')
            .map(|i| self.cursor_position + i)
            .unwrap_or(self.text.len())
    }

    /// Find first non-blank character on current line
    pub(super) fn find_first_non_blank(&self) -> usize {
        let line_start = self.find_line_start();
        let line_end = self.find_line_end();
        self.text[line_start..line_end]
            .find(|c: char| !c.is_whitespace())
            .map(|i| line_start + i)
            .unwrap_or(line_start)
    }

    /// Find start of next word
    pub(super) fn find_next_word_start(&self) -> usize {
        let mut pos = self.cursor_position;
        let chars: Vec<char> = self.text.chars().collect();
        let len = chars.len();

        // Skip current word
        while pos < len && !chars[pos].is_whitespace() {
            pos += 1;
        }
        // Skip whitespace
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }
        pos
    }

    /// Find start of previous word
    pub(super) fn find_prev_word_start(&self) -> usize {
        let mut pos = self.cursor_position;
        let chars: Vec<char> = self.text.chars().collect();

        if pos == 0 {
            return 0;
        }
        pos -= 1;

        // Skip whitespace
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }
        // Skip to start of word
        while pos > 0 && !chars[pos - 1].is_whitespace() {
            pos -= 1;
        }
        pos
    }

    /// Find end of current word
    pub(super) fn find_word_end(&self) -> usize {
        let mut pos = self.cursor_position;
        let chars: Vec<char> = self.text.chars().collect();
        let len = chars.len();

        if pos >= len {
            return len;
        }

        // Move to next position first
        pos += 1;
        // Skip whitespace
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }
        // Move to end of word
        while pos < len && !chars[pos].is_whitespace() {
            pos += 1;
        }
        pos.saturating_sub(1)
    }
}
