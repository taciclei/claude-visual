//! Templates panel footer

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;
use super::super::super::types::NotificationType;

impl ChatView {
    pub fn render_templates_footer(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let save_listener = cx.listener(|this, _, _window, cx| {
            this.show_notification("Use ⌘S to save current input as template", NotificationType::Info, cx);
        });

        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let accent_color = theme.colors.accent;
        let accent_bg = accent_color.opacity(0.1);
        let accent_bg_hover = accent_color.opacity(0.2);

        div()
            .px_4()
            .py_2()
            .border_t_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child("Click to insert • Type to filter")
            )
            .child(
                div()
                    .id("save-as-template-btn")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .bg(accent_bg)
                    .text_xs()
                    .text_color(accent_color)
                    .cursor_pointer()
                    .hover(move |s| s.bg(accent_bg_hover))
                    .on_click(save_listener)
                    .child("+ Save Current as Template")
            )
    }
}
