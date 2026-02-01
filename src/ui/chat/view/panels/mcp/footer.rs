//! MCP panel footer rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;

pub(crate) fn render_footer(theme: &Theme) -> impl IntoElement {
    div()
        .px_4()
        .py_2()
        .border_t_1()
        .border_color(theme.colors.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .child("Click a server to see tools • Click tool to use")
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .child("⎋")
                .child("Close")
        )
}
