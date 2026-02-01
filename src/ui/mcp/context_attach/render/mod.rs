//! Rendering implementation for MCP context attachment

use gpui::prelude::*;
use gpui::*;

use super::core::McpContextAttachPanel;
use super::types::*;

mod header;
mod resource_item;
mod resource_list;
mod search;

impl Render for McpContextAttachPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let attached_count = self
            .available_resources
            .iter()
            .filter(|r| matches!(r.status, AttachmentStatus::Attached))
            .count();

        div()
            .flex()
            .flex_col()
            .h_full()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_lg()
            .overflow_hidden()
            .child(self.render_header(attached_count, cx))
            .child(self.render_search(cx))
            .child(self.render_resource_list(cx))
    }
}
