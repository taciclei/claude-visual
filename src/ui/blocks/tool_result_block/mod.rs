//! Tool Result Block Component
//!
//! Displays MCP tool execution results in chat with collapsible sections,
//! JSON formatting, and status indicators.

mod types;
mod core;
mod header;
mod arguments;
mod content;
mod footer;
mod render;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{
    ToolExecutionStatus,
    ToolResult,
    ToolResultBlock,
    ToolResultBlockEvent,
};
