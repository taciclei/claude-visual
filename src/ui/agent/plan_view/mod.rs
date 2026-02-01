//! Plan View UI Component
//!
//! Displays an execution plan with steps, dependencies, and risk levels.

mod rendering;
mod state;

use gpui::prelude::*;
use gpui::*;

use crate::agent::planner::Plan;
use crate::app::theme::Theme;

/// Plan view component for displaying execution plans
pub struct PlanView {
    /// Plan to display
    plan: Option<Plan>,
    /// Completed step numbers
    completed_steps: Vec<usize>,
    /// Currently executing step number
    current_step: Option<usize>,
    /// Expanded steps (for showing details)
    expanded_steps: Vec<usize>,
    /// Theme
    theme: Theme,
}

impl PlanView {
    /// Create a new plan view
    pub fn new(theme: Theme) -> Self {
        Self {
            plan: None,
            completed_steps: Vec::new(),
            current_step: None,
            expanded_steps: Vec::new(),
            theme,
        }
    }
}

impl Render for PlanView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_background = self.theme.colors.background;
        let theme_text_muted = self.theme.colors.text_muted;

        div().flex().flex_col().h_full().bg(theme_background).child(
            if let Some(plan) = &self.plan {
                div()
                    .flex()
                    .flex_col()
                    .h_full()
                    .child(self.render_header(plan))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .flex_1()
                            .id("plan-scroll-container")
                            .overflow_y_scroll()
                            .p_4()
                            .children(plan.steps.iter().map(|step| self.render_step(step, cx))),
                    )
            } else {
                div().flex().items_center().justify_center().h_full().child(
                    div()
                        .text_sm()
                        .text_color(theme_text_muted)
                        .child("No plan loaded"),
                )
            },
        )
    }
}
