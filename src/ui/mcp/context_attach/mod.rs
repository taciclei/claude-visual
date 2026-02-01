//! MCP Context Attachment
//!
//! Component for attaching MCP resources and prompts to conversation context.

mod core;
mod render;
mod types;

#[cfg(test)]
mod tests;

// Re-export public types
pub use core::McpContextAttachPanel;
pub use types::{AttachableResource, AttachmentStatus, McpContextAttachEvent};
