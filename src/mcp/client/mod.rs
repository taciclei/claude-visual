//! MCP Client Implementation
//!
//! Handles communication with MCP servers via JSON-RPC 2.0 over stdio.

mod core;
mod features;
mod manager;
mod messaging;
mod process;
mod types;

// Re-export public types
pub use types::{McpClient, McpManager};
