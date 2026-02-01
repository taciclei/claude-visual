//! Pending call rendering for MCP tools panel

use gpui::*;
use gpui::prelude::*;
use super::core::McpToolsPanel;
use super::types::{PendingToolCall, McpToolsPanelEvent};

impl McpToolsPanel {
    /// Render a pending call item
    pub(crate) fn render_pending_call(&self, call: &PendingToolCall, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let call_id = call.id.clone();
        let call_id_deny = call.id.clone();

        div()
            .w_full()
            .p_3()
            .rounded_lg()
            .bg(theme.colors.warning.opacity(0.1))
            .border_1()
            .border_color(theme.colors.warning.opacity(0.3))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.text)
                                            .child(call.tool_name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!("from {}", call.server)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.warning)
                                    .child("Awaiting approval"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .justify_end()
                            .child(
                                div()
                                    .id(ElementId::Name(format!("deny-{}", call.id).into()))
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .text_sm()
                                    .cursor_pointer()
                                    .bg(theme.colors.error.opacity(0.2))
                                    .text_color(theme.colors.error)
                                    .hover(|this| this.bg(theme.colors.error.opacity(0.3)))
                                    .child("Deny")
                                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                                        cx.emit(McpToolsPanelEvent::DenyToolCall(call_id_deny.clone()));
                                    })),
                            )
                            .child(
                                div()
                                    .id(ElementId::Name(format!("approve-{}", call.id).into()))
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .text_sm()
                                    .cursor_pointer()
                                    .bg(theme.colors.success.opacity(0.2))
                                    .text_color(theme.colors.success)
                                    .hover(|this| this.bg(theme.colors.success.opacity(0.3)))
                                    .child("Approve")
                                    .on_click(cx.listener(move |_this, _event, _window, cx| {
                                        cx.emit(McpToolsPanelEvent::ApproveToolCall {
                                            call_id: call_id.clone(),
                                            permanent: false,
                                        });
                                    })),
                            ),
                    ),
            )
    }
}
