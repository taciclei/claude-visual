//! AgentTask definition and implementation

use super::types::{TaskPriority, TaskStatus, ToolCall};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single task in the agent's plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Unique task ID
    pub id: String,
    /// Parent task ID (for subtasks)
    pub parent_id: Option<String>,
    /// Task title/summary
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Current status
    pub status: TaskStatus,
    /// Priority
    pub priority: TaskPriority,
    /// Tool calls for this task
    pub tool_calls: Vec<ToolCall>,
    /// Output/result of the task
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// When task was created
    pub created_at: DateTime<Utc>,
    /// When task started
    pub started_at: Option<DateTime<Utc>>,
    /// When task completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Task metadata
    pub metadata: HashMap<String, String>,
    /// Retry count
    pub retry_count: u32,
    /// Max retries allowed
    pub max_retries: u32,
}

impl AgentTask {
    /// Create a new task
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: None,
            title: title.into(),
            description: description.into(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            tool_calls: Vec::new(),
            output: None,
            error: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            metadata: HashMap::new(),
            retry_count: 0,
            max_retries: 3,
        }
    }

    /// Create a subtask
    pub fn subtask(
        parent_id: &str,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let mut task = Self::new(title, description);
        task.parent_id = Some(parent_id.to_string());
        task
    }

    /// Set priority
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add a tool call
    pub fn add_tool_call(
        &mut self,
        name: impl Into<String>,
        arguments: serde_json::Value,
        requires_approval: bool,
    ) {
        self.tool_calls.push(ToolCall {
            name: name.into(),
            arguments,
            requires_approval,
            result: None,
        });
    }

    /// Start the task
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(Utc::now());
    }

    /// Complete the task
    pub fn complete(&mut self, output: impl Into<String>) {
        self.status = TaskStatus::Completed;
        self.output = Some(output.into());
        self.completed_at = Some(Utc::now());
    }

    /// Fail the task
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = TaskStatus::Failed;
        self.error = Some(error.into());
        self.completed_at = Some(Utc::now());
    }

    /// Pause the task
    pub fn pause(&mut self) {
        if self.status == TaskStatus::Running {
            self.status = TaskStatus::Paused;
        }
    }

    /// Resume the task
    pub fn resume(&mut self) {
        if self.status.can_resume() {
            self.status = TaskStatus::Running;
        }
    }

    /// Request approval
    pub fn request_approval(&mut self) {
        self.status = TaskStatus::WaitingApproval;
    }

    /// Cancel the task
    pub fn cancel(&mut self) {
        if !self.status.is_terminal() {
            self.status = TaskStatus::Cancelled;
            self.completed_at = Some(Utc::now());
        }
    }

    /// Check if task can retry
    pub fn can_retry(&self) -> bool {
        self.status == TaskStatus::Failed && self.retry_count < self.max_retries
    }

    /// Retry the task
    pub fn retry(&mut self) {
        if self.can_retry() {
            self.retry_count += 1;
            self.status = TaskStatus::Pending;
            self.error = None;
            self.started_at = None;
            self.completed_at = None;
        }
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> Option<i64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_milliseconds()),
            (Some(start), None) => Some((Utc::now() - start).num_milliseconds()),
            _ => None,
        }
    }
}
