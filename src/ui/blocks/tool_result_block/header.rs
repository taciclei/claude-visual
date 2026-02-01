//! Header rendering for tool result blocks

use gpui::prelude::*;
use gpui::*;

use super::types::{ToolExecutionStatus, ToolResultBlock};

impl ToolResultBlock {
    pub(super) fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let result = &self.result;

        // Status color
        let status_color = match result.status {
            ToolExecutionStatus::Success => theme.colors.success,
            ToolExecutionStatus::Error => theme.colors.error,
            ToolExecutionStatus::Pending => theme.colors.warning,
            ToolExecutionStatus::Cancelled => theme.colors.text_muted,
        };

        // Extract listener before div chain
        let on_header_click = cx.listener(|this, _, _window, cx| {
            this.toggle_collapsed(cx);
        });

        // Clone for move closure
        let theme_colors = theme.colors.clone();

        div()
            .id("tool-result-header")
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .cursor_pointer()
            .on_click(on_header_click)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Collapse indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme_colors.text_muted)
                            .child(if self.collapsed { "▸" } else { "▾" }),
                    )
                    // Tool icon
                    .child(
                        div()
                            .px_1()
                            .py_0p5()
                            .rounded_sm()
                            .bg(theme_colors.accent.opacity(0.1))
                            .text_xs()
                            .text_color(theme_colors.accent)
                            .child("🔧"),
                    )
                    // Tool name
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme_colors.text)
                            .child(result.tool_name.clone()),
                    )
                    // Server name
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(theme_colors.background)
                            .text_xs()
                            .text_color(theme_colors.text_muted)
                            .child(result.server_name.clone()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Duration
                    .when_some(result.duration, |this, duration| {
                        this.child(
                            gpui::div()
                                .text_xs()
                                .text_color(theme_colors.text_muted)
                                .child(self.format_duration(duration)),
                        )
                    })
                    // Status badge
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(status_color.opacity(0.15))
                            .text_xs()
                            .text_color(status_color)
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(result.status.icon())
                            .child(result.status.as_str()),
                    ),
            )
    }
}
