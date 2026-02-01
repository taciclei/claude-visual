//! Commands panel footer rendering

use gpui::*;
use gpui::prelude::*;

pub fn render_footer(theme: &crate::app::theme::Theme) -> impl IntoElement {
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
                .child("Click to use • Type / to filter")
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .child("⌘/")
                .child("Toggle")
        )
}
