//! Header rendering for MCP context attachment panel

use gpui::*;
use gpui::prelude::*;

use super::super::core::McpContextAttachPanel;
use super::super::types::*;

impl McpContextAttachPanel {
    /// Render the panel header with title, badge, and controls
    pub(super) fn render_header(&self, attached_count: usize, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;

        let filter_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_attached_only(cx);
        });

        let close_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(McpContextAttachEvent::Closed);
        });

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("Attach Resources"),
                    )
                    // Attached count badge
                    .when(attached_count > 0, |d| {
                        d.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_full()
                                .bg(theme.colors.success)
                                .text_xs()
                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                .child(format!("{}", attached_count)),
                        )
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Filter toggle
                    .child(
                        div()
                            .id("filter-attached")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .bg(if self.show_attached_only {
                                theme.colors.accent.opacity(0.2)
                            } else {
                                theme.colors.surface
                            })
                            .text_color(if self.show_attached_only {
                                theme.colors.accent
                            } else {
                                theme.colors.text_muted
                            })
                            .hover(|s| s.bg(surface_hover))
                            .cursor_pointer()
                            .on_click(filter_listener)
                            .child("Attached"),
                    )
                    // Close button
                    .child(
                        div()
                            .id("close-btn")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| {
                                s.bg(surface_hover)
                                    .text_color(text_color)
                            })
                            .cursor_pointer()
                            .on_click(close_listener)
                            .child("×"),
                    ),
            )
    }
}
