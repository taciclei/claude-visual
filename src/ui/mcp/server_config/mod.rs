//! MCP Server Configuration Editor
//!
//! UI component for editing MCP server configuration.

mod types;
mod core;
mod render;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{EditingServerConfig, EditingField, ServerConfigEditorEvent};
pub use core::ServerConfigEditor;
