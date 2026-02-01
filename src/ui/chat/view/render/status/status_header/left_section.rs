//! Left section badges rendering

use super::super::super::super::core::ChatView;
use super::helpers::{format_session_id_short, get_folder_name};
use crate::app::theme::Theme;
use crate::claude::message::{McpServerStatus, SessionInfo};
use crate::ui::chat::view::types::NotificationType;
use gpui::*;

impl ChatView {
    /// Render the model badge (clickable for model switching)
    pub(super) fn render_model_badge(
        &self,
        info: &SessionInfo,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("model-badge")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .cursor_pointer()
            .bg(theme.colors.accent.opacity(0.1))
            .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_model_switcher(cx);
            }))
            .child(div().text_xs().text_color(theme.colors.accent).child("🤖"))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.accent)
                    .font_weight(FontWeight::MEDIUM)
                    .child(info.model.clone()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.accent.opacity(0.6))
                    .child("▼"),
            )
    }

    /// Render the tools count badge
    pub(super) fn render_tools_badge(
        &self,
        info: &SessionInfo,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tools_count = info.tools.len();
        div()
            .id("status-tools")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.info.opacity(0.1))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.info.opacity(0.2)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_session_details(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .child(format!("🔧 {}", tools_count)),
            )
    }

    /// Render the skills/commands count badge
    pub(super) fn render_skills_badge(
        &self,
        info: &SessionInfo,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let total_count = info.skills.len() + info.slash_commands.len();
        div()
            .id("status-skills")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.warning.opacity(0.1))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.warning.opacity(0.2)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_session_details(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .child(format!("⚡ {}", total_count)),
            )
    }

    /// Render the agents count badge
    pub(super) fn render_agents_badge(
        &self,
        info: &SessionInfo,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let agents_count = info.agents.len();
        div()
            .id("status-agents")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.success.opacity(0.1))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.success.opacity(0.2)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_session_details(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.success)
                    .child(format!("🤖 {}", agents_count)),
            )
    }

    /// Render the think mode indicator
    pub(super) fn render_think_mode_badge(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("status-think-mode")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.warning.opacity(0.15))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.warning.opacity(0.25)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_think_mode(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.warning)
                    .font_weight(FontWeight::MEDIUM)
                    .child("🧠 Think Mode"),
            )
    }

    /// Render the pending permissions indicator
    pub(super) fn render_permissions_badge(
        &self,
        count: usize,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("status-permissions")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.error.opacity(0.15))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.error.opacity(0.25)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_permissions_panel(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.error)
                    .font_weight(FontWeight::MEDIUM)
                    .child(format!("🔐 {} pending", count)),
            )
    }

    /// Render the working directory badge
    pub(super) fn render_cwd_badge(&self, info: &SessionInfo, theme: &Theme) -> impl IntoElement {
        let cwd_for_click = info.cwd.clone();
        let folder_name = get_folder_name(&info.cwd);

        div()
            .id("status-cwd")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.surface)
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.surface_hover))
            .on_click(move |_, _window, _cx| {
                // Open directory in Finder (macOS)
                let _ = std::process::Command::new("open")
                    .arg(&cwd_for_click)
                    .spawn();
            })
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("📁"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .max_w(px(200.0))
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(folder_name),
            )
    }

    /// Render the MCP servers status badge
    pub(super) fn render_mcp_badge(&self, info: &SessionInfo, theme: &Theme) -> impl IntoElement {
        let connected_count = info
            .mcp_servers
            .iter()
            .filter(|s| s.status == McpServerStatus::Connected)
            .count();
        let total_count = info.mcp_servers.len();
        let all_connected = connected_count == total_count;
        let status_color = if all_connected {
            theme.colors.success
        } else if connected_count > 0 {
            theme.colors.warning
        } else {
            theme.colors.error
        };

        div()
            .id("status-mcp")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(status_color.opacity(0.1))
            .child(div().text_xs().text_color(status_color).child("🔌"))
            .child(
                div()
                    .text_xs()
                    .text_color(status_color)
                    .child(format!("{}/{} MCP", connected_count, total_count)),
            )
    }

    /// Render the session ID badge (left section version)
    pub(super) fn render_session_id_badge_left(
        &self,
        session_id: String,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let short_id = format_session_id_short(&session_id);

        div()
            .id("status-session-id")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.text_muted.opacity(0.1))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.text_muted.opacity(0.15)))
            .on_click(cx.listener(move |this, _, _window, cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(session_id.clone()));
                this.show_notification(
                    "Session ID copied to clipboard",
                    NotificationType::Success,
                    cx,
                );
            }))
            .child(
                div()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(theme.colors.text_muted)
                    .child(short_id),
            )
    }
}
