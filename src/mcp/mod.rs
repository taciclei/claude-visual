//! Model Context Protocol (MCP) Integration
//!
//! Implements the MCP client for connecting to MCP servers,
//! discovering tools, resources, and prompts.
//!
//! MCP uses JSON-RPC 2.0 over stdio for communication.

mod client;
mod config;
mod protocol;
mod server;
mod tools;

pub use client::{McpClient, McpManager};
pub use config::{McpConfig, McpServerConfig};
pub use protocol::{
    JsonRpcRequest, JsonRpcResponse, McpCapabilities, McpError, McpPrompt, McpResource, McpTool,
    ServerInfo,
};
pub use server::{
    create_shared_registry, McpServerRegistry, ServerHealth, ServerStatus, SharedMcpRegistry,
};
pub use tools::{build_arguments, EnrichedTool, ToolCategory, ToolRegistry};
