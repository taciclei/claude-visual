//! Message header rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::claude::message::MessageRole;
use super::super::view::MessageView;
use super::super::utils::{tool_icon, tool_description};

impl MessageView {
    pub(in crate::ui::chat::message) fn render_header(&self, theme: &Theme) -> Div {
        let role_label = self.role_label();
        let time = self.formatted_time();
        let collapsed = self.collapsed;
        let code_blocks = self.code_block_count();
        let is_assistant = self.message.role == MessageRole::Assistant;
        let is_user = self.message.role == MessageRole::User;
        let word_count = self.message.content.split_whitespace().count();

        let (role_bg, role_text) = match self.message.role {
            MessageRole::User => (theme.colors.accent, hsla(0.0, 0.0, 1.0, 1.0)),
            MessageRole::Assistant => (theme.colors.success.opacity(0.2), theme.colors.success),
            MessageRole::ToolUse => (theme.colors.info.opacity(0.2), theme.colors.info),
            MessageRole::ToolResult => (theme.colors.warning.opacity(0.2), theme.colors.warning),
            MessageRole::Error => (theme.colors.error.opacity(0.2), theme.colors.error),
            MessageRole::Thinking => (theme.colors.warning.opacity(0.15), theme.colors.warning),
            MessageRole::System => (theme.colors.text_muted.opacity(0.2), theme.colors.text_muted),
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Collapse indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(if collapsed { "▶" } else { "▼" }),
                    )
                    // Role badge
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(role_bg)
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(role_text)
                            .child(role_label),
                    )
                    // Tool name with icon if applicable
                    .when_some(self.message.tool_name.clone(), |this, tool_name| {
                        let icon = tool_icon(&tool_name);
                        let desc = tool_description(&tool_name);
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_sm()
                                        .child(icon)
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.colors.text)
                                        .child(tool_name)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(format!("- {}", desc))
                                ),
                        )
                    })
                    // Timestamp
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(time),
                    ),
            )
            // Right side - indicators
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Word count for user messages (when > 10 words)
                    .when(is_user && word_count > 10, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted.opacity(0.6))
                                .child(format!("{} words", word_count))
                        )
                    })
                    // Code blocks indicator (for assistant messages)
                    .when(is_assistant && code_blocks > 0, |d| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_px()
                                .rounded_sm()
                                .bg(theme.colors.info.opacity(0.1))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.info)
                                        .font_family("monospace")
                                        .child("{ }")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.info)
                                        .child(format!("{}", code_blocks))
                                )
                        )
                    })
                    // Word count for assistant messages (when > 50 words)
                    .when(is_assistant && word_count > 50, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted.opacity(0.6))
                                .child(format!("{} words", word_count))
                        )
                    })
                    // Token/cost estimate for user and assistant messages (when > 20 words)
                    .when((is_user || is_assistant) && word_count > 20, |d| {
                        let tokens = self.estimate_tokens();
                        let cost = self.estimate_cost();
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1()
                                .py_px()
                                .rounded_sm()
                                .bg(theme.colors.warning.opacity(0.1))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.warning.opacity(0.8))
                                        .child(format!("~{} tok", Self::format_tokens(tokens)))
                                )
                                .when(cost > 0.0001, |d| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted.opacity(0.5))
                                            .child(format!("(${:.4})", cost))
                                    )
                                })
                        )
                    })
                    // Bookmark indicator
                    .when(self.bookmarked, |this| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.warning)
                                .child("★"),
                        )
                    }),
            )
    }
}
