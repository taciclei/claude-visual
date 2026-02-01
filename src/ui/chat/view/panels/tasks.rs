//! Tasks panel render functions

use gpui::*;
use gpui::prelude::*;

use crate::ui::pct;
use super::super::core::ChatView;
use super::super::types::ChatViewEvent;

impl ChatView {
    pub fn render_tasks_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let tasks = &self.active_tasks;

        div()
            .id("tasks-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_tasks_panel(cx);
            }))
            .child(
                div()
                    .id("tasks-panel")
                    .w(px(550.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child("⚡"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Active Tasks")
                                    )
                                    .when(!tasks.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .px_2()
                                                .py_px()
                                                .rounded_full()
                                                .bg(theme.colors.accent.opacity(0.2))
                                                .text_xs()
                                                .text_color(theme.colors.accent)
                                                .child(format!("{} running", tasks.len()))
                                        )
                                    })
                            )
                            .child(
                                div()
                                    .id("close-tasks-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tasks_panel(cx);
                                    }))
                                    .child("×")
                            )
                    )
                    // Tasks list
                    .child(
                        div()
                            .id("tasks-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(tasks.is_empty(), |d| {
                                d.child(self.render_empty_tasks_state(theme, cx))
                            })
                            .children(tasks.iter().enumerate().map(|(idx, task)| {
                                let elapsed = chrono::Utc::now().signed_duration_since(task.started_at);
                                let elapsed_str = if elapsed.num_seconds() < 60 {
                                    format!("{}s", elapsed.num_seconds())
                                } else {
                                    format!("{}m {}s", elapsed.num_minutes(), elapsed.num_seconds() % 60)
                                };

                                div()
                                    .id(SharedString::from(format!("task-{}", idx)))
                                    .px_4()
                                    .py_3()
                                    .border_b_1()
                                    .border_color(theme.colors.border.opacity(0.5))
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    // Task header
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    // Animated spinner
                                                    .child(
                                                        div()
                                                            .size(px(12.0))
                                                            .rounded_full()
                                                            .bg(theme.colors.accent)
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.colors.text)
                                                            .child(task.description.clone())
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(elapsed_str)
                                            )
                                    )
                                    // Progress bar (if available)
                                    .when_some(task.progress, |d, progress| {
                                        d.child(
                                            div()
                                                .w_full()
                                                .h(px(4.0))
                                                .rounded_full()
                                                .bg(theme.colors.border)
                                                .child(
                                                    div()
                                                        .h_full()
                                                        .w(pct(progress as f32))
                                                        .rounded_full()
                                                        .bg(theme.colors.accent)
                                                )
                                        )
                                    })
                                    // Status message (if available)
                                    .when_some(task.status.clone(), |d, status| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(status)
                                        )
                                    })
                            }))
                    )
                    // Quick skills footer
                    .child(self.render_task_quick_skills(theme, cx))
            )
    }

    /// Render empty tasks state with skill suggestions
    fn render_empty_tasks_state(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_4()
            .py_6()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_2xl().child("✨"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("No active tasks")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.7))
                            .child("Start a skill to run parallel agents")
                    )
            )
            // Suggested skills to start
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .w_full()
                    .max_w(px(300.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text_muted)
                            .child("Start with a skill:")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .justify_center()
                            // APEX - runs multiple agents
                            .child(
                                div()
                                    .id("task-start-apex")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.accent.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.accent.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.accent)
                                    .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tasks_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/apex".to_string()));
                                    }))
                                    .child("⚡")
                                    .child("APEX")
                            )
                            // Explore - parallel search
                            .child(
                                div()
                                    .id("task-start-explore")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.info.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.info.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.info)
                                    .hover(|s| s.bg(theme.colors.info.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tasks_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                                    }))
                                    .child("🔍")
                                    .child("Explore")
                            )
                            // Review - parallel code review
                            .child(
                                div()
                                    .id("task-start-review")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.warning.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.warning.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.warning)
                                    .hover(|s| s.bg(theme.colors.warning.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tasks_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/review".to_string()));
                                    }))
                                    .child("👀")
                                    .child("Review")
                            )
                            // Brainstorm - research agents
                            .child(
                                div()
                                    .id("task-start-brainstorm")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.success.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.success.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.success)
                                    .hover(|s| s.bg(theme.colors.success.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tasks_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/brainstorm".to_string()));
                                    }))
                                    .child("💡")
                                    .child("Brainstorm")
                            )
                    )
            )
    }

    /// Render quick skills footer for the tasks panel
    fn render_task_quick_skills(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            // Skill buttons
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Quick:")
                    )
                    // Refactor (parallel file processing)
                    .child(
                        div()
                            .id("task-skill-refactor")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .text_xs()
                            .text_color(theme.colors.text)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_tasks_panel(cx);
                                cx.emit(ChatViewEvent::Submit("/refactor".to_string()));
                            }))
                            .child("🔄")
                            .child("Refactor")
                    )
                    // Docs (parallel doc research)
                    .child(
                        div()
                            .id("task-skill-docs")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(theme.colors.surface_hover)
                            .text_xs()
                            .text_color(theme.colors.text)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_tasks_panel(cx);
                                cx.emit(ChatViewEvent::Submit("/docs".to_string()));
                            }))
                            .child("📚")
                            .child("Docs")
                    )
            )
            // Task list button
            .child(
                div()
                    .id("task-show-all")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_tasks_panel(cx);
                        cx.emit(ChatViewEvent::Submit("/tasks".to_string()));
                    }))
                    .child("View all tasks →")
            )
    }

}
