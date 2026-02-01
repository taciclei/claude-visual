//! Export panel stats summary component

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    pub(super) fn render_export_stats(&self, theme: &Theme) -> impl IntoElement {
        let stats = self.calculate_stats();

        div()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background.opacity(0.5))
            .flex()
            .items_center()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(div().text_sm().child("💬"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(format!("{} messages", stats.message_count))
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(div().text_sm().child("📝"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(format!("{} words", stats.format_words()))
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(div().text_sm().child("⏱️"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(stats.format_duration())
                    )
            )
    }
}
