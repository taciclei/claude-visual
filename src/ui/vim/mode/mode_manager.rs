//! Mode management for VimState

use gpui::*;
use super::vim_state::VimState;
use super::VimMode;
use super::super::VimEvent;

impl VimState {
    /// Enable vim mode
    pub fn enable(&mut self, cx: &mut Context<Self>) {
        self.enabled = true;
        self.mode = VimMode::Normal;
        cx.emit(VimEvent::ModeChanged(self.mode));
        cx.notify();
    }

    /// Disable vim mode
    pub fn disable(&mut self, cx: &mut Context<Self>) {
        self.enabled = false;
        self.mode = VimMode::Insert; // When disabled, act like always in insert mode
        cx.notify();
    }

    /// Toggle vim mode
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            self.disable(cx);
        } else {
            self.enable(cx);
        }
    }

    /// Switch to a new mode
    pub fn set_mode(&mut self, mode: VimMode, cx: &mut Context<Self>) {
        if self.mode != mode {
            // Handle mode transition
            match (self.mode, mode) {
                (_, VimMode::Insert) => {
                    // Entering insert mode
                    self.visual_start = None;
                }
                (_, VimMode::Visual) => {
                    // Entering visual mode - set anchor
                    self.visual_start = Some(self.cursor);
                }
                (_, VimMode::Normal) => {
                    // Returning to normal mode
                    self.visual_start = None;
                    self.command_buffer.clear();
                    self.search_pattern.clear();
                    self.pending_operator = None;
                    self.count = None;
                }
                (_, VimMode::Command) => {
                    // Entering command mode
                    self.command_buffer.clear();
                }
                (_, VimMode::Search) => {
                    // Entering search mode
                    self.search_pattern.clear();
                }
                _ => {}
            }

            self.mode = mode;
            cx.emit(VimEvent::ModeChanged(mode));
            cx.notify();
        }
    }
}
