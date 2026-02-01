//! Getter methods for AgentExecutor

use crate::agent::planner::Plan;
use crate::agent::task::TaskTree;

use super::executor::AgentExecutor;
use super::super::types::ExecutorState;

impl AgentExecutor {
    /// Get current state
    pub fn state(&self) -> ExecutorState {
        self.state
    }

    /// Get current plan
    pub fn current_plan(&self) -> Option<&Plan> {
        self.current_plan.as_ref()
    }

    /// Get task tree
    pub fn task_tree(&self) -> &TaskTree {
        &self.task_tree
    }

    /// Get completed steps
    pub fn completed_steps(&self) -> &[usize] {
        &self.completed_steps
    }

    /// Get progress as (completed, total)
    pub fn progress(&self) -> (usize, usize) {
        let total = self.current_plan.as_ref().map(|p| p.steps.len()).unwrap_or(0);
        (self.completed_steps.len(), total)
    }

    /// Get the next runnable step (cloned to avoid borrow issues)
    pub(super) fn get_next_runnable_step(&self) -> Option<crate::agent::planner::PlanStep> {
        let plan = self.current_plan.as_ref()?;
        let runnable = plan.runnable_steps(&self.completed_steps);
        runnable.first().cloned().cloned()
    }
}
