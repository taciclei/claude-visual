//! Agent Task Management
//!
//! Defines task tree structure for agent mode execution.

mod node;
mod task;
mod tree;
mod types;

#[cfg(test)]
mod tests;

// Re-export public types
pub use node::TaskNode;
pub use task::AgentTask;
pub use tree::TaskTree;
pub use types::{TaskPriority, TaskStatus, ToolCall, ToolResult};
