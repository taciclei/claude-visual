//! Control buttons rendering

use super::{ExecutorView, ExecutorViewEvent};
use crate::agent::executor::ExecutorState;
use gpui::prelude::*;
use gpui::*;

impl ExecutorView {
    /// Render control buttons based on state
    pub(super) fn render_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.stats.state;

        // Extract listeners before div chains
        let on_pause = cx.listener(|_this, _, _window, cx| {
            cx.emit(ExecutorViewEvent::Pause);
        });

        let on_start = cx.listener(|_this, _, _window, cx| {
            cx.emit(ExecutorViewEvent::Start);
        });

        let on_cancel = cx.listener(|_this, _, _window, cx| {
            cx.emit(ExecutorViewEvent::Cancel);
        });

        // Copy theme colors for move closures
        let warning_color = self.theme.colors.warning;
        let success_color = self.theme.colors.success;
        let error_color = self.theme.colors.error;
        let bg_color = self.theme.colors.background;

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                // Play/Pause button
                if state == ExecutorState::Running {
                    div()
                        .id("pause-btn")
                        .px_3()
                        .py_1()
                        .bg(warning_color)
                        .text_color(bg_color)
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .on_click(on_pause)
                        .child("⏸ Pause")
                } else if state.can_resume() || state == ExecutorState::Idle {
                    div()
                        .id("start-btn")
                        .px_3()
                        .py_1()
                        .bg(success_color)
                        .text_color(bg_color)
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .on_click(on_start)
                        .child(if state == ExecutorState::Idle {
                            "▶ Start"
                        } else {
                            "▶ Resume"
                        })
                } else {
                    div().id("btn-placeholder-1")
                },
            )
            .child(
                // Cancel button (only when running or paused)
                if state == ExecutorState::Running || state == ExecutorState::Paused {
                    div()
                        .id("cancel-btn")
                        .px_3()
                        .py_1()
                        .bg(error_color)
                        .text_color(bg_color)
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .on_click(on_cancel)
                        .child("✕ Cancel")
                } else {
                    div().id("btn-placeholder-2")
                },
            )
    }
}
