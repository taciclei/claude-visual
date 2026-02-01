//! Resource list rendering for MCP context attachment panel

use gpui::*;
use gpui::prelude::*;

use super::super::core::McpContextAttachPanel;

impl McpContextAttachPanel {
    /// Render the resource list container
    pub(super) fn render_resource_list(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filtered = self.filtered_resources();

        div()
            .flex_1()
            .id("scroll-attach-resources")
            .overflow_y_scroll()
            .when(filtered.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .py_8()
                        .gap_2()
                        .child(
                            div()
                                .text_2xl()
                                .text_color(theme.colors.text_muted)
                                .child("📦"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("No resources available"),
                        ),
                )
            })
            .when(!filtered.is_empty(), |d| {
                d.children(filtered.into_iter().map(|(idx, resource)| {
                    self.render_resource_item(idx, resource, cx)
                }))
            })
    }
}
