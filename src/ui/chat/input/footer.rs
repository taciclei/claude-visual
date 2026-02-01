//! Footer rendering with hints and stats

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

use super::ChatInput;

impl ChatInput {
    /// Render footer with hints and character count
    pub(super) fn render_footer(
        &self,
        theme: &Theme,
        text_is_empty: bool,
        is_disabled: bool,
    ) -> impl IntoElement {
        let history_indicator = self.history_indicator();
        let is_browsing = self.is_browsing_history();

        div()
            .px_4()
            .pb_2()
            .flex()
            .items_center()
            .justify_between()
            // Left side - hints and history indicator
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    // History indicator when browsing
                    .when_some(history_indicator, |d, indicator| {
                        d.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .bg(theme.colors.info.opacity(0.15))
                                .text_color(theme.colors.info)
                                .child(indicator),
                        )
                    })
                    .when(text_is_empty && !is_disabled && !is_browsing, |d| {
                        d.child("Try:")
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.accent.opacity(0.1))
                                    .text_color(theme.colors.accent)
                                    .child("/apex"),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.accent.opacity(0.1))
                                    .text_color(theme.colors.accent)
                                    .child("/explore"),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.accent.opacity(0.1))
                                    .text_color(theme.colors.accent)
                                    .child("/debug"),
                            )
                            .child("·")
                            .child("@ for files")
                            .child("·")
                            .child("↑ history")
                    })
                    .when(!text_is_empty && !is_disabled && !is_browsing, |d| {
                        d.child("Enter to send")
                            .child("·")
                            .child("Shift+Enter for newline")
                    })
                    .when(is_browsing, |d| {
                        d.child("Browsing history")
                            .child("·")
                            .child("↓ for newer · Esc to exit")
                    }),
            )
            // Right side - character count and token estimate
            .when(!text_is_empty, |d| {
                let char_count = self.text.len();
                let word_count = self.text.split_whitespace().count();
                // Rough token estimate: ~1.3 tokens per word for English
                let estimated_tokens = ((word_count as f64) * 1.3).ceil() as usize;
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(format!("{} chars", char_count))
                        .child("·")
                        .child(format!("{} words", word_count))
                        .child("·")
                        .child(
                            div()
                                .text_color(theme.colors.warning.opacity(0.8))
                                .child(format!("~{} tokens", estimated_tokens)),
                        ),
                )
            })
    }
}
