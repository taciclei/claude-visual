//! Code block header title (language badge and line count)

use gpui::prelude::*;
use gpui::*;

use crate::ui::blocks::code_block::view::CodeBlockView;

pub(super) fn render_title(
    view: &CodeBlockView,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let theme = view.app_state.theme.read(cx);
    let line_count: Vec<&str> = view.code.lines().collect();
    let language_display = view.language_display();

    let accent_color = theme.colors.accent;
    let text_muted = theme.colors.text_muted;

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .px_2()
                .py_0p5()
                .rounded_sm()
                .bg(accent_color.opacity(0.2))
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(accent_color)
                .child(language_display),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_muted)
                .child(format!("{} lines", line_count.len())),
        )
}
