//! Task tree node

use super::task::AgentTask;
use serde::{Deserialize, Serialize};

/// A node in the task tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    /// The task
    pub(crate) task: AgentTask,
    /// Child task IDs
    pub(crate) children: Vec<String>,
    /// Whether node is expanded in UI
    pub(crate) is_expanded: bool,
}
