//! Permissions panel render functions

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;

mod header;
mod item;
mod footer;

impl ChatView {
    pub fn render_permissions_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let permissions: Vec<_> = self.pending_permissions.iter().enumerate().collect();
        let permissions_count = permissions.len();
        let has_multiple = permissions_count > 1;
        let is_empty = permissions_count == 0;

        // Extract listener before div chain
        let close_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_permissions_panel(cx);
        });

        // Full-screen overlay
        div()
            .id("permissions-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            // Click outside to close
            .on_click(close_listener)
            .child(
                // Panel
                div()
                    .id("permissions-panel")
                    .w(px(450.0))
                    .max_h(px(400.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.warning.opacity(0.5))
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    // Prevent clicks from propagating
                    .on_click(|_, _, _| {})
                    // Header
                    .child(header::render_header(theme, permissions.len(), cx))
                    // Permissions list
                    .child(item::render_permissions_list(theme, permissions, cx))
                    // Footer with bulk actions
                    .child(footer::render_footer(theme, is_empty, has_multiple, cx))
            )
    }
}
