//! Executor types, states, and events

/// Executor state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutorState {
    /// Executor is idle, no plan running
    Idle,
    /// Executor is running a plan
    Running,
    /// Executor is paused
    Paused,
    /// Executor is waiting for user approval
    WaitingApproval,
    /// Executor has completed
    Completed,
    /// Executor has failed
    Failed,
    /// Executor was cancelled
    Cancelled,
}

impl ExecutorState {
    /// Check if executor can be paused
    pub fn can_pause(&self) -> bool {
        matches!(self, ExecutorState::Running)
    }

    /// Check if executor can be resumed
    pub fn can_resume(&self) -> bool {
        matches!(self, ExecutorState::Paused | ExecutorState::WaitingApproval)
    }

    /// Check if executor is finished
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            ExecutorState::Completed | ExecutorState::Failed | ExecutorState::Cancelled
        )
    }

    /// Get state icon
    pub fn icon(&self) -> &'static str {
        match self {
            ExecutorState::Idle => "⏹️",
            ExecutorState::Running => "▶️",
            ExecutorState::Paused => "⏸️",
            ExecutorState::WaitingApproval => "🔐",
            ExecutorState::Completed => "✅",
            ExecutorState::Failed => "❌",
            ExecutorState::Cancelled => "🚫",
        }
    }
}

/// Events emitted by the executor
#[derive(Debug, Clone)]
pub enum ExecutorEvent {
    /// Executor state changed
    StateChanged(ExecutorState),
    /// A task started
    TaskStarted(String),
    /// A task completed
    TaskCompleted(String, String), // task_id, output
    /// A task failed
    TaskFailed(String, String), // task_id, error
    /// Tool execution requested
    ToolExecutionRequested(String, crate::agent::task::ToolCall), // task_id, tool_call
    /// Tool execution completed
    ToolExecutionCompleted(String, crate::agent::task::ToolResult), // tool_name, result
    /// Approval required for a step
    ApprovalRequired(String, String), // task_id, description
    /// Plan execution completed
    PlanCompleted(PlanResult),
    /// Progress update
    Progress {
        completed: usize,
        total: usize,
        current_task: Option<String>,
    },
}

/// Result of plan execution
#[derive(Debug, Clone)]
pub struct PlanResult {
    /// Plan ID
    pub plan_id: String,
    /// Whether execution was successful
    pub success: bool,
    /// Number of completed steps
    pub completed_steps: usize,
    /// Total number of steps
    pub total_steps: usize,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
}

/// Executor statistics
#[derive(Debug, Clone)]
pub struct ExecutorStats {
    /// Current state
    pub state: ExecutorState,
    /// Number of completed steps
    pub completed_steps: usize,
    /// Total number of steps
    pub total_steps: usize,
    /// Duration in milliseconds (if started)
    pub duration_ms: Option<u64>,
}

impl ExecutorStats {
    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.total_steps == 0 {
            0.0
        } else {
            (self.completed_steps as f32 / self.total_steps as f32) * 100.0
        }
    }
}
