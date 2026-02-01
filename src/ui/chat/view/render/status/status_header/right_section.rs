//! Right section rendering (tasks and indicators)

use super::super::super::super::core::ChatView;
use super::helpers::{format_elapsed_time, get_spinner, shorten_with_ellipsis};
use crate::app::theme::Theme;
use crate::claude::message::SessionInfo;
use crate::ui::chat::view::types::ActiveTask;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render a single active task item
    pub(super) fn render_active_task(
        &self,
        idx: usize,
        task: &ActiveTask,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let elapsed = chrono::Utc::now().signed_duration_since(task.started_at);
        let elapsed_str = format_elapsed_time(elapsed.num_seconds());
        let spinner = get_spinner(self.streaming.streaming_dots);
        let task_id = task.task_id.clone();

        div()
            .id(SharedString::from(format!("active-task-{}", idx)))
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.info.opacity(0.1))
            .border_1()
            .border_color(theme.colors.info.opacity(0.2))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.info.opacity(0.15)))
            // Animated spinner
            .child(div().text_xs().text_color(theme.colors.info).child(spinner))
            // Task description
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .max_w(px(150.0))
                    .overflow_hidden()
                    .child(task.description.clone()),
            )
            // Progress bar (if available)
            .when_some(task.progress, |d, progress| {
                d.child(
                    div()
                        .w(px(40.0))
                        .h(px(4.0))
                        .rounded_sm()
                        .bg(theme.colors.info.opacity(0.2))
                        .child(
                            div()
                                .h_full()
                                .w(px(40.0 * progress as f32 / 100.0))
                                .rounded_sm()
                                .bg(theme.colors.info),
                        ),
                )
            })
            // Elapsed time
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(elapsed_str),
            )
            // Task ID badge (if available)
            .when_some(task_id.clone(), |d, id| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted.opacity(0.6))
                        .font_family("monospace")
                        .child(format!("#{}", &id[..id.len().min(6)])),
                )
            })
            // Cancel button
            .child({
                let cancel_task_id = task.task_id.clone();
                div()
                    .id(SharedString::from(format!("cancel-task-{}", idx)))
                    .w(px(16.0))
                    .h(px(16.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.error.opacity(0.6))
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.error.opacity(0.2))
                            .text_color(theme.colors.error)
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.cancel_task(cancel_task_id.clone(), cx);
                    }))
                    .child("×")
            })
    }

    /// Render the session ID badge (right section version, when no tasks)
    pub(super) fn render_session_id_badge_right(&self, info: &SessionInfo, theme: &Theme) -> Div {
        let short_id = shorten_with_ellipsis(&info.session_id, 8);

        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("🔑"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child(short_id),
            )
    }

    /// Render the version badge
    pub(super) fn render_version_badge(&self, info: &SessionInfo, theme: &Theme) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child(format!("v{}", info.version)),
            )
    }

    /// Render the session cost indicator
    pub(super) fn render_cost_indicator(
        &self,
        cost: f64,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cost_color = if cost > 1.0 {
            theme.colors.error
        } else if cost > 0.1 {
            theme.colors.warning
        } else {
            theme.colors.success
        };

        div()
            .id("cost-indicator")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(cost_color.opacity(0.1))
            .cursor_pointer()
            .hover(|s| s.bg(cost_color.opacity(0.15)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_session_details(cx);
            }))
            .child(div().text_xs().text_color(cost_color).child("💰"))
            .child(
                div()
                    .text_xs()
                    .font_family("monospace")
                    .text_color(cost_color)
                    .child(format!("${:.4}", cost)),
            )
    }
}
