//! MCP Tool Execution Progress Panel
//!
//! Displays active tool executions with progress indicators,
//! elapsed time, and cancel functionality.

mod core;
mod render;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use core::ToolProgressPanel;
pub use types::{ActiveExecution, ExecutionPhase, ToolProgressPanelEvent};
