//! Context indicator view rendering

use super::colors::default_colors;
use super::events::ContextIndicatorEvent;
use super::state::ContextIndicator;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl Render for ContextIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();
        let percentage = self.usage.percentage();
        let tokens_display = self.usage.format_tokens();
        let max_display = self.usage.format_max_tokens();
        let is_warning = self.usage.is_warning();
        let is_critical = self.usage.is_critical();
        let progress_color = self.progress_color(&colors);
        let message_count = self.usage.message_count;
        let file_count = self.usage.file_count;
        let show_details = self.show_details;

        // Copy colors for move closures
        let hover_color = colors.hover;
        let border_color = colors.border;
        let surface_color = colors.surface;
        let text_muted_color = colors.text_muted;
        let text_color = colors.text;
        let accent_color = colors.accent;
        let error_color = colors.error;
        let warning_color = colors.warning;

        // Extract listeners before div chains
        let toggle_listener = cx.listener(|this, _, _window, cx| {
            this.toggle_details(cx);
        });

        let panel_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(ContextIndicatorEvent::ShowContextPanel);
        });

        div()
            .id("context-indicator")
            .flex()
            .items_center()
            .gap_2()
            .px_2()
            .py_1()
            .rounded_md()
            .cursor_pointer()
            .hover(move |style| style.bg(hover_color))
            .on_click(toggle_listener)
            // Compact view: progress bar with token count
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Progress bar
                    .child(
                        div()
                            .w(px(60.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(border_color)
                            .overflow_hidden()
                            .child(
                                div()
                                    .h_full()
                                    .w(pct(percentage.min(100.0)))
                                    .rounded_full()
                                    .bg(progress_color),
                            ),
                    )
                    // Token count
                    .child(
                        div()
                            .text_xs()
                            .text_color(if is_critical {
                                error_color
                            } else if is_warning {
                                warning_color
                            } else {
                                text_muted_color
                            })
                            .child(format!("{}/{}", tokens_display, max_display)),
                    ),
            )
            // Detailed view (shown when clicked)
            .when(show_details, move |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(28.0))
                        .right_0()
                        .w(px(200.0))
                        .bg(surface_color)
                        .border_1()
                        .border_color(border_color)
                        .rounded_md()
                        .shadow_lg()
                        .p_2()
                        .flex()
                        .flex_col()
                        .gap_2()
                        // Token usage
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(div().text_xs().text_color(text_muted_color).child("Tokens"))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(text_color)
                                        .child(format!("{} / {}", tokens_display, max_display)),
                                ),
                        )
                        // Percentage
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(div().text_xs().text_color(text_muted_color).child("Usage"))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(progress_color)
                                        .child(format!("{:.1}%", percentage)),
                                ),
                        )
                        // Messages
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted_color)
                                        .child("Messages"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_color)
                                        .child(message_count.to_string()),
                                ),
                        )
                        // Files
                        .when(file_count > 0, move |this| {
                            this.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div().text_xs().text_color(text_muted_color).child("Files"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(text_color)
                                            .child(file_count.to_string()),
                                    ),
                            )
                        })
                        // Warning message
                        .when(is_warning, move |this| {
                            this.child(
                                div()
                                    .mt_1()
                                    .pt_1()
                                    .border_t_1()
                                    .border_color(border_color)
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(if is_critical {
                                                error_color
                                            } else {
                                                warning_color
                                            })
                                            .child(if is_critical {
                                                "Context nearly full!"
                                            } else {
                                                "Context getting full"
                                            }),
                                    ),
                            )
                        })
                        // Show context panel button
                        .child(
                            div()
                                .id("show-context-panel")
                                .mt_1()
                                .pt_1()
                                .border_t_1()
                                .border_color(border_color)
                                .cursor_pointer()
                                .hover(move |style| style.bg(hover_color))
                                .rounded_sm()
                                .p_1()
                                .on_click(panel_listener)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(accent_color)
                                        .text_center()
                                        .child("Manage Context"),
                                ),
                        ),
                )
            })
    }
}
