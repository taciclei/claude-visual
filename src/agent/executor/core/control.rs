//! Control flow methods (pause, resume, cancel, approve, reject)

use super::executor::AgentExecutor;
use super::super::types::{ExecutorEvent, ExecutorState, PlanResult};

impl AgentExecutor {
    /// Pause execution
    pub async fn pause(&mut self) {
        if self.state.can_pause() {
            *self.pause_requested.lock().await = true;
        }
    }

    /// Resume execution
    pub async fn resume(&mut self) -> Result<PlanResult, String> {
        if !self.state.can_resume() {
            return Err("Cannot resume from current state".to_string());
        }

        *self.pause_requested.lock().await = false;
        self.start().await
    }

    /// Cancel execution
    pub async fn cancel(&mut self) {
        *self.cancel_requested.lock().await = true;
    }

    /// Approve current step and continue
    pub async fn approve(&mut self) -> Result<PlanResult, String> {
        if self.state != ExecutorState::WaitingApproval {
            return Err("Not waiting for approval".to_string());
        }

        // Find the step that was waiting for approval and clone it
        let step_to_execute: Option<crate::agent::planner::PlanStep> = self.get_next_runnable_step();

        if step_to_execute.is_none() && self.current_plan.is_none() {
            return Err("No plan".to_string());
        }

        // Execute the approved step
        self.set_state(ExecutorState::Running);
        if let Some(step) = step_to_execute {
            self.execute_step(&step).await?;
        }

        // Continue execution
        self.start().await
    }

    /// Reject current step and cancel
    pub async fn reject(&mut self, reason: &str) {
        self.set_state(ExecutorState::Cancelled);
        self.emit_event(ExecutorEvent::TaskFailed(
            "approval".to_string(),
            format!("User rejected: {}", reason),
        ));
    }
}
