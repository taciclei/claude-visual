//! Input configuration handlers

use gpui::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Toggle multiline input mode
    pub fn toggle_multiline_input(&mut self, cx: &mut Context<Self>) {
        self.multiline_input = !self.multiline_input;
        cx.notify();
    }

    /// Increase input height (multiline mode)
    pub fn increase_input_height(&mut self, cx: &mut Context<Self>) {
        if self.input_height_lines < 20 {
            self.input_height_lines += 1;
            cx.notify();
        }
    }

    /// Decrease input height (multiline mode)
    pub fn decrease_input_height(&mut self, cx: &mut Context<Self>) {
        if self.input_height_lines > 1 {
            self.input_height_lines -= 1;
            cx.notify();
        }
    }

    /// Toggle input hints
    pub fn toggle_input_hints(&mut self, cx: &mut Context<Self>) {
        self.show_input_hints = !self.show_input_hints;
        cx.notify();
    }
}
