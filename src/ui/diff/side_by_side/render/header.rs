//! Header rendering for side-by-side diff view

use gpui::*;
use gpui::prelude::*;

use crate::ui::diff::side_by_side::core::SideBySideDiffView;
use crate::ui::diff::side_by_side::types::DiffDisplayMode;

impl SideBySideDiffView {
    /// Render the diff view header with file info and stats
    pub(super) fn render_header(
        &self,
        theme: &crate::app::theme::Theme,
        file_path: &str,
        collapsed: bool,
        total_additions: usize,
        total_deletions: usize,
        applied_count: usize,
        rejected_count: usize,
        total_hunks: usize,
        comment_count: usize,
        display_mode: DiffDisplayMode,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        // Extract listener before div chain
        let toggle_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_collapsed(cx);
        });

        // Copy theme colors for closures
        let accent_color = theme.colors.accent;
        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let success_color = theme.colors.success;
        let error_color = theme.colors.error;
        let warning_color = theme.colors.warning;

        div()
            .id("diff-view-header")
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
            // Left side: file info
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Chevron
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted_color)
                            .child(if collapsed { "▶" } else { "▼" }),
                    )
                    // Mode badge
                    .child(
                        div()
                            .px_1()
                            .py_0p5()
                            .rounded_sm()
                            .bg(accent_color.opacity(0.2))
                            .text_xs()
                            .text_color(accent_color)
                            .child(match display_mode {
                                DiffDisplayMode::SideBySide => "SPLIT",
                                DiffDisplayMode::Unified => "UNIFIED",
                            }),
                    )
                    // File path
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_color)
                            .child(file_path.to_string()),
                    ),
            )
            // Right side: stats and actions
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
                                    .child(format!("+{}", total_additions)),
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
                                    .child(format!("-{}", total_deletions)),
                            ),
                    )
                    // Hunk progress
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted_color)
                            .child(format!(
                                "{}/{} hunks ({} ✓ {} ✗)",
                                applied_count + rejected_count,
                                total_hunks,
                                applied_count,
                                rejected_count
                            )),
                    )
                    // Comments
                    .when(comment_count > 0, |d| {
                        d.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .bg(warning_color.opacity(0.2))
                                .text_xs()
                                .text_color(warning_color)
                                .child(format!("{} 💬", comment_count)),
                        )
                    }),
            )
    }
}
