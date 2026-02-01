//! Render trait implementation for MCP tools panel

use gpui::*;
use gpui::prelude::*;
use super::core::McpToolsPanel;

impl Render for McpToolsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let filtered_tools = self.filtered_tools();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.colors.surface)
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .child(self.render_header(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_2()
                    .gap_2()
                    .when(!self.pending_calls.is_empty(), |this| {
                        let pending = self.pending_calls.clone();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.warning)
                                        .child("Pending Approvals"),
                                )
                                .children(
                                    pending.iter().map(|call| self.render_pending_call(call, cx)),
                                ),
                        )
                    })
                    .when(filtered_tools.is_empty() && self.pending_calls.is_empty(), |this| {
                this.child(
                            div()
                                .py_4()
                                .text_center()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("No tools available"),
                        )
                    })
                    .when(!filtered_tools.is_empty(), |this| {
                        let tools: Vec<_> = filtered_tools.iter().cloned().cloned().collect();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .children(
                                    tools
                                        .iter()
                                        .enumerate()
                                        .map(|(i, tool)| self.render_tool_item(tool, i, cx)),
                                ),
                        )
                    }),
            )
    }
}
