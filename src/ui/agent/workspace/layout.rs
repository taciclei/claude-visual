//! Layout and UI control methods

use super::state::AgentWorkspace;
use super::types::*;
use gpui::*;

impl AgentWorkspace {
    /// Toggle expanded state
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        self.layout = if self.is_expanded {
            AgentLayout::Expanded
        } else {
            AgentLayout::Normal
        };
        cx.notify();
    }

    /// Set layout
    pub fn set_layout(&mut self, layout: AgentLayout, cx: &mut Context<Self>) {
        self.layout = layout;
        cx.notify();
    }
}
