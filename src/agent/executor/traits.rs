//! Executor traits

use crate::agent::task::{ToolCall, ToolResult};

/// Tool executor trait for executing tool calls
#[async_trait::async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Execute a tool call and return the result
    async fn execute(&self, tool_call: &ToolCall) -> Result<ToolResult, String>;

    /// Check if a tool requires approval
    fn requires_approval(&self, tool_name: &str) -> bool;
}
