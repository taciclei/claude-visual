//! Input focus handling for ChatView

use super::core::ChatView;
use gpui::*;

impl ChatView {
    /// Focus the chat input field
    pub fn focus_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, _cx| {
            input.focus_handle.focus(window);
        });
        cx.notify();
    }
}
