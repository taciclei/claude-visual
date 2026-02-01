//! Helper methods for AgentExecutor

use crate::agent::task::TaskTree;

use super::super::types::{ExecutorEvent, ExecutorState, ExecutorStats};
use super::executor::AgentExecutor;

impl AgentExecutor {
    /// Check if a step should be auto-approved
    pub(super) fn should_auto_approve(&self, step: &crate::agent::planner::PlanStep) -> bool {
        self.auto_approve_low_risk && step.risk_level <= self.auto_approve_threshold
    }

    /// Set state and emit event
    pub(super) fn set_state(&mut self, state: ExecutorState) {
        if self.state != state {
            self.state = state;
            self.emit_event(ExecutorEvent::StateChanged(state));
        }
    }

    /// Emit an event
    pub(super) fn emit_event(&self, event: ExecutorEvent) {
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(event);
        }
    }

    /// Reset executor to initial state
    pub fn reset(&mut self) {
        self.state = ExecutorState::Idle;
        self.current_plan = None;
        self.task_tree = TaskTree::new();
        self.completed_steps.clear();
        self.started_at = None;
    }

    /// Get execution statistics
    pub fn stats(&self) -> ExecutorStats {
        let (completed, total) = self.progress();
        ExecutorStats {
            state: self.state,
            completed_steps: completed,
            total_steps: total,
            duration_ms: self.started_at.map(|s| s.elapsed().as_millis() as u64),
        }
    }
}
