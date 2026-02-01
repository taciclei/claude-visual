//! Branch header and git status rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

pub(crate) fn render_header(
    theme: &Theme,
    is_git_repo: bool,
    current_branch: Option<String>,
    has_changes: bool,
) -> impl IntoElement {
    div()
        .flex_shrink_0()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(theme.colors.border)
        .when(is_git_repo, |d| {
            d.flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child("Branch:"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_sm()
                        .bg(theme.colors.accent.opacity(0.2))
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.colors.accent)
                        .child(current_branch.unwrap_or_else(|| "detached".to_string())),
                )
                .when(has_changes, |d| {
                    d.child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(theme.colors.warning.opacity(0.2))
                            .text_xs()
                            .text_color(theme.colors.warning)
                            .child("modified"),
                    )
                })
        })
        .when(!is_git_repo, |d| {
            d.flex().items_center().gap_2().child(
                div()
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .bg(theme.colors.warning.opacity(0.2))
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .child("Not a git repo"),
            )
        })
}
