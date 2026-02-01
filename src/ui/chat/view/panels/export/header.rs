//! Export panel header component

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    pub(super) fn render_export_header(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let on_close = cx.listener(|this, _, _window, cx| {
            this.toggle_export_panel(cx);
        });

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
                    .child(div().text_base().child("💾"))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Export Conversation"),
                    ),
            )
            .child(
                div()
                    .id("close-export-panel")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(on_close)
                    .child("×"),
            )
    }
}
