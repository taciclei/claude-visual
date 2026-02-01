//! Header rendering

use gpui::*;
use gpui::prelude::*;
use super::core::McpServersPanel;
use super::types::{ServerConnectionStatus, McpServersPanelEvent};

impl McpServersPanel {
    /// Render the header
    pub(crate) fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .id("servers-header-toggle")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.toggle_expanded(cx);
                    }))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("MCP Servers"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(format!(
                                "({} connected)",
                                self.servers
                                    .iter()
                                    .filter(|s| s.status == ServerConnectionStatus::Connected)
                                    .count()
                            )),
                    ),
            )
            .child(
                // Add server button
                div()
                    .id("add-mcp-server")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .text_xs()
                    .cursor_pointer()
                    .text_color(theme.colors.accent)
                    .hover(|this| this.bg(theme.colors.surface_hover))
                    .child("+ Add")
                    .on_click(cx.listener(|_this, _event, _window, cx| {
                        cx.emit(McpServersPanelEvent::AddServer);
                    })),
            )
    }
}
