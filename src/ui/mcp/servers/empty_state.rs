//! Empty state rendering

use super::core::McpServersPanel;
use gpui::prelude::*;
use gpui::*;

impl McpServersPanel {
    /// Render the empty state
    pub(crate) fn render_empty(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .py_4()
            .text_center()
            .text_sm()
            .text_color(theme.colors.text_muted)
            .child("No MCP servers configured")
            .child(
                div()
                    .mt_2()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("Add servers in mcp.json or click \"+ Add\""),
            )
    }
}
