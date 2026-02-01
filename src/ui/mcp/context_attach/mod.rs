//! MCP Context Attachment
//!
//! Component for attaching MCP resources and prompts to conversation context.

mod types;
mod core;
mod render;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{AttachableResource, AttachmentStatus, McpContextAttachEvent};
pub use core::McpContextAttachPanel;
