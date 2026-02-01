//! Stats bar rendering for ChatView

use super::super::super::core::ChatView;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_stats_bar(&self, theme: &crate::app::theme::Theme) -> Div {
        let stats = self.calculate_stats();

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_1()
            .bg(theme.colors.surface)
            .border_t_1()
            .border_color(theme.colors.border)
            // Left side - message counts
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    // Total messages
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Messages:"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child(stats.message_count.to_string()),
                            ),
                    )
                    // User / Assistant breakdown
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.accent)
                                    .child(format!("{} you", stats.user_message_count)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("/"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.success)
                                    .child(format!("{} claude", stats.assistant_message_count)),
                            ),
                    )
                    // Tool uses
                    .when(stats.tool_use_count > 0, |d| {
                        d.child(
                            div().flex().items_center().gap_1().child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.info)
                                    .child(format!("{} tools", stats.tool_use_count)),
                            ),
                        )
                    }),
            )
            // Right side - word/token counts and duration
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    // Words
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Words:"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .child(stats.format_words()),
                            ),
                    )
                    // Estimated tokens with context progress
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child("~Tokens:"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.warning)
                                            .child(stats.format_tokens()),
                                    ),
                            )
                            // Context usage progress bar (estimate based on 200K context)
                            .child({
                                let max_tokens = 200_000usize; // Claude default
                                let percentage =
                                    (stats.estimated_tokens as f32 / max_tokens as f32 * 100.0)
                                        .min(100.0);
                                let progress_color = if percentage > 95.0 {
                                    theme.colors.error
                                } else if percentage > 80.0 {
                                    theme.colors.warning
                                } else {
                                    theme.colors.success
                                };
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .w(px(40.0))
                                            .h(px(4.0))
                                            .rounded_full()
                                            .bg(theme.colors.border)
                                            .overflow_hidden()
                                            .child(
                                                div()
                                                    .h_full()
                                                    .w(pct(percentage))
                                                    .rounded_full()
                                                    .bg(progress_color),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(progress_color)
                                            .child(format!("{:.0}%", percentage)),
                                    )
                                    // Warning badge when context is getting full
                                    .when(percentage > 80.0, |d| {
                                        let (icon, label) = if percentage > 95.0 {
                                            ("⚠️", "Context almost full!")
                                        } else {
                                            ("📊", "Context filling up")
                                        };
                                        d.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_2()
                                                .py_px()
                                                .rounded_md()
                                                .bg(progress_color.opacity(0.15))
                                                .border_1()
                                                .border_color(progress_color.opacity(0.3))
                                                .child(div().text_xs().child(icon))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(progress_color)
                                                        .child(label),
                                                ),
                                        )
                                    })
                            }),
                    )
                    // Duration
                    .when(stats.duration_minutes > 0, |d| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Duration:"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text)
                                        .child(stats.format_duration()),
                                ),
                        )
                    })
                    // Session model (from Claude CLI)
                    .when_some(self.session_info.as_ref(), |d, info| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Model:"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.accent)
                                        .child(info.model.clone()),
                                ),
                        )
                    })
                    // Actual tokens used
                    .when(
                        self.stats.input_tokens > 0 || self.stats.output_tokens > 0,
                        |d| {
                            d.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child("Tokens:"),
                                    )
                                    .child(div().text_xs().text_color(theme.colors.info).child(
                                        format!(
                                            "{}↓ {}↑",
                                            Self::format_token_count(self.stats.input_tokens),
                                            Self::format_token_count(self.stats.output_tokens)
                                        ),
                                    )),
                            )
                        },
                    )
                    // Session cost
                    .when(self.stats.cost > 0.0, |d| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Cost:"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.success)
                                        .child(format!("${:.4}", self.stats.cost)),
                                ),
                        )
                    }),
            )
    }
}
