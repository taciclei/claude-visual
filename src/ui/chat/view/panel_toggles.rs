//! Panel toggle methods for ChatView

use super::core::ChatView;
use gpui::Context;

impl ChatView {
    /// Toggle quick settings panel
    pub fn toggle_quick_settings(&mut self, cx: &mut Context<Self>) {
        self.panels.quick_settings = !self.panels.quick_settings;
        cx.notify();
    }

    /// Toggle statistics panel
    pub fn toggle_stats_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.stats_panel = !self.panels.stats_panel;
        cx.notify();
    }
}
