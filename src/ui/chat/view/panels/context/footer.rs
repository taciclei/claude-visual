//! Context panel footer component

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;

impl ChatView {
    pub(super) fn render_context_footer(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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
                    .child("@file:path to add files"),
            )
            .child(
                div()
                    .id("compact-context-btn")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .bg(theme.colors.warning.opacity(0.1))
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.warning.opacity(0.2)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.send_slash_command("/compact", cx);
                        this.toggle_context_panel(cx);
                    }))
                    .child("/compact to free space"),
            )
    }
}
