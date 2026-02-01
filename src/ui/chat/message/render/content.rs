//! Content rendering for different message types

use gpui::prelude::*;
use gpui::*;

use super::super::view::MessageView;
use crate::app::theme::Theme;
use crate::markdown::renderer::MarkdownRenderer;

impl MessageView {
    pub(in crate::ui::chat::message) fn render_user_content(&self, theme: &Theme) -> Div {
        div()
            .px_3()
            .py_2()
            .text_sm()
            .text_color(theme.colors.text)
            .child(self.message.content.clone())
    }

    pub(in crate::ui::chat::message) fn render_assistant_content(&self, theme: &Theme) -> Div {
        div().px_3().py_2().child(MarkdownRenderer::new(
            &self.message.content,
            self.app_state.clone(),
        ))
    }

    /// Render thinking content (Claude's reasoning)
    pub(in crate::ui::chat::message) fn render_thinking_content(&self, theme: &Theme) -> Div {
        let content = &self.message.content;
        let char_count = content.len();
        let word_count = content.split_whitespace().count();
        let line_count = content.lines().count();

        div()
            .px_3()
            .py_2()
            .bg(theme.colors.warning.opacity(0.05))
            .border_l_2()
            .border_color(theme.colors.warning.opacity(0.3))
            .rounded_r_md()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Header with brain icon and stats
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    // Brain icon
                                    .child(div().text_sm().child("🧠"))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.warning)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Extended Thinking"),
                                    ),
                            )
                            // Stats badges
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .px_1()
                                            .py_px()
                                            .rounded_sm()
                                            .bg(theme.colors.warning.opacity(0.1))
                                            .text_xs()
                                            .text_color(theme.colors.warning.opacity(0.8))
                                            .child(format!("{} words", word_count)),
                                    )
                                    .child(
                                        div()
                                            .px_1()
                                            .py_px()
                                            .rounded_sm()
                                            .bg(theme.colors.warning.opacity(0.1))
                                            .text_xs()
                                            .text_color(theme.colors.warning.opacity(0.8))
                                            .child(format!("{} lines", line_count)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted.opacity(0.6))
                                            .child(format!(
                                                "~{} tokens",
                                                (word_count as f64 * 1.3).ceil() as usize
                                            )),
                                    ),
                            ),
                    )
                    // Content with proper formatting
                    .child(
                        div()
                            .id("thinking-content")
                            .mt_1()
                            .p_2()
                            .rounded_md()
                            .bg(theme.colors.background.opacity(0.5))
                            .text_sm()
                            .font_family("JetBrains Mono")
                            .text_color(theme.colors.text_muted)
                            .max_h(px(300.0))
                            .overflow_y_scroll()
                            .child(content.clone()),
                    ),
            )
    }

    /// Render system content
    pub(in crate::ui::chat::message) fn render_system_content(&self, theme: &Theme) -> Div {
        div()
            .px_3()
            .py_2()
            .text_sm()
            .italic()
            .text_color(theme.colors.text_muted)
            .child(self.message.content.clone())
    }
}
