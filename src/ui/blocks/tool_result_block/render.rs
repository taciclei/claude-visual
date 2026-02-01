//! Main render implementation for tool result blocks

use gpui::prelude::*;
use gpui::*;

use super::types::{ToolExecutionStatus, ToolResultBlock};

impl Render for ToolResultBlock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        // Border color based on status
        let border_color = match self.result.status {
            ToolExecutionStatus::Success => theme.colors.success.opacity(0.3),
            ToolExecutionStatus::Error => theme.colors.error.opacity(0.3),
            ToolExecutionStatus::Pending => theme.colors.warning.opacity(0.3),
            ToolExecutionStatus::Cancelled => theme.colors.border,
        };

        div()
            .w_full()
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(border_color)
            .bg(theme.colors.surface)
            // Header
            .child(self.render_header(cx))
            // Content (when not collapsed)
            .when(!self.collapsed, |this| {
                this.child(self.render_arguments(cx))
            })
            // Result content
            .when(!self.collapsed, |this| this.child(self.render_content(cx)))
            // Footer actions
            .when(!self.collapsed, |this| this.child(self.render_footer(cx)))
    }
}
