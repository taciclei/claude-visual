//! Main panel Render trait implementation

use gpui::prelude::*;
use gpui::*;

use crate::ui::mcp::logs::core::McpLogsPanel;
use crate::ui::mcp::logs::types::{LogLevel, McpLogsPanelEvent};

impl Render for McpLogsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let filtered_logs = self.filtered_logs();
        let log_count = filtered_logs.len();
        let total_count = self.logs.len();
        let (debug_count, info_count, warn_count, error_count) = self.count_by_level();

        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let surface_hover = theme.colors.surface_hover;
        let border_color = theme.colors.border;
        let background = theme.colors.background;

        let toggle_expanded_handler = cx.listener(|this, _, _window, cx| {
            this.toggle_expanded(cx);
        });

        let toggle_auto_scroll_handler = cx.listener(|this, _, _window, cx| {
            this.toggle_auto_scroll(cx);
        });

        let clear_logs_handler = cx.listener(|this, _, _window, cx| {
            this.clear(cx);
            cx.emit(McpLogsPanelEvent::ClearLogs);
        });

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(background)
            // Header
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_2()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(border_color)
                    // Title and counts
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .id("toggle-logs-expanded")
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .cursor_pointer()
                                    .on_click(toggle_expanded_handler)
                                    .child(if self.expanded { "▼" } else { "▶" })
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(text_color)
                                            .child("Server Logs"),
                                    ),
                            )
                            // Level counts
                            .when(error_count > 0, |d| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py_px()
                                        .text_xs()
                                        .rounded_sm()
                                        .bg(LogLevel::Error.color().opacity(0.2))
                                        .text_color(LogLevel::Error.color())
                                        .child(format!("{} errors", error_count)),
                                )
                            })
                            .when(warn_count > 0, |d| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py_px()
                                        .text_xs()
                                        .rounded_sm()
                                        .bg(LogLevel::Warning.color().opacity(0.2))
                                        .text_color(LogLevel::Warning.color())
                                        .child(format!("{} warnings", warn_count)),
                                )
                            }),
                    )
                    // Actions
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Auto-scroll toggle
                            .child(
                                div()
                                    .id("auto-scroll-toggle")
                                    .px_2()
                                    .py_1()
                                    .text_xs()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .bg(if self.auto_scroll {
                                        accent_color.opacity(0.2)
                                    } else {
                                        gpui::transparent_black()
                                    })
                                    .text_color(if self.auto_scroll {
                                        accent_color
                                    } else {
                                        text_muted
                                    })
                                    .hover(|s| s.bg(surface_hover))
                                    .on_click(toggle_auto_scroll_handler)
                                    .child("Auto-scroll"),
                            )
                            // Clear button
                            .child(
                                div()
                                    .id("clear-logs-button")
                                    .px_2()
                                    .py_1()
                                    .text_xs()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .text_color(text_muted)
                                    .hover(|s| s.bg(surface_hover).text_color(text_color))
                                    .on_click(clear_logs_handler)
                                    .child("Clear"),
                            ),
                    ),
            )
            // Filter bar (when expanded)
            .when(self.expanded, |d| {
                d.child(
                    div()
                        .w_full()
                        .px_3()
                        .py_2()
                        .flex()
                        .items_center()
                        .gap_3()
                        .border_b_1()
                        .border_color(border_color)
                        // Level filters
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(self.render_level_filter(LogLevel::Debug, &theme, cx))
                                .child(self.render_level_filter(LogLevel::Info, &theme, cx))
                                .child(self.render_level_filter(LogLevel::Warning, &theme, cx))
                                .child(self.render_level_filter(LogLevel::Error, &theme, cx)),
                        )
                        // Count indicator
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!("{} / {} logs", log_count, total_count)),
                        ),
                )
            })
            // Log entries (when expanded)
            .when(self.expanded, |d| {
                d.child(
                    div()
                        .flex_1()
                        .id("scroll-logs-entries")
                        .overflow_y_scroll()
                        .when(filtered_logs.is_empty(), |d| {
                            d.child(
                                div()
                                    .w_full()
                                    .py_8()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(text_muted)
                                            .child("No logs to display"),
                                    ),
                            )
                        })
                        .when(!filtered_logs.is_empty(), |d| {
                            d.children(
                                filtered_logs
                                    .iter()
                                    .map(|(idx, entry)| {
                                        self.render_log_entry(*idx, entry, &theme, cx)
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        }),
                )
            })
    }
}
