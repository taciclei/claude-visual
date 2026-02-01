//! Footer actions rendering for tool result blocks

use gpui::prelude::*;
use gpui::*;

use super::types::{ToolExecutionStatus, ToolResultBlock, ToolResultBlockEvent};

/// Get contextual skill suggestion based on tool name and status
fn get_tool_skill_suggestion(
    tool_name: &str,
    status: &ToolExecutionStatus,
) -> Option<(&'static str, &'static str, &'static str)> {
    let tool_lower = tool_name.to_lowercase();

    // Error-specific suggestions
    if matches!(status, ToolExecutionStatus::Error) {
        if tool_lower.contains("bash") {
            return Some(("🐛", "Debug", "/debug"));
        }
        return Some(("🐛", "Debug", "/debug"));
    }

    // Success suggestions based on tool type
    match tool_lower.as_str() {
        "bash" => Some(("📖", "Explain", "/explain")),
        "read" => Some(("📖", "Explain", "/explain")),
        "write" | "edit" => Some(("👀", "Review", "/review")),
        "grep" | "glob" => Some(("🔍", "Explore", "/explore")),
        "task" => Some(("⚡", "APEX", "/apex")),
        "websearch" | "webfetch" => Some(("💡", "Brainstorm", "/brainstorm")),
        _ => None,
    }
}

impl ToolResultBlock {
    pub(super) fn render_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let theme_colors = theme.colors.clone();

        // Extract listeners before div chains
        let on_copy_click = cx.listener(|_this, _, _window, cx| {
            cx.emit(ToolResultBlockEvent::CopyResult);
        });

        let tool_name = self.result.tool_name.clone();
        let server_name = self.result.server_name.clone();
        let arguments = self.result.arguments.clone();

        let on_retry_click = cx.listener(move |_this, _, _window, cx| {
            cx.emit(ToolResultBlockEvent::Retry {
                tool_name: tool_name.clone(),
                server_name: server_name.clone(),
                arguments: arguments.clone(),
            });
        });

        // Get contextual skill suggestion
        let skill_suggestion =
            get_tool_skill_suggestion(&self.result.tool_name, &self.result.status);

        div()
            .px_3()
            .py_2()
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            // Left side: contextual skill suggestions
            .child(div().flex().items_center().gap_2().when_some(
                skill_suggestion,
                |d, (icon, label, cmd)| {
                    let cmd_str = cmd.to_string();
                    let accent = theme_colors.accent;
                    d.child(
                        div()
                            .id("skill-suggestion")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .text_xs()
                            .bg(accent.opacity(0.1))
                            .border_1()
                            .border_color(accent.opacity(0.2))
                            .text_color(accent)
                            .cursor_pointer()
                            .hover(move |s| s.bg(accent.opacity(0.2)))
                            .on_click(cx.listener(move |_this, _, _window, cx| {
                                cx.emit(ToolResultBlockEvent::ExecuteSkill(cmd_str.clone()));
                            }))
                            .child(icon)
                            .child(label),
                    )
                },
            ))
            // Right side: action buttons
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Copy button
                    .when(
                        self.result.content.is_some() || self.result.error.is_some(),
                        |d| {
                            // Clone for move closure
                            let theme_colors_copy = theme_colors.clone();

                            d.child(
                                div()
                                    .id("copy-btn")
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(theme_colors.text_muted)
                                    .hover(move |style| {
                                        style
                                            .bg(theme_colors_copy.surface_hover)
                                            .text_color(theme_colors_copy.text)
                                    })
                                    .cursor_pointer()
                                    .on_click(on_copy_click)
                                    .child("Copy"),
                            )
                        },
                    )
                    // Retry button (only for errors)
                    .when(self.result.status == ToolExecutionStatus::Error, |d| {
                        // Clone for move closure
                        let theme_colors_retry = theme_colors.clone();

                        d.child(
                            div()
                                .id("retry-btn")
                                .px_2()
                                .py_1()
                                .rounded_sm()
                                .text_xs()
                                .text_color(theme_colors.accent)
                                .hover(move |style| {
                                    style
                                        .bg(theme_colors_retry.accent.opacity(0.1))
                                        .text_color(theme_colors_retry.accent)
                                })
                                .cursor_pointer()
                                .on_click(on_retry_click)
                                .child("Retry"),
                        )
                    }),
            )
    }
}
