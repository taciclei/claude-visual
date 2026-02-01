//! Tool Result Block Component
//!
//! Displays MCP tool execution results in chat with collapsible sections,
//! JSON formatting, and status indicators.

mod arguments;
mod content;
mod core;
mod footer;
mod header;
mod render;
mod types;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{ToolExecutionStatus, ToolResult, ToolResultBlock, ToolResultBlockEvent};
