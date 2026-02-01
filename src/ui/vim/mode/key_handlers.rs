//! Key handling for different Vim modes

use gpui::*;
use super::vim_state::VimState;
use super::VimMode;
use super::super::{VimAction, VimEvent};

impl VimState {
    /// Handle a key event
    pub fn handle_key(&mut self, key: &str, window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        if !self.enabled {
            return None;
        }

        match self.mode {
            VimMode::Normal => self.handle_normal_key(key, window, cx),
            VimMode::Insert => self.handle_insert_key(key, window, cx),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => {
                self.handle_visual_key(key, window, cx)
            }
            VimMode::Command => self.handle_command_key(key, window, cx),
            VimMode::Search => self.handle_search_key(key, window, cx),
        }
    }

    /// Handle key in normal mode
    pub(super) fn handle_normal_key(&mut self, key: &str, _window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        // Check for count prefix (0-9)
        if let Some(digit) = key.chars().next().filter(|c| c.is_ascii_digit()) {
            if digit != '0' || self.count.is_some() {
                let d = digit.to_digit(10).unwrap() as usize;
                self.count = Some(self.count.unwrap_or(0) * 10 + d);
                return None;
            }
        }

        let count = self.count.take().unwrap_or(1);
        let action = self.key_handler.handle_normal(key, count, self.pending_operator);

        if let Some(ref a) = action {
            match a {
                VimAction::EnterInsertMode => {
                    self.set_mode(VimMode::Insert, cx);
                }
                VimAction::EnterInsertModeAppend => {
                    self.set_mode(VimMode::Insert, cx);
                    // Move cursor right first
                }
                VimAction::EnterInsertModeLineStart => {
                    self.set_mode(VimMode::Insert, cx);
                    // Move to line start first
                }
                VimAction::EnterInsertModeLineEnd => {
                    self.set_mode(VimMode::Insert, cx);
                    // Move to line end first
                }
                VimAction::EnterInsertModeNewLineBelow => {
                    self.set_mode(VimMode::Insert, cx);
                }
                VimAction::EnterInsertModeNewLineAbove => {
                    self.set_mode(VimMode::Insert, cx);
                }
                VimAction::EnterVisualMode => {
                    self.set_mode(VimMode::Visual, cx);
                }
                VimAction::EnterVisualLineMode => {
                    self.set_mode(VimMode::VisualLine, cx);
                }
                VimAction::EnterVisualBlockMode => {
                    self.set_mode(VimMode::VisualBlock, cx);
                }
                VimAction::EnterCommandMode => {
                    self.set_mode(VimMode::Command, cx);
                }
                VimAction::EnterSearchMode => {
                    self.set_mode(VimMode::Search, cx);
                }
                VimAction::SetOperator(op) => {
                    self.pending_operator = Some(*op);
                    return None; // Don't emit action yet
                }
                _ => {}
            }
            cx.emit(VimEvent::ActionExecuted(a.clone()));
        }

        action
    }

    /// Handle key in insert mode
    pub(super) fn handle_insert_key(&mut self, key: &str, _window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        if key == "escape" {
            self.set_mode(VimMode::Normal, cx);
            Some(VimAction::ExitInsertMode)
        } else {
            // In insert mode, most keys pass through
            None
        }
    }

    /// Handle key in visual mode
    pub(super) fn handle_visual_key(&mut self, key: &str, _window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        if key == "escape" {
            self.set_mode(VimMode::Normal, cx);
            return Some(VimAction::ExitVisualMode);
        }

        let count = self.count.take().unwrap_or(1);
        let action = self.key_handler.handle_visual(key, count);

        if let Some(ref a) = action {
            match a {
                VimAction::Yank | VimAction::Delete | VimAction::Change => {
                    self.set_mode(VimMode::Normal, cx);
                }
                _ => {}
            }
            cx.emit(VimEvent::ActionExecuted(a.clone()));
        }

        action
    }

    /// Handle key in command mode
    pub(super) fn handle_command_key(&mut self, key: &str, _window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        match key {
            "escape" => {
                self.set_mode(VimMode::Normal, cx);
                Some(VimAction::CancelCommand)
            }
            "enter" => {
                let cmd = self.command_buffer.clone();
                self.set_mode(VimMode::Normal, cx);
                cx.emit(VimEvent::CommandEntered(cmd.clone()));
                Some(VimAction::ExecuteCommand(cmd))
            }
            "backspace" => {
                self.command_buffer.pop();
                cx.notify();
                None
            }
            _ if key.len() == 1 => {
                self.command_buffer.push_str(key);
                cx.notify();
                None
            }
            _ => None,
        }
    }

    /// Handle key in search mode
    pub(super) fn handle_search_key(&mut self, key: &str, _window: &mut Window, cx: &mut Context<Self>) -> Option<VimAction> {
        match key {
            "escape" => {
                self.set_mode(VimMode::Normal, cx);
                Some(VimAction::CancelSearch)
            }
            "enter" => {
                self.last_search = self.search_pattern.clone();
                self.set_mode(VimMode::Normal, cx);
                Some(VimAction::ExecuteSearch(self.search_pattern.clone()))
            }
            "backspace" => {
                self.search_pattern.pop();
                cx.notify();
                None
            }
            _ if key.len() == 1 => {
                self.search_pattern.push_str(key);
                cx.notify();
                None
            }
            _ => None,
        }
    }
}
