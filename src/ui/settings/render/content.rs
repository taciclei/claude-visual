//! Settings tab content area

use gpui::*;
use gpui::prelude::*;
use super::super::core::SettingsModal;
use super::super::types::SettingsTab;

impl SettingsModal {
    pub(super) fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .p_4()
            .id("scroll-settings-content")
            .overflow_y_scroll()
            .when(self.active_tab == SettingsTab::Appearance, |d| {
                d.child(self.render_appearance_tab(cx))
            })
            .when(self.active_tab == SettingsTab::Editor, |d| {
                d.child(self.render_editor_tab(cx))
            })
            .when(self.active_tab == SettingsTab::Keybindings, |d| {
                d.child(self.render_keybindings_tab(cx))
            })
            .when(self.active_tab == SettingsTab::Git, |d| {
                d.child(self.render_git_tab(cx))
            })
            .when(self.active_tab == SettingsTab::Claude, |d| {
                d.child(self.render_claude_tab(cx))
            })
            .when(self.active_tab == SettingsTab::Mcp, |d| {
                d.child(self.render_mcp_tab(cx))
            })
    }
}
