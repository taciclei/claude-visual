//! Header rendering for MCP tools panel

use gpui::*;
use gpui::prelude::*;
use super::core::McpToolsPanel;

impl McpToolsPanel {
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
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("MCP Tools"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(format!("({} available)", self.tools.len())),
                    )
                    .when(!self.pending_calls.is_empty(), |this| {
                this.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_full()
                                .bg(theme.colors.warning)
                                .text_xs()
                                .text_color(theme.colors.background)
                                .child(format!("{}", self.pending_calls.len())),
                        )
                    }),
            )
    }
}
