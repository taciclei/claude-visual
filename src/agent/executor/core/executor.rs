//! AgentExecutor struct definition

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::agent::planner::Plan;
use crate::agent::task::TaskTree;

use super::super::traits::ToolExecutor;
use super::super::types::ExecutorEvent;

/// Agent executor for running plans
pub struct AgentExecutor {
    /// Current state
    pub(super) state: super::super::types::ExecutorState,
    /// Current plan being executed
    pub(super) current_plan: Option<Plan>,
    /// Task tree for the current plan
    pub(super) task_tree: TaskTree,
    /// Completed step numbers
    pub(super) completed_steps: Vec<usize>,
    /// Event sender
    pub(super) event_tx: Option<mpsc::UnboundedSender<ExecutorEvent>>,
    /// Tool executor
    pub(super) tool_executor: Option<Arc<dyn ToolExecutor>>,
    /// Auto-approve low-risk steps
    pub(super) auto_approve_low_risk: bool,
    /// Risk threshold for auto-approval (0-10)
    pub(super) auto_approve_threshold: u8,
    /// Pause flag
    pub(super) pause_requested: Arc<Mutex<bool>>,
    /// Cancel flag
    pub(super) cancel_requested: Arc<Mutex<bool>>,
    /// Execution start time
    pub(super) started_at: Option<std::time::Instant>,
}

impl Default for AgentExecutor {
    fn default() -> Self {
        Self::new()
    }
}
