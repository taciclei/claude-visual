//! Control buttons rendering

use super::super::state::AgentWorkspace;
use super::super::types::*;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

pub(crate) fn render_controls(
    workspace: &AgentWorkspace,
    colors: &SimpleColors,
    cx: &mut Context<AgentWorkspace>,
) -> impl IntoElement {
    let mode = workspace.mode;
    let progress = workspace.progress;
    let current_step = workspace.current_step;
    let total_steps = workspace.total_steps;
    let is_expanded = workspace.is_expanded;
    let has_pending_approval = workspace.pending_approval.is_some();

    // Copy colors for closures
    let colors_warning = colors.warning;
    let colors_success = colors.success;
    let colors_error = colors.error;
    let colors_text_muted = colors.text_muted;
    let colors_hover = colors.hover;
    let colors_border = colors.border;
    let colors_accent = colors.accent;

    // Extract listeners
    let on_pause = cx.listener(|this, _, _window, cx| {
        this.pause(cx);
    });
    let on_resume = cx.listener(|this, _, _window, cx| {
        this.resume(cx);
    });
    let on_cancel = cx.listener(|this, _, _window, cx| {
        this.cancel(cx);
    });
    let on_toggle = cx.listener(|this, _, _window, cx| {
        this.toggle_expanded(cx);
    });

    div()
        .flex()
        .items_center()
        .gap_3()
        // Progress
        .when(total_steps > 0, |d| {
            d.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w(px(100.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(colors_border)
                            .overflow_hidden()
                            .child(
                                div()
                                    .h_full()
                                    .w(pct(progress))
                                    .rounded_full()
                                    .bg(colors_accent),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors_text_muted)
                            .child(format!("{}/{}", current_step, total_steps)),
                    ),
            )
        })
        // Pause/Resume button
        .when(mode == AgentMode::Executing, |d| {
            d.child(
                div()
                    .id("agent-pause")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(colors_warning)
                    .border_1()
                    .border_color(colors_warning.opacity(0.3))
                    .cursor_pointer()
                    .hover(|s| s.bg(colors_warning.opacity(0.1)))
                    .on_click(on_pause)
                    .child("Pause"),
            )
        })
        .when(mode == AgentMode::Paused && !has_pending_approval, |d| {
            d.child(
                div()
                    .id("agent-resume")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(colors_success)
                    .border_1()
                    .border_color(colors_success.opacity(0.3))
                    .cursor_pointer()
                    .hover(|s| s.bg(colors_success.opacity(0.1)))
                    .on_click(on_resume)
                    .child("Resume"),
            )
        })
        // Cancel button
        .when(
            mode == AgentMode::Executing || mode == AgentMode::Paused,
            |d| {
                d.child(
                    div()
                        .id("agent-cancel")
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .text_xs()
                        .text_color(colors_error)
                        .border_1()
                        .border_color(colors_error.opacity(0.3))
                        .cursor_pointer()
                        .hover(|s| s.bg(colors_error.opacity(0.1)))
                        .on_click(on_cancel)
                        .child("Cancel"),
                )
            },
        )
        // Expand/collapse button
        .child(
            div()
                .id("agent-toggle")
                .px_2()
                .py_1()
                .rounded_sm()
                .text_xs()
                .text_color(colors_text_muted)
                .cursor_pointer()
                .hover(|s| s.bg(colors_hover))
                .on_click(on_toggle)
                .child(if is_expanded { "Collapse" } else { "Expand" }),
        )
}
