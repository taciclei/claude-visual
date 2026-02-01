//! Rendering methods for PlanView

use gpui::*;
use gpui::prelude::*;
use crate::agent::planner::{Plan, PlanStep};
use super::PlanView;

impl PlanView {
    /// Get risk level color
    fn risk_color(&self, risk: u8) -> Hsla {
        if risk > 7 {
            self.theme.colors.error
        } else if risk > 4 {
            self.theme.colors.warning
        } else {
            self.theme.colors.success
        }
    }

    /// Render a single step
    pub(super) fn render_step(&self, step: &PlanStep, cx: &mut Context<Self>) -> impl IntoElement {
        let is_completed = self.completed_steps.contains(&step.step_number);
        let is_current = self.current_step == Some(step.step_number);
        let is_expanded = self.expanded_steps.contains(&step.step_number);
        let step_number = step.step_number;

        let status_icon = if is_completed {
            "✅"
        } else if is_current {
            "🔄"
        } else {
            "⏳"
        };

        let border_color = if is_current {
            self.theme.colors.accent
        } else if is_completed {
            self.theme.colors.success
        } else {
            self.theme.colors.border
        };

        let theme_surface = self.theme.colors.surface;
        let theme_text_muted = self.theme.colors.text_muted;
        let theme_text = self.theme.colors.text;
        let theme_accent = self.theme.colors.accent;
        let theme_warning = self.theme.colors.warning;
        let theme_surface_hover = self.theme.colors.surface_hover;
        let risk_color = self.risk_color(step.risk_level);

        let on_click = cx.listener(move |this, _, _window, cx| {
            this.toggle_step(step_number, cx);
        });

        div()
            .flex()
            .flex_col()
            .p_3()
            .bg(theme_surface)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .child(
                // Step header
                div()
                    .id(ElementId::Name(format!("step-{}", step_number).into()))
                    .flex()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .on_click(on_click)
                    .child(
                        // Expand indicator
                        div()
                            .w_4()
                            .h_4()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(theme_text_muted)
                            .child(if is_expanded { "▼" } else { "▶" })
                    )
                    .child(
                        // Status icon
                        div()
                            .text_sm()
                            .child(status_icon)
                    )
                    .child(
                        // Step number
                        div()
                            .text_xs()
                            .text_color(theme_text_muted)
                            .child(format!("#{}", step.step_number))
                    )
                    .child(
                        // Step title
                        div()
                            .flex_1()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(if is_completed {
                                theme_text_muted
                            } else {
                                theme_text
                            })
                            .when(is_completed, |el| el.line_through())
                            .child(step.title.clone())
                    )
                    .child(
                        // Risk level badge
                        div()
                            .px_2()
                            .py(px(2.0))
                            .bg(risk_color.opacity(0.2))
                            .text_color(risk_color)
                            .text_xs()
                            .rounded_sm()
                            .child(format!("Risk: {}", step.risk_level))
                    )
                    .child(
                        // Approval badge
                        if step.requires_approval {
                            div()
                                .px_2()
                                .py(px(2.0))
                                .bg(theme_warning.opacity(0.2))
                                .text_color(theme_warning)
                                .text_xs()
                                .rounded_sm()
                                .child("🔐")
                        } else {
                            div()
                        }
                    )
            )
            .when(is_expanded, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .mt_2()
                        .pl_6()
                        .child(
                            // Description
                            div()
                                .text_sm()
                                .text_color(theme_text_muted)
                                .child(step.description.clone())
                        )
                        .child(
                            // Tools
                            if !step.tools.is_empty() {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme_text_muted)
                                            .child("Tools:")
                                    )
                                    .children(
                                        step.tools.iter().map(|tool| {
                                            div()
                                                .px_2()
                                                .py(px(2.0))
                                                .bg(theme_accent.opacity(0.2))
                                                .text_color(theme_accent)
                                                .text_xs()
                                                .rounded_sm()
                                                .child(tool.clone())
                                        })
                                    )
                            } else {
                                div()
                            }
                        )
                        .child(
                            // Dependencies
                            if !step.depends_on.is_empty() {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme_text_muted)
                                            .child("Depends on:")
                                    )
                                    .children(
                                        step.depends_on.iter().map(|dep| {
                                            div()
                                                .px_2()
                                                .py(px(2.0))
                                                .bg(theme_surface_hover)
                                                .text_color(theme_text_muted)
                                                .text_xs()
                                                .rounded_sm()
                                                .child(format!("#{}", dep))
                                        })
                                    )
                            } else {
                                div()
                            }
                        )
                )
            })
    }

    /// Render plan header
    pub(super) fn render_header(&self, plan: &Plan) -> impl IntoElement {
        let completed = self.completed_steps.len();
        let total = plan.steps.len();
        let percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        let theme_border = self.theme.colors.border;
        let theme_text = self.theme.colors.text;
        let theme_text_muted = self.theme.colors.text_muted;
        let theme_surface = self.theme.colors.surface;
        let theme_success = self.theme.colors.success;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .border_b_1()
            .border_color(theme_border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme_text)
                                    .child(plan.title.clone())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme_text_muted)
                                    .child(plan.description.clone())
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme_text_muted)
                            .child(format!("{}/{} steps ({:.0}%)", completed, total, percentage))
                    )
            )
            .child(
                // Progress bar
                div()
                    .w_full()
                    .h_1()
                    .bg(theme_surface)
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .bg(theme_success)
                            .rounded_full()
                            .w(relative(percentage / 100.0))
                    )
            )
    }
}
