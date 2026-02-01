//! Type definitions for MCP client

use super::super::config::McpServerConfig;
use super::super::protocol::*;
use std::io::BufReader;
use std::process::Child;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

/// MCP Client for communicating with an MCP server
pub struct McpClient {
    /// Server configuration
    pub(crate) config: McpServerConfig,
    /// Server name
    pub(crate) name: String,
    /// Child process handle
    pub(crate) process: Option<Child>,
    /// Stdin writer (wrapped in Arc<Mutex> for thread safety)
    pub(crate) stdin: Option<Arc<Mutex<std::process::ChildStdin>>>,
    /// Stdout reader
    pub(crate) stdout: Option<BufReader<std::process::ChildStdout>>,
    /// Request ID counter
    pub(crate) request_id: AtomicU64,
    /// Server info after initialization
    pub(crate) server_info: Option<ServerInfo>,
    /// Server capabilities
    pub(crate) capabilities: Option<ServerCapabilities>,
    /// Available tools
    pub(crate) tools: Vec<McpTool>,
    /// Available resources
    pub(crate) resources: Vec<McpResource>,
    /// Available prompts
    pub(crate) prompts: Vec<McpPrompt>,
    /// Whether the client is initialized
    pub(crate) initialized: bool,
}

use std::collections::HashMap;

/// MCP Manager for handling multiple MCP server connections
pub struct McpManager {
    /// Connected clients
    pub(crate) clients: HashMap<String, McpClient>,
}
