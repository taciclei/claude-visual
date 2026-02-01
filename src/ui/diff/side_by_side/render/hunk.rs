//! Hunk rendering for side-by-side diff view

use gpui::prelude::*;
use gpui::*;

use crate::ui::diff::hunk::{HunkAction, ManagedHunk};
use crate::ui::diff::side_by_side::core::SideBySideDiffView;

impl SideBySideDiffView {
    /// Render a single hunk in side-by-side mode
    pub(super) fn render_hunk_side_by_side(
        &self,
        hunk: &ManagedHunk,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let hunk_id = hunk.id;
        let is_focused = self.hunk_manager.focused_hunk == Some(hunk_id);

        // Prepare old and new lines
        let mut old_lines: Vec<(Option<usize>, String, bool)> = Vec::new();
        let mut new_lines: Vec<(Option<usize>, String, bool)> = Vec::new();

        for line in &hunk.lines {
            match line.line_type {
                ' ' => {
                    // Context line goes to both sides
                    old_lines.push((line.old_line, line.content.clone(), false));
                    new_lines.push((line.new_line, line.content.clone(), false));
                }
                '-' => {
                    // Removed line goes to old side, empty placeholder on new
                    old_lines.push((line.old_line, line.content.clone(), true));
                    new_lines.push((None, String::new(), false));
                }
                '+' => {
                    // Added line goes to new side, empty placeholder on old
                    old_lines.push((None, String::new(), false));
                    new_lines.push((line.new_line, line.content.clone(), true));
                }
                _ => {}
            }
        }

        // Align arrays (add empty placeholders if needed)
        let max_len = old_lines.len().max(new_lines.len());
        while old_lines.len() < max_len {
            old_lines.push((None, String::new(), false));
        }
        while new_lines.len() < max_len {
            new_lines.push((None, String::new(), false));
        }

        let status_color = {
            let (r, g, b) = hunk.status.color();
            rgb(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
        };

        // Extract listener before div chain
        let apply_listener = cx.listener(move |this, _, _window, cx| {
            this.apply_hunk_action(hunk_id, HunkAction::Apply, cx);
        });

        let reject_listener = cx.listener(move |this, _, _window, cx| {
            this.apply_hunk_action(hunk_id, HunkAction::Reject, cx);
        });

        // Copy theme colors for closures
        let success_color = theme.colors.success;
        let error_color = theme.colors.error;
        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let background_color = theme.colors.background;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let show_line_numbers = self.show_line_numbers;

        div()
            .w_full()
            .rounded_md()
            .overflow_hidden()
            .border_1()
            .border_color(if is_focused {
                accent_color
            } else {
                border_color
            })
            .mb_2()
            // Hunk header
            .child(
                div()
                    .id(SharedString::from(format!("hunk-header-{}", hunk_id)))
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_1()
                    .bg(surface_color)
                    .border_b_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Status indicator
                            .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(status_color))
                            // Header text
                            .child(
                                div()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(accent_color)
                                    .child(hunk.header.clone()),
                            ),
                    )
                    // Action buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            // Apply button
                            .child(
                                div()
                                    .id(SharedString::from(format!("apply-{}", hunk_id)))
                                    .px_2()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(success_color.opacity(0.2))
                                    .text_xs()
                                    .text_color(success_color)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(success_color.opacity(0.3)))
                                    .on_click(apply_listener)
                                    .child("Apply"),
                            )
                            // Reject button
                            .child(
                                div()
                                    .id(SharedString::from(format!("reject-{}", hunk_id)))
                                    .px_2()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(error_color.opacity(0.2))
                                    .text_xs()
                                    .text_color(error_color)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(error_color.opacity(0.3)))
                                    .on_click(reject_listener)
                                    .child("Reject"),
                            ),
                    ),
            )
            // Side by side content
            .when(hunk.expanded, move |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .w_full()
                        // Old side (left)
                        .child(
                            div()
                                .flex_1()
                                .border_r_1()
                                .border_color(border_color)
                                .children(old_lines.iter().enumerate().map(
                                    |(idx, (line_num, content, is_change))| {
                                        let bg = if *is_change {
                                            error_color.opacity(0.12)
                                        } else if content.is_empty() && line_num.is_none() {
                                            surface_color.opacity(0.5)
                                        } else {
                                            background_color
                                        };

                                        div()
                                            .flex()
                                            .flex_row()
                                            .w_full()
                                            .bg(bg)
                                            // Line number
                                            .when(show_line_numbers, |d| {
                                                d.child(
                                                    div()
                                                        .w(px(40.0))
                                                        .flex_shrink_0()
                                                        .px_1()
                                                        .py_0p5()
                                                        .text_xs()
                                                        .font_family("JetBrains Mono")
                                                        .text_color(text_muted_color)
                                                        .text_right()
                                                        .border_r_1()
                                                        .border_color(border_color)
                                                        .child(
                                                            line_num
                                                                .map(|n| n.to_string())
                                                                .unwrap_or_default(),
                                                        ),
                                                )
                                            })
                                            // Content
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!(
                                                        "old-line-{}-{}",
                                                        hunk_id, idx
                                                    )))
                                                    .flex_1()
                                                    .px_2()
                                                    .py_0p5()
                                                    .text_xs()
                                                    .font_family("JetBrains Mono")
                                                    .whitespace_nowrap()
                                                    .text_color(if *is_change {
                                                        error_color
                                                    } else {
                                                        text_color
                                                    })
                                                    .child(if content.is_empty() {
                                                        " ".to_string()
                                                    } else {
                                                        content.clone()
                                                    }),
                                            )
                                    },
                                )),
                        )
                        // New side (right)
                        .child(div().flex_1().children(new_lines.iter().enumerate().map(
                            |(idx, (line_num, content, is_change))| {
                                let bg = if *is_change {
                                    success_color.opacity(0.12)
                                } else if content.is_empty() && line_num.is_none() {
                                    surface_color.opacity(0.5)
                                } else {
                                    background_color
                                };

                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .bg(bg)
                                    // Line number
                                    .when(show_line_numbers, |d| {
                                        d.child(
                                            div()
                                                .w(px(40.0))
                                                .flex_shrink_0()
                                                .px_1()
                                                .py_0p5()
                                                .text_xs()
                                                .font_family("JetBrains Mono")
                                                .text_color(text_muted_color)
                                                .text_right()
                                                .border_r_1()
                                                .border_color(border_color)
                                                .child(
                                                    line_num
                                                        .map(|n| n.to_string())
                                                        .unwrap_or_default(),
                                                ),
                                        )
                                    })
                                    // Content
                                    .child(
                                        div()
                                            .id(SharedString::from(format!(
                                                "new-line-{}-{}",
                                                hunk_id, idx
                                            )))
                                            .flex_1()
                                            .px_2()
                                            .py_0p5()
                                            .text_xs()
                                            .font_family("JetBrains Mono")
                                            .whitespace_nowrap()
                                            .text_color(if *is_change {
                                                success_color
                                            } else {
                                                text_color
                                            })
                                            .child(if content.is_empty() {
                                                " ".to_string()
                                            } else {
                                                content.clone()
                                            }),
                                    )
                            },
                        ))),
                )
            })
    }
}
