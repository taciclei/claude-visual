//! State management for PlanView

use gpui::*;
use crate::agent::planner::Plan;
use super::PlanView;

impl PlanView {
    /// Set the plan to display
    pub fn set_plan(&mut self, plan: Option<Plan>, cx: &mut Context<Self>) {
        self.plan = plan;
        self.completed_steps.clear();
        self.current_step = None;
        self.expanded_steps.clear();
        cx.notify();
    }

    /// Mark a step as completed
    pub fn mark_completed(&mut self, step_number: usize, cx: &mut Context<Self>) {
        if !self.completed_steps.contains(&step_number) {
            self.completed_steps.push(step_number);
            cx.notify();
        }
    }

    /// Set the current executing step
    pub fn set_current_step(&mut self, step_number: Option<usize>, cx: &mut Context<Self>) {
        self.current_step = step_number;
        cx.notify();
    }

    /// Toggle step expansion
    pub fn toggle_step(&mut self, step_number: usize, cx: &mut Context<Self>) {
        if self.expanded_steps.contains(&step_number) {
            self.expanded_steps.retain(|&s| s != step_number);
        } else {
            self.expanded_steps.push(step_number);
        }
        cx.notify();
    }
}
