//! Toolbar rendering for side-by-side diff view

use gpui::*;
use gpui::prelude::*;

use crate::ui::diff::side_by_side::core::SideBySideDiffView;
use crate::ui::diff::side_by_side::types::DiffDisplayMode;

impl SideBySideDiffView {
    /// Render the toolbar with mode toggle and bulk actions
    pub(super) fn render_toolbar(
        &self,
        theme: &crate::app::theme::Theme,
        display_mode: DiffDisplayMode,
        pending_count: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listeners before div chain
        let toggle_mode_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_mode(cx);
        });

        let apply_all_listener = cx.listener(|this, _, _window, cx| {
            this.apply_all(cx);
        });

        let reject_all_listener = cx.listener(|this, _, _window, cx| {
            this.reject_all(cx);
        });

        let reset_all_listener = cx.listener(|this, _, _window, cx| {
            this.reset_all(cx);
        });

        // Copy theme colors for closures
        let surface_color = theme.colors.surface;
        let border_color = theme.colors.border;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let success_color = theme.colors.success;
        let error_color = theme.colors.error;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            .bg(surface_color.opacity(0.5))
            .border_b_1()
            .border_color(border_color)
            // Mode toggle
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .id("toggle-mode")
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(surface_color)
                            .text_xs()
                            .text_color(text_color)
                            .cursor_pointer()
                            .hover(|s| s.bg(border_color))
                            .on_click(toggle_mode_listener)
                            .child(match display_mode {
                                DiffDisplayMode::SideBySide => "Switch to Unified",
                                DiffDisplayMode::Unified => "Switch to Split",
                            }),
                    ),
            )
            // Bulk actions
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .when(pending_count > 0, |d| {
                        d.child(
                            div()
                                .id("apply-all")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .bg(success_color.opacity(0.2))
                                .text_xs()
                                .text_color(success_color)
                                .cursor_pointer()
                                .hover(|s| s.bg(success_color.opacity(0.3)))
                                .on_click(apply_all_listener)
                                .child("Apply All"),
                        )
                        .child(
                            div()
                                .id("reject-all")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .bg(error_color.opacity(0.2))
                                .text_xs()
                                .text_color(error_color)
                                .cursor_pointer()
                                .hover(|s| s.bg(error_color.opacity(0.3)))
                                .on_click(reject_all_listener)
                                .child("Reject All"),
                        )
                    })
                    .child(
                        div()
                            .id("reset-all")
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(surface_color)
                            .text_xs()
                            .text_color(text_muted_color)
                            .cursor_pointer()
                            .hover(|s| s.bg(border_color))
                            .on_click(reset_all_listener)
                            .child("Reset"),
                    ),
            )
    }
}
