//! Basic task types and enums

use serde::{Deserialize, Serialize};

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending execution
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was skipped
    Skipped,
    /// Task is paused
    Paused,
    /// Task is waiting for approval
    WaitingApproval,
    /// Task was cancelled
    Cancelled,
}

impl TaskStatus {
    /// Check if task is terminal (no more state changes expected)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Skipped | TaskStatus::Cancelled
        )
    }

    /// Check if task can be resumed
    pub fn can_resume(&self) -> bool {
        matches!(self, TaskStatus::Paused | TaskStatus::WaitingApproval)
    }

    /// Get status icon
    pub fn icon(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "⏳",
            TaskStatus::Running => "🔄",
            TaskStatus::Completed => "✅",
            TaskStatus::Failed => "❌",
            TaskStatus::Skipped => "⏭️",
            TaskStatus::Paused => "⏸️",
            TaskStatus::WaitingApproval => "🔐",
            TaskStatus::Cancelled => "🚫",
        }
    }
}

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Tool call that a task might execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name
    pub name: String,
    /// Tool arguments
    pub arguments: serde_json::Value,
    /// Whether tool requires approval
    pub requires_approval: bool,
    /// Tool result (if executed)
    pub result: Option<ToolResult>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Success or failure
    pub success: bool,
    /// Output content
    pub output: String,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution duration in ms
    pub duration_ms: u64,
}
