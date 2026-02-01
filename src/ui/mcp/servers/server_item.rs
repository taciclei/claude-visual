//! Server item rendering

use gpui::*;
use gpui::prelude::*;
use super::core::McpServersPanel;
use super::types::{ServerConnectionStatus, McpServersPanelEvent};

impl McpServersPanel {
    /// Render a server item
    pub(crate) fn render_server_item(&self, server: &super::types::ServerItem, index: usize, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_selected = self.selected_server == Some(index);
        let is_enabled = server.config.enabled;
        let name = server.name.clone();
        let name_for_toggle = name.clone();
        let name_for_connect = name.clone();
        let server_status = server.status;

        div()
            .id(ElementId::Name(format!("mcp-server-{}", index).into()))
            .w_full()
            .py_2()
            .px_3()
            .rounded_md()
            .cursor_pointer()
            .when(is_selected, |this| {
                this.bg(theme.colors.surface_hover)
            })
            .hover(|this| this.bg(theme.colors.surface_hover))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        // Status indicator
                        div()
                            .w_2()
                            .h_2()
                            .rounded_full()
                            .bg(server.status.color()),
                    )
                    .child(
                        // Server name and info
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(if is_enabled {
                                                theme.colors.text
                                            } else {
                                                theme.colors.text_muted
                                            })
                                            .child(server.name.clone()),
                                    )
                                    .when(!is_enabled, |this| {
                this.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("(disabled)"),
                                        )
                                    }),
                            )
                            .child(
                                // Command and status
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(server.config.command.clone()),
                                    )
                                    .when(server.status == ServerConnectionStatus::Connected, |this| {
                this.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(format!(
                                                    "{} tools, {} resources",
                                                    server.tool_count, server.resource_count
                                                )),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        // Actions
                        div()
                            .flex()
                            .flex_row()
                            .gap_1()
                            .child(
                                // Enable/Disable toggle
                                div()
                                    .id(ElementId::Name(format!("toggle-{}", index).into()))
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .text_xs()
                                    .cursor_pointer()
                                    .bg(if is_enabled {
                                        theme.colors.accent.opacity(0.2)
                                    } else {
                                        theme.colors.surface
                                    })
                                    .text_color(if is_enabled {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.text_muted
                                    })
                                    .hover(|this| this.bg(theme.colors.surface_hover))
                                    .child(if is_enabled { "Enabled" } else { "Disabled" })
                                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                                        if is_enabled {
                                            cx.emit(McpServersPanelEvent::DisableServer(name_for_toggle.clone()));
                                        } else {
                                            cx.emit(McpServersPanelEvent::EnableServer(name_for_toggle.clone()));
                                        }
                                    })),
                            )
                            .when(is_enabled, |this| {
                this.child(
                                    // Connect/Disconnect button
                                    div()
                                        .id(ElementId::Name(format!("connect-{}", index).into()))
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .text_xs()
                                        .cursor_pointer()
                                        .bg(match server_status {
                                            ServerConnectionStatus::Connected => theme.colors.error.opacity(0.2),
                                            ServerConnectionStatus::Connecting => theme.colors.warning.opacity(0.2),
                                            _ => theme.colors.success.opacity(0.2),
                                        })
                                        .text_color(match server_status {
                                            ServerConnectionStatus::Connected => theme.colors.error,
                                            ServerConnectionStatus::Connecting => theme.colors.warning,
                                            _ => theme.colors.success,
                                        })
                                        .hover(|this| this.bg(theme.colors.surface_hover))
                                        .child(match server_status {
                                            ServerConnectionStatus::Connected => "Disconnect",
                                            ServerConnectionStatus::Connecting => "Connecting...",
                                            _ => "Connect",
                                        })
                                        .on_click(cx.listener(move |_this, _event, _window, cx| {
                                            let name = name_for_connect.clone();
                                            if server_status == ServerConnectionStatus::Connected {
                                                cx.emit(McpServersPanelEvent::DisconnectServer(name));
                                            } else if server_status != ServerConnectionStatus::Connecting {
                                                cx.emit(McpServersPanelEvent::ConnectServer(name));
                                            }
                                        })),
                                )
                            }),
                    ),
            )
            .when_some(server.error.clone(), |this, error| {
                this.child(
                    div()
                        .mt_1()
                        .px_3()
                        .text_xs()
                        .text_color(theme.colors.error)
                        .child(error),
                )
            })
    }
}
