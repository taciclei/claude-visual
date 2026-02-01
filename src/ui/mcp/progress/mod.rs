//! MCP Tool Execution Progress Panel
//!
//! Displays active tool executions with progress indicators,
//! elapsed time, and cancel functionality.

mod types;
mod core;
mod render;

#[cfg(test)]
mod tests;

// Re-export public API
pub use types::{ExecutionPhase, ActiveExecution, ToolProgressPanelEvent};
pub use core::ToolProgressPanel;
