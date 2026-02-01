//! Diff preview rendering

use crate::ui::pct;
use crate::ui::workspace::core::Workspace;
use gpui::prelude::*;
use gpui::*;

impl Workspace {
    /// Render diff preview overlay
    pub(in crate::ui::workspace) fn render_diff_preview(
        &self,
        path: &str,
        diff: &str,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let path_display = path.to_string();
        let lines: Vec<_> = diff.lines().collect();
        let additions = lines.iter().filter(|l| l.starts_with('+')).count();
        let deletions = lines.iter().filter(|l| l.starts_with('-')).count();
        let is_side_by_side = self.diff_side_by_side;

        // Prepare side-by-side data if needed
        let side_by_side_lines: Vec<(String, String)> = if is_side_by_side {
            self.prepare_side_by_side_lines(&lines)
        } else {
            Vec::new()
        };

        // Extract listeners before div builder chain
        let hide_listener = cx.listener(|this, _, _window, cx| {
            this.hide_diff_preview(cx);
        });

        let keydown_listener = cx.listener(|this, event: &KeyDownEvent, _window, cx| {
            if event.keystroke.key == "escape" {
                this.hide_diff_preview(cx);
            }
        });

        let toggle_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_diff_mode(cx);
        });

        let close_listener = cx.listener(|this, _, _window, cx| {
            this.hide_diff_preview(cx);
        });

        div()
            .id("diff-preview-overlay")
            .absolute()
            .inset_0()
            .bg(theme.colors.background.opacity(0.8))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, hide_listener)
            // Escape to close
            .on_key_down(keydown_listener)
            .child(
                div()
                    .id("diff-preview-content")
                    .w(if is_side_by_side {
                        px(1000.0)
                    } else {
                        px(700.0)
                    })
                    .max_h(pct(80.0))
                    .rounded_xl()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_mouse_down(MouseButton::Left, |_, _window, cx| {
                        cx.stop_propagation();
                    })
                    // Header
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    // Mode toggle
                                    .child(
                                        div()
                                            .id("diff-mode-toggle")
                                            .px_2()
                                            .py_1()
                                            .rounded_sm()
                                            .bg(theme.colors.accent.opacity(0.2))
                                            .text_xs()
                                            .text_color(theme.colors.accent)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.colors.accent.opacity(0.3)))
                                            .on_click(toggle_listener)
                                            .child(if is_side_by_side {
                                                "Split"
                                            } else {
                                                "Unified"
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child(path_display),
                                    )
                                    // Stats
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_sm()
                                                    .bg(theme.colors.success.opacity(0.2))
                                                    .text_xs()
                                                    .text_color(theme.colors.success)
                                                    .child(format!("+{}", additions)),
                                            )
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_sm()
                                                    .bg(theme.colors.error.opacity(0.2))
                                                    .text_xs()
                                                    .text_color(theme.colors.error)
                                                    .child(format!("-{}", deletions)),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .id("diff-preview-close")
                                    .size(px(28.0))
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|style| {
                                        style
                                            .bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .cursor_pointer()
                                    .on_click(close_listener)
                                    .child("×"),
                            ),
                    )
                    // Diff content - Unified view
                    .when(!is_side_by_side, |d| {
                        d.child(
                            div()
                                .flex_1()
                                .id("scroll-diff-unified")
                                .overflow_y_scroll()
                                .p_2()
                                .font_family("JetBrains Mono")
                                .text_xs()
                                .children(lines.iter().map(|line| {
                                    let (bg_color, text_color) = if line.starts_with('+') {
                                        (theme.colors.success.opacity(0.15), theme.colors.success)
                                    } else if line.starts_with('-') {
                                        (theme.colors.error.opacity(0.15), theme.colors.error)
                                    } else {
                                        (hsla(0.0, 0.0, 0.0, 0.0), theme.colors.text_muted)
                                    };

                                    div()
                                        .w_full()
                                        .px_2()
                                        .py_0p5()
                                        .bg(bg_color)
                                        .text_color(text_color)
                                        .child(line.to_string())
                                })),
                        )
                    })
                    // Diff content - Side-by-side view
                    .when_some(
                        if is_side_by_side {
                            Some(side_by_side_lines.clone())
                        } else {
                            None
                        },
                        {
                            let error_color = theme.colors.error;
                            let error_bg = theme.colors.error.opacity(0.15);
                            let success_color = theme.colors.success;
                            let success_bg = theme.colors.success.opacity(0.15);
                            let muted_color = theme.colors.text_muted;
                            let surface_bg = theme.colors.surface.opacity(0.5);
                            let border_color = theme.colors.border;

                            move |d, side_by_side_clone| {
                                let side_by_side_owned: Vec<(String, String)> = side_by_side_clone
                                    .iter()
                                    .map(|(a, b)| (a.clone(), b.clone()))
                                    .collect();

                                // Pre-compute left children
                                let left_children: Vec<_> = side_by_side_owned
                                    .iter()
                                    .map(|(old, _)| {
                                        let (bg_color, text_color) = if old.starts_with('-') {
                                            (error_bg, error_color)
                                        } else if old.is_empty() {
                                            (surface_bg, muted_color)
                                        } else {
                                            (hsla(0.0, 0.0, 0.0, 0.0), muted_color)
                                        };
                                        let content = if old.is_empty() {
                                            " ".to_string()
                                        } else {
                                            old.clone()
                                        };

                                        div()
                                            .w_full()
                                            .px_2()
                                            .py_0p5()
                                            .bg(bg_color)
                                            .text_color(text_color)
                                            .whitespace_nowrap()
                                            .child(content)
                                    })
                                    .collect();

                                // Pre-compute right children
                                let right_children: Vec<_> = side_by_side_owned
                                    .iter()
                                    .map(|(_, new)| {
                                        let (bg_color, text_color) = if new.starts_with('+') {
                                            (success_bg, success_color)
                                        } else if new.is_empty() {
                                            (surface_bg, muted_color)
                                        } else {
                                            (hsla(0.0, 0.0, 0.0, 0.0), muted_color)
                                        };
                                        let content = if new.is_empty() {
                                            " ".to_string()
                                        } else {
                                            new.clone()
                                        };

                                        div()
                                            .w_full()
                                            .px_2()
                                            .py_0p5()
                                            .bg(bg_color)
                                            .text_color(text_color)
                                            .whitespace_nowrap()
                                            .child(content)
                                    })
                                    .collect();

                                d.child(
                                    div()
                                        .flex_1()
                                        .id("scroll-diff-side-by-side")
                                        .overflow_y_scroll()
                                        .flex()
                                        .flex_row()
                                        // Left pane (old)
                                        .child(
                                            div()
                                                .flex_1()
                                                .border_r_1()
                                                .border_color(border_color)
                                                .p_2()
                                                .font_family("JetBrains Mono")
                                                .text_xs()
                                                .children(left_children),
                                        )
                                        // Right pane (new)
                                        .child(
                                            div()
                                                .flex_1()
                                                .p_2()
                                                .font_family("JetBrains Mono")
                                                .text_xs()
                                                .children(right_children),
                                        ),
                                )
                            }
                        },
                    ),
            )
    }
}
