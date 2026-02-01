//! Content section rendering for tool result blocks

use gpui::prelude::*;
use gpui::*;

use super::types::{ToolExecutionStatus, ToolResultBlock};

impl ToolResultBlock {
    pub(super) fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let has_content = self.result.content.is_some();
        let has_error = self.result.error.is_some();
        let is_pending = self.result.status == ToolExecutionStatus::Pending;

        div()
            // Success content
            .when(has_content, |d| {
                if let Some(content) = &self.result.content {
                    let formatted = self.format_json(content);
                    d.child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(theme.colors.background)
                            .text_sm()
                            .font_family("JetBrains Mono")
                            .text_color(theme.colors.text)
                            .whitespace_nowrap()
                            .max_h(px(300.0))
                            .id("scroll-tool-result")
                            .overflow_y_scroll()
                            .child(formatted),
                    )
                } else {
                    d
                }
            })
            // Error content
            .when(has_error && !has_content, |d| {
                if let Some(error) = &self.result.error {
                    d.child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(theme.colors.error.opacity(0.05))
                            .text_sm()
                            .text_color(theme.colors.error)
                            .whitespace_nowrap()
                            .child(error.clone()),
                    )
                } else {
                    d
                }
            })
            // Pending state
            .when(is_pending && !has_content && !has_error, |d| {
                d.child(
                    div()
                        .px_3()
                        .py_4()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .child(
                            // Spinning indicator
                            div()
                                .size(px(16.0))
                                .rounded_full()
                                .border_2()
                                .border_color(theme.colors.accent.opacity(0.3))
                                .border_color(theme.colors.accent),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("Executing tool..."),
                        ),
                )
            })
            // Empty state
            .when(!has_content && !has_error && !is_pending, |d| {
                d.child(
                    div()
                        .px_3()
                        .py_4()
                        .flex()
                        .justify_center()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("No result"),
                )
            })
    }
}
