//! Block components for chat messages
//!
//! Inspired by Warp's block-based UI for terminal output.

pub mod code_block;
pub mod code_lsp;
pub mod diff_block;
pub mod file_block;
pub mod lazy_block;
pub mod tool_result_block;

pub use code_block::{CodeBlockEvent, CodeBlockView, CodeDisplayMode, HighlightStyle, HighlightedRange};
pub use lazy_block::{LazyBlock, LazyBlockConfig, LazyBlockEvent, LazyState, VisibilityObserver};
pub use tool_result_block::{
    ToolExecutionStatus, ToolResult, ToolResultBlock, ToolResultBlockEvent,
};
