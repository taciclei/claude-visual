//! MCP Tools Panel
//!
//! UI component for viewing and approving MCP tool calls.

mod types;
mod core;
mod pending_call;
mod tool_item;
mod header;
mod render;

// Re-export public types
pub use types::{
    ToolApprovalStatus,
    ToolItem,
    PendingToolCall,
    McpToolsPanelEvent,
};

// Re-export main struct
pub use core::McpToolsPanel;
