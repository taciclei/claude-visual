//! Task management operations

use gpui::*;
use crate::agent::Plan;
use super::state::AgentWorkspace;
use super::types::*;

impl AgentWorkspace {
    /// Start a new task
    pub fn start_task(&mut self, description: impl Into<String>, cx: &mut Context<Self>) {
        let description = description.into();
        self.task_description = Some(description.clone());
        self.mode = AgentMode::Planning;
        self.progress = 0.0;
        self.current_step = 0;
        self.error = None;
        self.logs.clear();
        self.add_log(LogLevel::Info, format!("Starting task: {}", description));
        cx.emit(AgentWorkspaceEvent::TaskStarted(description));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.notify();
    }

    /// Set the generated plan
    pub fn set_plan(&mut self, plan: Plan, cx: &mut Context<Self>) {
        let step_count = plan.steps.len();
        self.total_steps = step_count;
        self.plan = Some(plan.clone());
        self.mode = AgentMode::Executing;
        self.add_log(LogLevel::Success, format!("Plan generated with {} steps", step_count));
        cx.emit(AgentWorkspaceEvent::PlanGenerated(format!("{} steps", step_count)));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.notify();
    }

    /// Mark current step as completed
    pub fn complete_step(&mut self, cx: &mut Context<Self>) {
        let step_name = self.plan
            .as_ref()
            .and_then(|p| p.steps.get(self.current_step))
            .map(|s| s.description.clone())
            .unwrap_or_else(|| format!("Step {}", self.current_step + 1));

        self.add_log(LogLevel::Success, format!("Completed: {}", step_name));
        cx.emit(AgentWorkspaceEvent::StepCompleted(self.current_step, step_name));

        self.current_step += 1;
        self.update_progress();

        if self.current_step >= self.total_steps {
            self.complete_task(cx);
        } else {
            cx.notify();
        }
    }

    /// Complete the task
    pub(crate) fn complete_task(&mut self, cx: &mut Context<Self>) {
        let task_desc = self.task_description.clone().unwrap_or_default();
        self.mode = AgentMode::Completed;
        self.progress = 100.0;
        self.add_log(LogLevel::Success, "Task completed successfully");
        cx.emit(AgentWorkspaceEvent::TaskCompleted(task_desc));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.emit(AgentWorkspaceEvent::ShowNotification(
            "Task completed".to_string(),
            NotificationType::Success,
        ));
        cx.notify();
    }

    /// Fail the task
    pub fn fail_task(&mut self, error: impl Into<String>, cx: &mut Context<Self>) {
        let error = error.into();
        let task_desc = self.task_description.clone().unwrap_or_default();
        self.mode = AgentMode::Failed;
        self.error = Some(error.clone());
        self.add_log(LogLevel::Error, format!("Task failed: {}", error));
        cx.emit(AgentWorkspaceEvent::TaskFailed(task_desc, error.clone()));
        cx.emit(AgentWorkspaceEvent::ModeChanged(self.mode));
        cx.emit(AgentWorkspaceEvent::ShowNotification(
            format!("Task failed: {}", error),
            NotificationType::Error,
        ));
        cx.notify();
    }
}
