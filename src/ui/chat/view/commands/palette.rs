//! Command palette state management
//!
//! This module contains palette toggle and state management logic.

use super::super::core::ChatView;
use gpui::*;

impl ChatView {
    /// Toggle command palette
    pub fn toggle_command_palette(&mut self, cx: &mut Context<Self>) {
        self.panels.command_palette = !self.panels.command_palette;
        if self.panels.command_palette {
            self.palette.query.clear();
            self.palette.selected_index = 0;
        }
        cx.notify();
    }
}
