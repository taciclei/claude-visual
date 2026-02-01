//! Rendering implementation for tool progress panel

use gpui::*;
use gpui::prelude::*;

use super::core::ToolProgressPanel;
use super::types::{ExecutionPhase, ToolProgressPanelEvent};

impl EventEmitter<ToolProgressPanelEvent> for ToolProgressPanel {}

impl Focusable for ToolProgressPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ToolProgressPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let active_count = self.active_count();

        // Don't render if no executions
        if self.executions.is_empty() {
            return div().into_any_element();
        }

        // Get executions in order
        let executions: Vec<_> = self
            .execution_order
            .iter()
            .filter_map(|id| self.executions.get(id).cloned())
            .collect();

        div()
            .w_full()
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface)
            .overflow_hidden()
            // Header
            .child(
                div()
                    .id("progress-header")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(theme.colors.surface)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_collapsed(cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Collapse indicator
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(if self.collapsed { "▸" } else { "▾" }),
                            )
                            // Title
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child("Tool Executions"),
                            )
                            // Active count badge
                            .when(active_count > 0, |d| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.colors.accent)
                                        .text_xs()
                                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                        .child(format!("{}", active_count)),
                                )
                            }),
                    )
                    // Clear all button
                    .when(executions.iter().any(|e| !e.phase.is_active()), |d| {
                        d.child(
                            div()
                                .id("clear-all-btn")
                                .px_2()
                                .py_1()
                                .rounded_sm()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .hover(|style| {
                                    style
                                        .bg(theme.colors.surface_hover)
                                        .text_color(theme.colors.text)
                                })
                                .cursor_pointer()
                                .on_click(cx.listener(|_this, _, _window, cx| {
                                    cx.emit(ToolProgressPanelEvent::DismissAll);
                                }))
                                .child("Clear Done"),
                        )
                    }),
            )
            // Execution list
            .when(!self.collapsed, |this| {
                this.child(
                    gpui::div()
                        .max_h(px(300.0))
                        .id("scroll-progress-executions")
                        .overflow_y_scroll()
                        .children(executions.into_iter().map(|execution| {
                            let id = execution.id.clone();
                            let is_active = execution.phase.is_active();

                            // Phase color
                            let phase_color = match execution.phase {
                                ExecutionPhase::Preparing => theme.colors.text_muted,
                                ExecutionPhase::Executing => theme.colors.accent,
                                ExecutionPhase::Processing => theme.colors.accent,
                                ExecutionPhase::Completed => theme.colors.success,
                                ExecutionPhase::Failed => theme.colors.error,
                                ExecutionPhase::Cancelled => theme.colors.warning,
                            };

                            div()
                                .id(ElementId::Name(format!("exec-{}", id).into()))
                                .px_3()
                                .py_2()
                                .border_b_1()
                                .border_color(theme.colors.border)
                                .hover(|style| style.bg(theme.colors.surface_hover))
                                // Main row
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
                                                // Status indicator
                                                .child(
                                                    div()
                                                        .size(px(8.0))
                                                        .rounded_full()
                                                        .bg(phase_color)
                                                        .when(is_active, |d| {
                                                            // Pulsing animation hint
                                                            d.border_1()
                                                                .border_color(phase_color.opacity(0.5))
                                                        }),
                                                )
                                                // Tool name
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.colors.text)
                                                        .child(execution.tool_name.clone()),
                                                )
                                                // Server badge
                                                .child(
                                                    div()
                                                        .px_1()
                                                        .py_0p5()
                                                        .rounded_sm()
                                                        .bg(theme.colors.background)
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(execution.server_name.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                // Elapsed time
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(execution.elapsed_str()),
                                                )
                                                // Cancel button for active executions
                                                .when(is_active, |d| {
                                                    let cancel_id = id.clone();
                                                    d.child(
                                                        div()
                                                            .id(ElementId::Name(
                                                                format!("cancel-{}", id).into(),
                                                            ))
                                                            .px_2()
                                                            .py_0p5()
                                                            .rounded_sm()
                                                            .text_xs()
                                                            .text_color(theme.colors.error)
                                                            .hover(|style| {
                                                                style
                                                                    .bg(theme.colors.error.opacity(0.1))
                                                            })
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(
                                                                move |_this, _, _window, cx| {
                                                                    cx.emit(
                                                                        ToolProgressPanelEvent::Cancel(
                                                                            cancel_id.clone(),
                                                                        ),
                                                                    );
                                                                },
                                                            ))
                                                            .child("Cancel"),
                                                    )
                                                })
                                                // Dismiss button for completed
                                                .when(!is_active, |d| {
                                                    let dismiss_id = id.clone();
                                                    d.child(
                                                        div()
                                                            .id(ElementId::Name(
                                                                format!("dismiss-{}", id).into(),
                                                            ))
                                                            .px_2()
                                                            .py_0p5()
                                                            .rounded_sm()
                                                            .text_xs()
                                                            .text_color(theme.colors.text_muted)
                                                            .hover(|style| {
                                                                style.bg(theme.colors.surface_hover)
                                                            })
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(
                                                                move |_this, _, _window, cx| {
                                                                    cx.emit(
                                                                        ToolProgressPanelEvent::Dismiss(
                                                                            dismiss_id.clone(),
                                                                        ),
                                                                    );
                                                                },
                                                            ))
                                                            .child("×"),
                                                    )
                                                }),
                                        ),
                                )
                                // Progress bar (if available)
                                .when_some(execution.progress, |d, progress| {
                                    d.child(
                                        div()
                                            .mt_2()
                                            .h(px(4.0))
                                            .w_full()
                                            .rounded_full()
                                            .bg(theme.colors.background)
                                            .child(
                                                div()
                                                    .h_full()
                                                    .w(relative(progress as f32 / 100.0))
                                                    .rounded_full()
                                                    .bg(phase_color),
                                            ),
                                    )
                                })
                                // Status message
                                .when_some(execution.status_message.clone(), |d, message| {
                                    d.child(
                                        div()
                                            .mt_1()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(message),
                                    )
                                })
                                // Error message
                                .when_some(execution.error.clone(), |d, error| {
                                    d.child(
                                        div()
                                            .mt_1()
                                            .text_xs()
                                            .text_color(theme.colors.error)
                                            .child(error),
                                    )
                                })
                        })),
                )
            })
            .into_any_element()
    }
}
