//! Search input rendering for MCP context attachment panel

use gpui::*;
use gpui::prelude::*;

use super::super::core::McpContextAttachPanel;

impl McpContextAttachPanel {
    /// Render the search input section
    pub(super) fn render_search(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(theme.colors.border)
                    .rounded_sm()
                    .child(div().text_xs().text_color(theme.colors.text_muted).child("🔍"))
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(if self.filter_text.is_empty() {
                                theme.colors.text_muted
                            } else {
                                theme.colors.text
                            })
                            .child(if self.filter_text.is_empty() {
                                "Filter resources...".to_string()
                            } else {
                                self.filter_text.clone()
                            }),
                    ),
            )
    }
}
