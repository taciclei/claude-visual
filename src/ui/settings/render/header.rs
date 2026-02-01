//! Settings modal header

use gpui::*;
use crate::app::theme::Theme;
use super::super::core::SettingsModal;

impl SettingsModal {
    pub(super) fn render_header(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let on_close = cx.listener(|this, _, _window, cx| {
            this.dismiss(cx);
        });

        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text)
                    .child("Settings"),
            )
            .child(
                div()
                    .id("settings-close")
                    .size(px(28.0))
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.colors.text_muted)
                    .hover(move |style| {
                        style
                            .bg(surface_hover)
                            .text_color(text_color)
                    })
                    .cursor_pointer()
                    .on_click(on_close)
                    .child("x"),
            )
    }
}
