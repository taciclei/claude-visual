//! Templates panel header

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;

impl ChatView {
    pub fn render_templates_header(
        &self,
        theme: &crate::app::theme::Theme,
        total_count: usize,
        custom_count: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let close_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_templates_panel(cx);
        });

        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let accent_color = theme.colors.accent;
        let success_color = theme.colors.success;
        let surface_hover = theme.colors.surface_hover;

        div()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_base().child("📝"))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_color)
                            .child("Prompt Templates"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_px()
                            .rounded_full()
                            .bg(accent_color.opacity(0.2))
                            .text_xs()
                            .text_color(accent_color)
                            .child(format!("{} templates", total_count)),
                    )
                    .when(custom_count > 0, |d| {
                        d.child(
                            div()
                                .px_2()
                                .py_px()
                                .rounded_full()
                                .bg(success_color.opacity(0.2))
                                .text_xs()
                                .text_color(success_color)
                                .child(format!("{} custom", custom_count)),
                        )
                    }),
            )
            .child(
                div()
                    .id("close-templates-panel")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(text_muted)
                    .hover(move |s| s.bg(surface_hover))
                    .on_click(close_listener)
                    .child("×"),
            )
    }
}
