//! Feature discovery and operations (tools, resources, prompts)

use super::super::protocol::*;
use super::messaging::send_request;
use super::types::McpClient;
use serde_json::Value;
use std::collections::HashMap;

impl McpClient {
    /// Initialize the MCP connection
    pub fn initialize(&mut self) -> Result<InitializeResult, McpError> {
        if !self.is_running() {
            return Err(McpError::NotInitialized);
        }

        let params = InitializeParams::default();
        let response: InitializeResult = send_request(self, "initialize", Some(params))?;

        self.server_info = Some(response.server_info.clone());
        self.capabilities = Some(response.capabilities.clone());

        // Send initialized notification
        super::messaging::send_notification(self, "notifications/initialized", None::<()>)?;

        self.initialized = true;

        // Discover available features
        self.refresh_tools()?;
        self.refresh_resources()?;
        self.refresh_prompts()?;

        Ok(response)
    }

    /// Refresh the list of available tools
    pub fn refresh_tools(&mut self) -> Result<(), McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        // Check if server supports tools
        if let Some(caps) = &self.capabilities {
            if caps.tools.is_none() {
                return Ok(());
            }
        }

        let result: ListToolsResult = send_request(self, "tools/list", None::<()>)?;
        self.tools = result.tools;

        Ok(())
    }

    /// Refresh the list of available resources
    pub fn refresh_resources(&mut self) -> Result<(), McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        // Check if server supports resources
        if let Some(caps) = &self.capabilities {
            if caps.resources.is_none() {
                return Ok(());
            }
        }

        let result: ListResourcesResult = send_request(self, "resources/list", None::<()>)?;
        self.resources = result.resources;

        Ok(())
    }

    /// Refresh the list of available prompts
    pub fn refresh_prompts(&mut self) -> Result<(), McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        // Check if server supports prompts
        if let Some(caps) = &self.capabilities {
            if caps.prompts.is_none() {
                return Ok(());
            }
        }

        let result: ListPromptsResult = send_request(self, "prompts/list", None::<()>)?;
        self.prompts = result.prompts;

        Ok(())
    }

    /// Call a tool
    pub fn call_tool(
        &mut self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        let params = CallToolParams {
            name: name.to_string(),
            arguments,
        };

        send_request(self, "tools/call", Some(params))
    }

    /// Read a resource
    pub fn read_resource(&mut self, uri: &str) -> Result<ResourceContents, McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        #[derive(serde::Serialize)]
        struct ReadResourceParams {
            uri: String,
        }

        let params = ReadResourceParams {
            uri: uri.to_string(),
        };

        #[derive(serde::Deserialize)]
        struct ReadResourceResult {
            contents: Vec<ResourceContents>,
        }

        let result: ReadResourceResult = send_request(self, "resources/read", Some(params))?;
        result
            .contents
            .into_iter()
            .next()
            .ok_or_else(|| McpError::Protocol("Empty resource response".into()))
    }

    /// Get a prompt
    pub fn get_prompt(
        &mut self,
        name: &str,
        arguments: Option<HashMap<String, String>>,
    ) -> Result<GetPromptResult, McpError> {
        if !self.initialized {
            return Err(McpError::NotInitialized);
        }

        #[derive(serde::Serialize)]
        struct GetPromptParams {
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            arguments: Option<HashMap<String, String>>,
        }

        let params = GetPromptParams {
            name: name.to_string(),
            arguments,
        };

        send_request(self, "prompts/get", Some(params))
    }
}
