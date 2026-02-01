//! Add expression input rendering

use gpui::prelude::*;
use gpui::*;

use super::super::core::WatchView;
use crate::app::theme::Theme;

impl WatchView {
    pub fn render_add_input(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let input_text = self.input_text.clone();
        let text_color = theme.colors.text;
        let success = theme.colors.success;
        let error = theme.colors.error;
        let surface = theme.colors.surface;
        let background = theme.colors.background;
        let accent = theme.colors.accent;
        let border = theme.colors.border;

        let on_confirm = cx.listener(|this, _, _window, cx| {
            let expr = this.input_text.clone();
            this.add_expression(expr, cx);
        });

        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.cancel_adding(cx);
        });

        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .bg(surface.opacity(0.5))
            .border_b_1()
            .border_color(border)
            .child(
                div()
                    .id("watch-input")
                    .flex_1()
                    .px_2()
                    .py_1()
                    .bg(background)
                    .border_1()
                    .border_color(accent)
                    .rounded_sm()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(text_color)
                    .child(if input_text.is_empty() {
                        "Enter expression...".to_string()
                    } else {
                        input_text.clone()
                    }),
            )
            .child(
                div()
                    .id("watch-add-confirm")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(success)
                    .cursor_pointer()
                    .hover(|s| s.bg(success.opacity(0.1)))
                    .on_click(on_confirm)
                    .child("✓"),
            )
            .child(
                div()
                    .id("watch-add-cancel")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(error)
                    .cursor_pointer()
                    .hover(|s| s.bg(error.opacity(0.1)))
                    .on_click(on_cancel)
                    .child("✗"),
            )
    }
}
