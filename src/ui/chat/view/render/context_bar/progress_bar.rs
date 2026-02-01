//! Context progress bar render function for ChatView

use gpui::*;
use gpui::prelude::*;
use crate::app::theme::Theme;
use crate::ui::pct;
use super::super::super::core::ChatView;
use super::super::super::types::ChatViewEvent;

impl ChatView {
    pub fn render_context_progress_bar(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        let percentage = self.context_usage_percentage();
        let width_pct = (percentage * 100.0) as u32;
        let status_color = self.context_status_color(theme);
        let usage_text = self.format_context_usage();

        // Calculate input/output token percentages for segmented bar
        let total_tokens = self.stats.input_tokens + self.stats.output_tokens;
        let capacity = self.context_capacity.max(1);
        let input_pct = if total_tokens > 0 {
            ((self.stats.input_tokens as f32 / capacity as f32) * 100.0).min(100.0)
        } else {
            0.0
        };
        let output_pct = if total_tokens > 0 {
            ((self.stats.output_tokens as f32 / capacity as f32) * 100.0).min(100.0 - input_pct)
        } else {
            0.0
        };

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_1()
            // Progress bar container
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("📊")
                    )
                    // Segmented progress bar showing input (blue) and output (green)
                    .child(
                        div()
                            .w(px(120.0))
                            .h(px(6.0))
                            .rounded_full()
                            .bg(theme.colors.border.opacity(0.3))
                            .overflow_hidden()
                            .flex()
                            .when(total_tokens > 0, |d| {
                                d.child(
                                    div()
                                        .h_full()
                                        .bg(theme.colors.info) // Input tokens (blue)
                                        .w(pct(input_pct))
                                )
                                .child(
                                    div()
                                        .h_full()
                                        .bg(theme.colors.success) // Output tokens (green)
                                        .w(pct(output_pct))
                                )
                            })
                            .when(total_tokens == 0, |d| {
                                d.child(
                                    div()
                                        .h_full()
                                        .rounded_full()
                                        .bg(status_color)
                                        .w(pct(width_pct as f32))
                                )
                            })
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(status_color)
                            .child(usage_text)
                    )
            )
            // Token breakdown legend (when we have token data)
            .when(total_tokens > 0, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .ml_2()
                        // Input tokens
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_sm()
                                        .bg(theme.colors.info)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(format!("In: {}", Self::format_token_count(self.stats.input_tokens)))
                                )
                        )
                        // Output tokens
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_sm()
                                        .bg(theme.colors.success)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(format!("Out: {}", Self::format_token_count(self.stats.output_tokens)))
                                )
                        )
                )
            })
            // Warning message if context is filling up with compact button
            .when_some(self.context_warning_message(), |d, warning| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_px()
                        .rounded_sm()
                        .bg(theme.colors.warning.opacity(0.1))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child("⚠️")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child(warning)
                                )
                        )
                        // Quick compact button
                        .child(
                            div()
                                .id("quick-compact-btn")
                                .px_2()
                                .py_px()
                                .rounded_sm()
                                .bg(theme.colors.warning.opacity(0.2))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.colors.warning)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.colors.warning.opacity(0.3)))
                                .on_click(cx.listener(|_this, _, _window, cx| {
                                    cx.emit(ChatViewEvent::Submit("/compact".to_string()));
                                }))
                                .child("📦 Compact")
                        )
                )
            })
    }
}
