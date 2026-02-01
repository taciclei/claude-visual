//! Left section of status bar - project info, git, model, streaming, MCP, context, memory

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::types::StatusBarEvent;
use super::helpers::shorten_path;
use super::status_bar::StatusBar;

/// Render left section of status bar
pub(crate) fn render_left_section(
    git_branch: &Option<String>,
    project_path: &Option<String>,
    model_name: &str,
    is_streaming: bool,
    mcp_server_count: usize,
    mcp_tool_count: usize,
    context_files_count: usize,
    memory_items_count: usize,
    theme: &Theme,
    cx: &mut Context<StatusBar>,
) -> impl IntoElement {
    let git_status = git_branch.clone().unwrap_or_else(|| "no-git".to_string());
    let path_display = shorten_path(project_path);

    div()
        .flex()
        .items_center()
        .gap_2()
        // Git status
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .text_color(if git_branch.is_some() {
                    theme.colors.success
                } else {
                    theme.colors.text_muted
                })
                .child(git_status)
        )
        // Bullet separator
        .child(
            div()
                .text_color(theme.colors.text_muted)
                .child("•")
        )
        // Project path
        .child(
            div()
                .text_color(theme.colors.text)
                .child(path_display)
        )
        // Bullet separator
        .child(
            div()
                .text_color(theme.colors.text_muted)
                .child("•")
        )
        // Model name (clickable to open model switcher)
        .child(
            div()
                .id("status-model")
                .flex()
                .items_center()
                .gap_1()
                .px_1()
                .rounded_sm()
                .cursor_pointer()
                .text_color(theme.colors.accent)
                .font_weight(FontWeight::MEDIUM)
                .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                .on_click(cx.listener(|_this, _, _window, cx| {
                    cx.emit(StatusBarEvent::OpenModelSwitcher);
                }))
                .child(model_name.to_string())
                .child(
                    div()
                        .text_color(theme.colors.text_muted)
                        .text_xs()
                        .child("▾")
                )
        )
        // Streaming indicator (clickable to stop)
        .when(is_streaming, |this| {
            this.child(
                div()
                    .id("status-streaming")
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.error.opacity(0.1)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(StatusBarEvent::StopStreaming);
                    }))
                    .child(
                        div()
                            .size(px(6.0))
                            .rounded_full()
                            .bg(theme.colors.success)
                    )
                    .child(
                        div()
                            .text_color(theme.colors.success)
                            .child("streaming")
                    )
                    .child(
                        div()
                            .text_color(theme.colors.text_muted)
                            .ml_1()
                            .text_xs()
                            .child("(⌘. to stop)")
                    )
            )
        })
        // MCP servers indicator (when connected)
        .when(mcp_server_count > 0, |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .child("🔌")
                    .child(format!("{} MCP", mcp_server_count))
                    .when(mcp_tool_count > 0, |d| {
                        d.child(
                            div()
                                .text_color(theme.colors.text_muted)
                                .child(format!("({})", mcp_tool_count))
                        )
                    })
            )
        })
        // Context files indicator (when files attached)
        .when(context_files_count > 0, |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.accent)
                    .child("📎")
                    .child(format!("{}", context_files_count))
            )
        })
        // Memory items indicator (when memories exist)
        .when(memory_items_count > 0, |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .child("🧠")
                    .child(format!("{}", memory_items_count))
            )
        })
}
