//! Token usage section rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the token usage section
    pub(crate) fn render_token_usage_section(
        &self,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child("TOKEN USAGE")
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
                    .child(self.render_detail_row("Input Tokens", &Self::format_token_count(self.stats.input_tokens), &theme))
                    .child(self.render_detail_row("Output Tokens", &Self::format_token_count(self.stats.output_tokens), &theme))
                    .child(self.render_detail_row("Total Tokens", &Self::format_token_count(self.stats.input_tokens + self.stats.output_tokens), &theme))
                    .child(self.render_detail_row("Context Used", &format!("{:.1}%", self.context_usage_percentage() * 100.0), &theme))
            )
    }
}
