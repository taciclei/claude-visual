//! Constructor and builder methods

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::agent::task::TaskTree;

use super::super::traits::ToolExecutor;
use super::super::types::{ExecutorEvent, ExecutorState};
use super::executor::AgentExecutor;

impl AgentExecutor {
    /// Create a new executor
    pub fn new() -> Self {
        Self {
            state: ExecutorState::Idle,
            current_plan: None,
            task_tree: TaskTree::new(),
            completed_steps: Vec::new(),
            event_tx: None,
            tool_executor: None,
            auto_approve_low_risk: true,
            auto_approve_threshold: 3,
            pause_requested: Arc::new(Mutex::new(false)),
            cancel_requested: Arc::new(Mutex::new(false)),
            started_at: None,
        }
    }

    /// Set the event sender
    pub fn with_event_sender(mut self, tx: mpsc::UnboundedSender<ExecutorEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Set the tool executor
    pub fn with_tool_executor(mut self, executor: Arc<dyn ToolExecutor>) -> Self {
        self.tool_executor = Some(executor);
        self
    }

    /// Set auto-approve threshold
    pub fn with_auto_approve_threshold(mut self, threshold: u8) -> Self {
        self.auto_approve_threshold = threshold.min(10);
        self
    }
}
