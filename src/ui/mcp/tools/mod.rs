//! MCP Tools Panel
//!
//! UI component for viewing and approving MCP tool calls.

mod core;
mod header;
mod pending_call;
mod render;
mod tool_item;
mod types;

// Re-export public types
pub use types::{McpToolsPanelEvent, PendingToolCall, ToolApprovalStatus, ToolItem};

// Re-export main struct
pub use core::McpToolsPanel;
