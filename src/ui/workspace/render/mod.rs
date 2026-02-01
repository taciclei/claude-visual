//! Workspace render implementation split into logical modules

mod action_registry;
mod content;
mod overlays;
mod sidebar;

use super::core::Workspace;
use gpui::prelude::*;
use gpui::*;

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let active_chat_view = self.chat_views.get(self.active_chat_index).cloned();

        // Start with action registration
        action_registry::register_actions(
            div()
                .id("workspace")
                .key_context("Workspace")
                .size_full()
                .bg(theme.colors.background)
                .text_color(theme.colors.text)
                .flex()
                .flex_row(),
            cx,
        )
        // Sidebar
        .child(sidebar::render_sidebar(self, &theme, cx))
        // Main content area
        .child(content::render_content(self, &theme, active_chat_view, cx))
        // Overlays and indicators
        .children(overlays::render_overlays(self, &theme, cx))
    }
}
