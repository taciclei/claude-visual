//! Core McpClient methods (constructor, getters, lifecycle)

use super::super::config::McpServerConfig;
use super::super::protocol::*;
use super::types::McpClient;
use std::sync::atomic::AtomicU64;

impl McpClient {
    /// Create a new MCP client for a server configuration
    pub fn new(name: impl Into<String>, config: McpServerConfig) -> Self {
        Self {
            name: name.into(),
            config,
            process: None,
            stdin: None,
            stdout: None,
            request_id: AtomicU64::new(1),
            server_info: None,
            capabilities: None,
            tools: Vec::new(),
            resources: Vec::new(),
            prompts: Vec::new(),
            initialized: false,
        }
    }

    /// Get the server name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get server info (available after initialization)
    pub fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> Option<&ServerCapabilities> {
        self.capabilities.as_ref()
    }

    /// Get available tools
    pub fn tools(&self) -> &[McpTool] {
        &self.tools
    }

    /// Get available resources
    pub fn resources(&self) -> &[McpResource] {
        &self.resources
    }

    /// Get available prompts
    pub fn prompts(&self) -> &[McpPrompt] {
        &self.prompts
    }

    /// Check if client is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Check if process is running
    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        let _ = super::process::stop(self);
    }
}
