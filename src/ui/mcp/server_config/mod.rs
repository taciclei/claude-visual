//! MCP Server Configuration Editor
//!
//! UI component for editing MCP server configuration.

mod core;
mod render;
mod types;

#[cfg(test)]
mod tests;

// Re-export public types
pub use core::ServerConfigEditor;
pub use types::{EditingField, EditingServerConfig, ServerConfigEditorEvent};
