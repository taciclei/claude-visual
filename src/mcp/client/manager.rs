//! MCP Manager for handling multiple server connections

use super::super::config::McpServerConfig;
use super::super::protocol::*;
use super::types::{McpClient, McpManager};
use serde_json::Value;
use std::collections::HashMap;

impl McpManager {
    /// Create a new MCP manager
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Connect to an MCP server
    pub fn connect(
        &mut self,
        name: impl Into<String>,
        config: McpServerConfig,
    ) -> Result<(), McpError> {
        let name = name.into();

        if self.clients.contains_key(&name) {
            return Err(McpError::Connection(format!(
                "Server '{}' already connected",
                name
            )));
        }

        let mut client = McpClient::new(name.clone(), config);
        client.start()?;
        client.initialize()?;

        self.clients.insert(name, client);
        Ok(())
    }

    /// Disconnect from an MCP server
    pub fn disconnect(&mut self, name: &str) -> Result<(), McpError> {
        if let Some(mut client) = self.clients.remove(name) {
            client.stop()?;
        }
        Ok(())
    }

    /// Disconnect from all servers
    pub fn disconnect_all(&mut self) {
        for (_, mut client) in self.clients.drain() {
            let _ = client.stop();
        }
    }

    /// Get a client by name
    pub fn get(&self, name: &str) -> Option<&McpClient> {
        self.clients.get(name)
    }

    /// Get a mutable client by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut McpClient> {
        self.clients.get_mut(name)
    }

    /// Get all connected client names
    pub fn connected_servers(&self) -> impl Iterator<Item = &String> {
        self.clients.keys()
    }

    /// Get all available tools from all connected servers
    pub fn all_tools(&self) -> Vec<(&str, &McpTool)> {
        self.clients
            .iter()
            .flat_map(|(name, client)| client.tools().iter().map(move |tool| (name.as_str(), tool)))
            .collect()
    }

    /// Get all available resources from all connected servers
    pub fn all_resources(&self) -> Vec<(&str, &McpResource)> {
        self.clients
            .iter()
            .flat_map(|(name, client)| {
                client
                    .resources()
                    .iter()
                    .map(move |resource| (name.as_str(), resource))
            })
            .collect()
    }

    /// Get all available prompts from all connected servers
    pub fn all_prompts(&self) -> Vec<(&str, &McpPrompt)> {
        self.clients
            .iter()
            .flat_map(|(name, client)| {
                client
                    .prompts()
                    .iter()
                    .map(move |prompt| (name.as_str(), prompt))
            })
            .collect()
    }

    /// Call a tool on a specific server
    pub fn call_tool(
        &mut self,
        server: &str,
        tool_name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let client = self
            .clients
            .get_mut(server)
            .ok_or_else(|| McpError::Connection(format!("Server '{}' not connected", server)))?;

        client.call_tool(tool_name, arguments)
    }

    /// Read a resource from a specific server
    pub fn read_resource(&mut self, server: &str, uri: &str) -> Result<ResourceContents, McpError> {
        let client = self
            .clients
            .get_mut(server)
            .ok_or_else(|| McpError::Connection(format!("Server '{}' not connected", server)))?;

        client.read_resource(uri)
    }

    /// Get a prompt from a specific server
    pub fn get_prompt(
        &mut self,
        server: &str,
        prompt_name: &str,
        arguments: Option<HashMap<String, String>>,
    ) -> Result<GetPromptResult, McpError> {
        let client = self
            .clients
            .get_mut(server)
            .ok_or_else(|| McpError::Connection(format!("Server '{}' not connected", server)))?;

        client.get_prompt(prompt_name, arguments)
    }
}

impl Default for McpManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for McpManager {
    fn drop(&mut self) {
        self.disconnect_all();
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = McpServerConfig::new("echo");
        let client = McpClient::new("test", config);

        assert_eq!(client.name(), "test");
        assert!(!client.is_running());
        assert!(!client.is_initialized());
    }

    #[test]
    fn test_manager_creation() {
        let manager = McpManager::new();
        assert_eq!(manager.connected_servers().count(), 0);
    }
}
