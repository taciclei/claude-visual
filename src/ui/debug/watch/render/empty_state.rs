//! Empty state rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::super::core::WatchView;

impl WatchView {
    pub fn render_empty_state(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;

        let on_add = cx.listener(|this, _, _window, cx| {
            this.start_adding(cx);
        });

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .py_4()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child("No watch expressions"),
            )
            .child(
                div()
                    .id("add-watch-empty")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(accent)
                    .cursor_pointer()
                    .hover(|s| s.bg(accent.opacity(0.1)))
                    .on_click(on_add)
                    .child("+ Add Expression"),
            )
    }
}
