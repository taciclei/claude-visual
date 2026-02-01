//! Tool item rendering for MCP tools panel

use super::core::McpToolsPanel;
use super::types::{ToolApprovalStatus, ToolItem};
use gpui::prelude::*;
use gpui::*;

impl McpToolsPanel {
    /// Render a tool item
    pub(crate) fn render_tool_item(
        &self,
        tool: &ToolItem,
        index: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_selected = self.selected_tool == Some(index);

        div()
            .id(ElementId::Name(format!("mcp-tool-{}", index).into()))
            .w_full()
            .py_2()
            .px_3()
            .rounded_md()
            .cursor_pointer()
            .when(is_selected, |this| this.bg(theme.colors.surface_hover))
            .hover(|this| this.bg(theme.colors.surface_hover))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
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
                                            .child(tool.tool.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!("({})", tool.server)),
                                    ),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded_md()
                                    .text_xs()
                                    .bg(match tool.approval {
                                        ToolApprovalStatus::Pending => {
                                            theme.colors.warning.opacity(0.2)
                                        }
                                        ToolApprovalStatus::ApprovedSession => {
                                            theme.colors.success.opacity(0.2)
                                        }
                                        ToolApprovalStatus::ApprovedPermanent => {
                                            theme.colors.accent.opacity(0.2)
                                        }
                                        ToolApprovalStatus::Denied => {
                                            theme.colors.error.opacity(0.2)
                                        }
                                    })
                                    .text_color(match tool.approval {
                                        ToolApprovalStatus::Pending => theme.colors.warning,
                                        ToolApprovalStatus::ApprovedSession => theme.colors.success,
                                        ToolApprovalStatus::ApprovedPermanent => {
                                            theme.colors.accent
                                        }
                                        ToolApprovalStatus::Denied => theme.colors.error,
                                    })
                                    .child(match tool.approval {
                                        ToolApprovalStatus::Pending => "Pending",
                                        ToolApprovalStatus::ApprovedSession => "Session",
                                        ToolApprovalStatus::ApprovedPermanent => "Always",
                                        ToolApprovalStatus::Denied => "Denied",
                                    }),
                            ),
                    )
                    .when_some(tool.tool.description.clone(), |this, desc| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(desc),
                        )
                    }),
            )
    }
}
