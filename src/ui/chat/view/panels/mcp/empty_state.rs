//! MCP panel empty state rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;

pub(crate) fn render_empty_state(theme: &Theme) -> impl IntoElement {
    div()
        .px_4()
        .py_8()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .child(
            div()
                .size(px(48.0))
                .rounded_full()
                .bg(theme.colors.text_muted.opacity(0.1))
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_xl().child("🔌"))
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.colors.text)
                .child("No MCP servers connected")
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .text_center()
                .child("MCP (Model Context Protocol) servers extend Claude's capabilities with custom tools and resources.")
        )
        .child(
            div()
                .mt_2()
                .px_3()
                .py_2()
                .rounded_md()
                .bg(theme.colors.accent.opacity(0.1))
                .text_xs()
                .text_color(theme.colors.accent)
                .child("Configure in ~/.claude/settings.json")
        )
}
