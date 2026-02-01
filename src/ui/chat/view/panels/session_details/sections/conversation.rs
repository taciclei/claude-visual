//! Conversation section rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the conversation stats section
    pub(crate) fn render_conversation_section(
        &self,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        let stats = self.calculate_stats();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child("CONVERSATION")
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.render_detail_row("Messages", &format!("{}", stats.message_count), &theme))
                    .child(self.render_detail_row("Your Messages", &format!("{}", stats.user_message_count), &theme))
                    .child(self.render_detail_row("Claude Messages", &format!("{}", stats.assistant_message_count), &theme))
                    .child(self.render_detail_row("Tool Uses", &format!("{}", stats.tool_use_count), &theme))
                    .child(self.render_detail_row("Duration", &stats.format_duration(), &theme))
            )
    }
}
