//! Think mode and thinking display methods

use gpui::*;

use super::types::NotificationType;
use super::ChatView;

impl ChatView {
    /// Enable extended thinking mode
    pub fn enable_think_mode(&mut self, cx: &mut Context<Self>) {
        self.think_mode_enabled = true;
        self.send_slash_command("/think", cx);
        self.show_notification(
            "Extended thinking enabled - Claude will reason more deeply",
            NotificationType::Info,
            cx,
        );
        cx.notify();
    }

    /// Disable extended thinking mode
    pub fn disable_think_mode(&mut self, cx: &mut Context<Self>) {
        self.think_mode_enabled = false;
        self.send_slash_command("/think off", cx);
        self.show_notification("Extended thinking disabled", NotificationType::Info, cx);
        cx.notify();
    }

    /// Toggle extended thinking mode
    pub fn toggle_think_mode(&mut self, cx: &mut Context<Self>) {
        if self.think_mode_enabled {
            self.disable_think_mode(cx);
        } else {
            self.enable_think_mode(cx);
        }
    }

    /// Check if think mode is enabled
    pub fn is_think_mode_enabled(&self) -> bool {
        self.think_mode_enabled
    }

    /// Toggle thinking/reasoning display
    pub fn toggle_thinking(&mut self, cx: &mut Context<Self>) {
        self.show_thinking = !self.show_thinking;
        cx.notify();
    }

    /// Toggle include thinking option
    pub fn toggle_export_thinking(&mut self, cx: &mut Context<Self>) {
        self.export.include_thinking = !self.export.include_thinking;
        cx.notify();
    }
}
