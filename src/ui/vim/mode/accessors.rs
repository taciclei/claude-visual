//! Accessors for VimState cursor, buffers, and focus

use super::vim_state::VimState;
use gpui::*;

impl VimState {
    /// Get command buffer
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }

    /// Get search pattern
    pub fn search_pattern(&self) -> &str {
        &self.search_pattern
    }

    /// Get cursor position
    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, line: usize, col: usize, cx: &mut Context<Self>) {
        self.cursor = (line, col);
        cx.notify();
    }

    /// Get focus handle
    pub fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
