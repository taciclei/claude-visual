//! Stepper component implementation

use std::sync::Arc;
use gpui::*;
use crate::app::state::AppState;
use super::types::*;

/// Stepper component
pub struct Stepper {
    pub(super) app_state: Arc<AppState>,
    /// Steps
    pub(super) steps: Vec<Step>,
    /// Current active step index
    pub(super) current: usize,
    /// Orientation
    pub(super) orientation: StepperOrientation,
    /// Allow clicking to navigate
    pub(super) clickable: bool,
    /// Show step numbers
    pub(super) show_numbers: bool,
    /// Allow going back
    pub(super) allow_back: bool,
}

impl Stepper {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            steps: Vec::new(),
            current: 0,
            orientation: StepperOrientation::default(),
            clickable: true,
            show_numbers: true,
            allow_back: true,
        }
    }

    /// Set steps
    pub fn set_steps(&mut self, steps: Vec<Step>, cx: &mut Context<Self>) {
        self.steps = steps;
        self.update_statuses(cx);
        cx.notify();
    }

    /// Set current step
    pub fn set_current(&mut self, index: usize, cx: &mut Context<Self>) {
        let old = self.current;
        self.current = index.min(self.steps.len().saturating_sub(1));
        if old != self.current {
            cx.emit(StepperEvent::StepChanged { from: old, to: self.current });
            self.update_statuses(cx);
        }
        cx.notify();
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: StepperOrientation, cx: &mut Context<Self>) {
        self.orientation = orientation;
        cx.notify();
    }

    /// Set clickable
    pub fn set_clickable(&mut self, clickable: bool, cx: &mut Context<Self>) {
        self.clickable = clickable;
        cx.notify();
    }

    /// Set show numbers
    pub fn set_show_numbers(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_numbers = show;
        cx.notify();
    }

    /// Set allow back
    pub fn set_allow_back(&mut self, allow: bool, cx: &mut Context<Self>) {
        self.allow_back = allow;
        cx.notify();
    }

    /// Go to next step
    pub fn next(&mut self, cx: &mut Context<Self>) {
        if self.current < self.steps.len() - 1 {
            self.set_current(self.current + 1, cx);
        } else {
            cx.emit(StepperEvent::Completed);
        }
    }

    /// Go to previous step
    pub fn prev(&mut self, cx: &mut Context<Self>) {
        if self.allow_back && self.current > 0 {
            self.set_current(self.current - 1, cx);
        }
    }

    /// Go to specific step
    pub fn go_to(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.clickable {
            // Can only go to completed steps or the next step
            if index <= self.current || (self.allow_back && index < self.steps.len()) {
                self.set_current(index, cx);
            }
        }
    }

    /// Mark current step as completed and advance
    pub fn complete_current(&mut self, cx: &mut Context<Self>) {
        if self.current < self.steps.len() {
            self.steps[self.current].status = StepStatus::Completed;
            self.next(cx);
        }
    }

    /// Mark current step as error
    pub fn error_current(&mut self, cx: &mut Context<Self>) {
        if self.current < self.steps.len() {
            self.steps[self.current].status = StepStatus::Error;
            cx.notify();
        }
    }

    /// Reset to first step
    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.current = 0;
        for step in &mut self.steps {
            step.status = StepStatus::Pending;
        }
        self.update_statuses(cx);
        cx.notify();
    }

    /// Get current step index
    pub fn current(&self) -> usize {
        self.current
    }

    /// Check if on last step
    pub fn is_last(&self) -> bool {
        self.current >= self.steps.len().saturating_sub(1)
    }

    /// Check if on first step
    pub fn is_first(&self) -> bool {
        self.current == 0
    }

    /// Update step statuses based on current
    pub(super) fn update_statuses(&mut self, _cx: &mut Context<Self>) {
        for (i, step) in self.steps.iter_mut().enumerate() {
            if step.status != StepStatus::Error && step.status != StepStatus::Skipped {
                if i < self.current {
                    step.status = StepStatus::Completed;
                } else if i == self.current {
                    step.status = StepStatus::Active;
                } else {
                    step.status = StepStatus::Pending;
                }
            }
        }
    }
}

impl EventEmitter<StepperEvent> for Stepper {}
