//! DiffBlockView Render implementation

use gpui::*;
use gpui::prelude::*;

use super::component::DiffBlockView;
use super::render_content::{render_content, prepare_hunk_data};

impl Render for DiffBlockView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let collapsed = self.collapsed;
        let file_path = self.file_path.clone();
        let additions = self.additions;
        let deletions = self.deletions;

        // Copy theme colors for closures
        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let success_color = theme.colors.success;
        let error_color = theme.colors.error;
        let background_color = theme.colors.background;

        // Pre-compute hunk data for rendering
        let hunk_data = prepare_hunk_data(&self.hunks);

        // Extract listener before div chain
        let toggle_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_collapsed(cx);
        });

        div()
            .w_full()
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(border_color)
            // Header
            .child(
                div()
                    .id("diff-header")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(surface_color)
                    .border_b_1()
                    .border_color(border_color)
                    .cursor_pointer()
                    .on_click(toggle_listener)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Chevron indicator
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .child(if collapsed { "▶" } else { "▼" }),
                            )
                            // Diff icon
                            .child(
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(accent_color.opacity(0.2))
                                    .text_xs()
                                    .text_color(accent_color)
                                    .child("DIFF"),
                            )
                            // File path
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text_color)
                                    .child(file_path),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            // Stats
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_sm()
                                            .bg(success_color.opacity(0.2))
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(success_color)
                                            .child(format!("+{}", additions)),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_sm()
                                            .bg(error_color.opacity(0.2))
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(error_color)
                                            .child(format!("-{}", deletions)),
                                    ),
                            ),
                    ),
            )
            // Diff content
            .when(!collapsed, |d| {
                d.child(render_content(
                    hunk_data,
                    border_color,
                    surface_color,
                    text_color,
                    text_muted_color,
                    accent_color,
                    success_color,
                    error_color,
                    background_color,
                ))
            })
    }
}
