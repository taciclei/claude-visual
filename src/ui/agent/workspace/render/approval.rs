//! Approval prompt rendering

use super::super::state::AgentWorkspace;
use super::super::types::*;
use gpui::prelude::*;
use gpui::*;

pub(crate) fn render_approval_prompt(
    approval: &PendingApproval,
    colors: &SimpleColors,
    cx: &mut Context<AgentWorkspace>,
) -> impl IntoElement {
    // Copy colors for closures
    let colors_warning = colors.warning;
    let colors_text = colors.text;
    let colors_text_muted = colors.text_muted;
    let colors_success = colors.success;
    let colors_error = colors.error;

    // Extract listeners
    let on_approve = cx.listener(|this, _, _window, cx| {
        this.approve(cx);
    });
    let on_reject = cx.listener(|this, _, _window, cx| {
        this.reject("User rejected", cx);
    });

    div()
        .px_3()
        .py_3()
        .bg(colors_warning.opacity(0.05))
        .border_b_1()
        .border_color(colors_warning.opacity(0.3))
        .flex()
        .flex_col()
        .gap_2()
        // Header
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(colors_warning)
                        .child("Approval Required"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_sm()
                        .bg(colors_warning.opacity(0.2))
                        .text_xs()
                        .text_color(colors_warning)
                        .child(approval.risk_level.clone()),
                ),
        )
        // Description
        .child(
            div()
                .text_sm()
                .text_color(colors_text)
                .child(approval.step_description.clone()),
        )
        // Tool name
        .when(approval.tool_name.is_some(), |d| {
            d.child(
                div()
                    .text_xs()
                    .text_color(colors_text_muted)
                    .child(format!("Tool: {}", approval.tool_name.as_ref().unwrap())),
            )
        })
        // Buttons
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .mt_1()
                .child(
                    div()
                        .id("approve-step")
                        .px_3()
                        .py_1()
                        .rounded_sm()
                        .bg(colors_success)
                        .text_xs()
                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(colors_success.opacity(0.9)))
                        .on_click(on_approve)
                        .child("Approve"),
                )
                .child(
                    div()
                        .id("reject-step")
                        .px_3()
                        .py_1()
                        .rounded_sm()
                        .bg(colors_error)
                        .text_xs()
                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(colors_error.opacity(0.9)))
                        .on_click(on_reject)
                        .child("Reject"),
                ),
        )
}
