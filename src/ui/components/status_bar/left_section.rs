//! Left section of status bar - project info, git, model, streaming, MCP, context, memory

use gpui::prelude::*;
use gpui::*;

use super::helpers::shorten_path;
use super::status_bar::StatusBar;
use super::types::StatusBarEvent;
use crate::app::theme::Theme;

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
                .child(git_status),
        )
        // Bullet separator
        .child(div().text_color(theme.colors.text_muted).child("•"))
        // Project path
        .child(div().text_color(theme.colors.text).child(path_display))
        // Bullet separator
        .child(div().text_color(theme.colors.text_muted).child("•"))
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
                        .child("▾"),
                ),
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
                    .child(div().size(px(6.0)).rounded_full().bg(theme.colors.success))
                    .child(div().text_color(theme.colors.success).child("streaming"))
                    .child(
                        div()
                            .text_color(theme.colors.text_muted)
                            .ml_1()
                            .text_xs()
                            .child("(⌘. to stop)"),
                    ),
            )
        })
        // MCP servers indicator (clickable to open MCP panel)
        .when(mcp_server_count > 0, |this| {
            this.child(
                div()
                    .id("status-mcp")
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .hover(|s| s.bg(theme.colors.info.opacity(0.1)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(StatusBarEvent::OpenMcpPanel);
                    }))
                    .child("🔌")
                    .child(format!("{} MCP", mcp_server_count))
                    .when(mcp_tool_count > 0, |d| {
                        d.child(
                            div()
                                .text_color(theme.colors.text_muted)
                                .child(format!("({}t)", mcp_tool_count)),
                        )
                    }),
            )
        })
        // Context files indicator (clickable to open context panel)
        .when(context_files_count > 0, |this| {
            this.child(
                div()
                    .id("status-context")
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(StatusBarEvent::OpenContextPanel);
                    }))
                    .child("📎")
                    .child(format!(
                        "{} file{}",
                        context_files_count,
                        if context_files_count == 1 { "" } else { "s" }
                    )),
            )
        })
        // Memory items indicator (clickable to manage memory)
        .when(memory_items_count > 0, |this| {
            this.child(
                div()
                    .id("status-memory")
                    .flex()
                    .items_center()
                    .gap_1()
                    .ml_2()
                    .px_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .hover(|s| s.bg(theme.colors.warning.opacity(0.1)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(StatusBarEvent::OpenMemoryPanel);
                    }))
                    .child("🧠")
                    .child(format!("{} mem", memory_items_count)),
            )
        })
}
