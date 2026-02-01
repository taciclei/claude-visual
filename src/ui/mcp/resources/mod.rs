//! MCP Resources Panel
//!
//! UI component for viewing and browsing MCP resources and prompts.

mod core;
mod render;
mod traits;
mod types;

// Re-export public types
pub use core::McpResourcesPanel;
pub use types::{McpResourcesPanelEvent, PromptItem, ResourceItem, ResourcesTab};
