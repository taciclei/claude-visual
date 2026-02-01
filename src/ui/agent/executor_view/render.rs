//! Main render implementation

use super::ExecutorView;
use crate::agent::executor::ExecutorState;
use gpui::prelude::*;
use gpui::*;

impl Render for ExecutorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.stats.state;
        let percentage = self.stats.completion_percentage();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(self.theme.colors.surface)
            .border_1()
            .border_color(self.theme.colors.border)
            .rounded_lg()
            .child(
                // Header with state
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_lg().child(state.icon()))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(self.theme.colors.text)
                                    .child(format!("{:?}", state)),
                            ),
                    )
                    .child(
                        // Duration
                        if let Some(ms) = self.stats.duration_ms {
                            div()
                                .text_xs()
                                .text_color(self.theme.colors.text_muted)
                                .child(Self::format_duration(ms))
                        } else {
                            div()
                        },
                    ),
            )
            .child(
                // Progress section
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(self.theme.colors.text_muted)
                                    .child("Progress"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(self.theme.colors.text_muted)
                                    .child(format!(
                                        "{}/{} ({:.0}%)",
                                        self.stats.completed_steps,
                                        self.stats.total_steps,
                                        percentage
                                    )),
                            ),
                    )
                    .child(
                        // Progress bar
                        div()
                            .w_full()
                            .h_2()
                            .bg(self.theme.colors.background)
                            .rounded_full()
                            .overflow_hidden()
                            .child(
                                div()
                                    .h_full()
                                    .bg(if state == ExecutorState::Failed {
                                        self.theme.colors.error
                                    } else if state == ExecutorState::Completed {
                                        self.theme.colors.success
                                    } else {
                                        self.theme.colors.accent
                                    })
                                    .rounded_full()
                                    .w(relative(percentage / 100.0)),
                            ),
                    ),
            )
            .child(self.render_approval_prompt(cx))
            .child(self.render_controls(cx))
    }
}

impl EventEmitter<super::ExecutorViewEvent> for ExecutorView {}
