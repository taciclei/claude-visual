//! Type definitions for tool result blocks

use std::sync::Arc;
use std::time::Duration;

use gpui::*;
use serde_json::Value;

use crate::app::state::AppState;

/// Status of a tool execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolExecutionStatus {
    /// Tool executed successfully
    Success,
    /// Tool execution failed
    Error,
    /// Tool execution is pending
    Pending,
    /// Tool execution was cancelled
    Cancelled,
}

impl ToolExecutionStatus {
    /// Get display text
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Error => "Error",
            Self::Pending => "Pending",
            Self::Cancelled => "Cancelled",
        }
    }

    /// Get icon character
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Error => "✗",
            Self::Pending => "⋯",
            Self::Cancelled => "○",
        }
    }
}

/// A tool execution result for display
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// Tool name
    pub tool_name: String,
    /// Server name
    pub server_name: String,
    /// Input arguments (as JSON)
    pub arguments: Option<Value>,
    /// Result content
    pub content: Option<Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution status
    pub status: ToolExecutionStatus,
    /// Execution duration
    pub duration: Option<Duration>,
}

/// Events emitted by ToolResultBlock
pub enum ToolResultBlockEvent {
    /// Toggle collapsed state
    ToggleCollapsed,
    /// Copy result to clipboard
    CopyResult,
    /// Retry the tool execution
    Retry {
        tool_name: String,
        server_name: String,
        arguments: Option<Value>,
    },
    /// Execute a Claude Code skill (e.g., /debug, /explain)
    ExecuteSkill(String),
}

/// Tool result block for displaying MCP tool execution results
pub struct ToolResultBlock {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) result: ToolResult,
    /// Whether the block is collapsed
    pub(crate) collapsed: bool,
    /// Whether arguments section is expanded
    pub(crate) args_expanded: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}
