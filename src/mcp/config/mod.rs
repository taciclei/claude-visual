//! MCP Server Configuration
//!
//! Handles loading and managing MCP server configurations from mcp.json.

mod presets;
mod types;

#[cfg(test)]
mod tests;

pub use presets::*;
pub use types::{McpConfig, McpServerConfig};
