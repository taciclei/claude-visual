//! MCP servers panel render functions

mod empty_state;
mod footer;
mod header;
mod quick_tools;

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub(crate) fn render_mcp_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let servers = self
            .session_info
            .as_ref()
            .map(|info| info.mcp_servers.clone())
            .unwrap_or_default();

        // Count connected vs total
        let connected_count = servers
            .iter()
            .filter(|s| s.status == crate::claude::message::McpServerStatus::Connected)
            .count();
        let total_tools: usize = servers.iter().map(|s| s.tool_count).sum();
        let total_resources: usize = servers.iter().map(|s| s.resource_count).sum();

        // Extract listener for overlay click
        let overlay_click = cx.listener(|this, _, _window, cx| {
            this.toggle_mcp_panel(cx);
        });

        div()
            .id("mcp-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(overlay_click)
            .child(
                div()
                    .id("mcp-panel")
                    .w(px(500.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(header::render_header(
                        theme,
                        connected_count,
                        servers.len(),
                        total_tools,
                        total_resources,
                        cx.listener(|this, _, _window, cx| {
                            this.toggle_mcp_panel(cx);
                        }),
                    ))
                    // Server list
                    .child(
                        div()
                            .id("mcp-server-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(servers.is_empty(), |d| {
                                d.child(empty_state::render_empty_state(theme))
                            })
                            .children(servers.iter().enumerate().map(|(idx, server)| {
                                let server_name = server.name.clone();
                                let server_name_for_expand = server.name.clone();
                                let server_name_for_use = server.name.clone();
                                let is_expanded = self.is_mcp_server_expanded(&server.name);
                                let tools = self.get_mcp_server_tools(&server.name);

                                let (status_color, status_icon, status_text) = match server.status {
                                    crate::claude::message::McpServerStatus::Connected => (theme.colors.success, "●", "Connected"),
                                    crate::claude::message::McpServerStatus::Connecting => (theme.colors.warning, "◐", "Connecting..."),
                                    crate::claude::message::McpServerStatus::Disconnected => (theme.colors.text_muted, "○", "Disconnected"),
                                    crate::claude::message::McpServerStatus::Error => (theme.colors.error, "✕", "Error"),
                                };

                                // Copy theme colors for move closures
                                let surface_hover = theme.colors.surface_hover;
                                let text = theme.colors.text;
                                let text_muted = theme.colors.text_muted;
                                let accent = theme.colors.accent;
                                let background = theme.colors.background;
                                let border = theme.colors.border;
                                let info = theme.colors.info;

                                div()
                                    .flex()
                                    .flex_col()
                                    // Server header row (clickable to expand)
                                    .child(
                                        div()
                                            .id(ElementId::Name(format!("mcp-server-{}", idx).into()))
                                            .px_4()
                                            .py_3()
                                            .border_b_1()
                                            .border_color(border.opacity(0.5))
                                            .cursor_pointer()
                                            .hover(move |s| s.bg(surface_hover))
                                            .on_click(cx.listener(move |this, _, _window, cx| {
                                                this.toggle_mcp_server_expanded(&server_name_for_expand, cx);
                                            }))
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            // Expand indicator
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(text_muted)
                                                    .child(if is_expanded { "▼" } else { "▶" })
                                            )
                                            // Status indicator with color
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(status_color)
                                                    .child(status_icon)
                                            )
                                            // Server info
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(2.0))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(
                                                                div()
                                                                    .text_sm()
                                                                    .font_weight(FontWeight::MEDIUM)
                                                                    .text_color(text)
                                                                    .child(server.name.clone())
                                                            )
                                                            .child(
                                                                div()
                                                                    .px_2()
                                                                    .py_px()
                                                                    .rounded_sm()
                                                                    .bg(status_color.opacity(0.1))
                                                                    .text_xs()
                                                                    .text_color(status_color)
                                                                    .child(status_text)
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_3()
                                                            .text_xs()
                                                            .text_color(text_muted)
                                                            .child(format!("🔧 {} tools", server.tool_count))
                                                            .child(format!("📄 {} resources", server.resource_count))
                                                    )
                                            )
                                            // Quick use button
                                            .child(
                                                div()
                                                    .id(ElementId::Name(format!("mcp-use-{}", idx).into()))
                                                    .px_2()
                                                    .py_1()
                                                    .rounded_md()
                                                    .text_xs()
                                                    .text_color(accent)
                                                    .bg(accent.opacity(0.1))
                                                    .hover(move |s| s.bg(accent.opacity(0.2)))
                                                    .cursor_pointer()
                                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                                        let prompt = format!("Use the {} MCP server to ", server_name_for_use);
                                                        this.input.update(cx, |input, cx| {
                                                            input.clear(cx);
                                                            input.insert_text(&prompt, cx);
                                                        });
                                                        this.panels.mcp_panel = false;
                                                        cx.notify();
                                                    }))
                                                    .child("Use...")
                                            )
                                    )
                                    // Expanded tools list
                                    .when(is_expanded && !tools.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .px_4()
                                                .py_2()
                                                .pl(px(48.0))
                                                .bg(background)
                                                .border_b_1()
                                                .border_color(border.opacity(0.3))
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(text_muted)
                                                        .mb_1()
                                                        .child("Available tools:")
                                                )
                                                .children(tools.iter().take(10).enumerate().map(|(tidx, tool)| {
                                                    let tool_name = tool.clone();
                                                    div()
                                                        .id(ElementId::Name(format!("mcp-tool-{}-{}", idx, tidx).into()))
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_sm()
                                                        .text_xs()
                                                        .text_color(info)
                                                        .hover(move |s| s.bg(info.opacity(0.1)))
                                                        .cursor_pointer()
                                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                                            this.use_mcp_tool(&tool_name, cx);
                                                        }))
                                                        .child(format!("🔧 {}", tool))
                                                }))
                                                .when(tools.len() > 10, |d| {
                                                    d.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(text_muted)
                                                            .mt_1()
                                                            .child(format!("...and {} more", tools.len() - 10))
                                                    )
                                                })
                                        )
                                    })
                                    // Show placeholder when expanded but no matching tools found
                                    .when(is_expanded && tools.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .px_4()
                                                .py_2()
                                                .pl(px(48.0))
                                                .bg(background)
                                                .border_b_1()
                                                .border_color(border.opacity(0.3))
                                                .text_xs()
                                                .text_color(text_muted)
                                                .child(format!("{} tools available (click 'Use...' to interact)", server.tool_count))
                                        )
                                    })
                            }))
                    )
                    // Footer with help text
                    .child(footer::render_footer(theme))
            )
    }
}
