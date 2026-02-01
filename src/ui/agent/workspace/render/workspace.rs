//! Main workspace rendering implementation

use gpui::*;
use gpui::prelude::*;
use super::super::types::*;
use super::super::state::AgentWorkspace;
use super::{render_approval_prompt, header, controls, logs};

impl Render for AgentWorkspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();
        let mode = self.mode;
        let is_expanded = self.is_expanded;
        let has_pending_approval = self.pending_approval.is_some();
        let task_description = self.task_description.clone();
        let error = self.error.clone();

        // Don't render if disabled
        if mode == AgentMode::Disabled {
            return div().id("agent-workspace-disabled").h_0().w_0();
        }

        div()
            .id("agent-workspace")
            .w_full()
            .bg(colors.surface)
            .border_t_1()
            .border_color(colors.border)
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(colors.border)
                    // Left: Status and mode
                    .child(header::render_header(mode, task_description, &colors))
                    // Right: Progress and controls
                    .child(controls::render_controls(self, &colors, cx)),
            )
            // Error message
            .when(error.is_some(), |d| {
                d.child(
                    div()
                        .px_3()
                        .py_2()
                        .bg(colors.error.opacity(0.1))
                        .border_b_1()
                        .border_color(colors.error.opacity(0.3))
                        .child(
                            div()
                                .text_sm()
                                .text_color(colors.error)
                                .child(error.unwrap()),
                        ),
                )
            })
            // Pending approval
            .when(has_pending_approval, |d| {
                let approval = self.pending_approval.as_ref().unwrap();
                d.child(render_approval_prompt(approval, &colors, cx))
            })
            // Expanded content (logs)
            .when(is_expanded && !self.logs.is_empty(), |d| {
                d.child(logs::render_logs(&self.logs, &colors))
            })
    }
}
