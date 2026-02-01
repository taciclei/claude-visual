//! No changes message rendering

use gpui::*;

use crate::app::theme::Theme;

pub(crate) fn render_no_changes(theme: &Theme) -> impl IntoElement {
    div()
        .px_4()
        .py_4()
        .flex()
        .flex_col()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_sm()
                .text_color(theme.colors.success)
                .child("Working tree clean"),
        )
}

pub(crate) fn render_no_git_repo(theme: &Theme) -> impl IntoElement {
    div()
        .py_8()
        .flex()
        .flex_col()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_sm()
                .text_color(theme.colors.text_muted)
                .text_center()
                .child("Select a git repository"),
        )
}
