//! Mouse and drag event handlers

use gpui::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Set file drag active state
    pub fn set_file_drag_active(&mut self, active: bool, file_count: usize, cx: &mut Context<Self>) {
        self.file_drag_active = active;
        self.drag_file_count = file_count;
        cx.notify();
    }
}
