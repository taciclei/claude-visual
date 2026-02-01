//! Create worktree button rendering

use gpui::*;

use crate::app::theme::Theme;
use super::super::types::WorktreePanel;

pub(crate) fn render_create_button(
    theme: &Theme,
    cx: &mut Context<WorktreePanel>,
) -> impl IntoElement {
    let accent_color = theme.colors.accent;
    let accent_hover = theme.colors.accent_hover;

    let on_click = cx.listener(|this, _, _window, cx| {
        this.create_worktree(cx);
    });

    div()
        .flex_shrink_0()
        .px_3()
        .py_3()
        .border_t_1()
        .border_color(theme.colors.border)
        .child(
            div()
                .id("create-worktree-button")
                .w_full()
                .px_3()
                .py_2()
                .rounded_md()
                .bg(accent_color)
                .hover(move |style| style.bg(accent_hover))
                .cursor_pointer()
                .flex()
                .justify_center()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .on_click(on_click)
                .child("+ New Worktree"),
        )
}
