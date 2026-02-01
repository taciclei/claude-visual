//! Context panel header component

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;

impl ChatView {
    pub(super) fn render_context_header(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_base().child("📚"))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Session Context"),
                    ),
            )
            .child(
                div()
                    .id("close-context-panel")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_context_panel(cx);
                    }))
                    .child("×"),
            )
    }
}
