//! Core MCP configuration types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// MCP configuration file (mcp.json)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpConfig {
    /// MCP servers configuration
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerConfig {
    /// Command to execute to start the server
    pub command: String,
    /// Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Whether the server is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Description of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Auto-approve tool calls (dangerous, use with caution)
    #[serde(default)]
    pub auto_approve: Vec<String>,
}

fn default_enabled() -> bool {
    true
}

impl McpConfig {
    /// Load MCP configuration from a file
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Load MCP configuration from the default location
    /// Searches in order:
    /// 1. Current directory ./mcp.json
    /// 2. Project root .mcp/mcp.json
    /// 3. User config ~/.config/claude-visual/mcp.json
    pub fn load_default(project_root: Option<&Path>) -> anyhow::Result<Self> {
        // Try current directory first
        let cwd = std::env::current_dir()?;
        let cwd_config = cwd.join("mcp.json");
        if cwd_config.exists() {
            return Self::load(&cwd_config);
        }

        // Try project root
        if let Some(root) = project_root {
            let project_config = root.join(".mcp").join("mcp.json");
            if project_config.exists() {
                return Self::load(&project_config);
            }

            // Also try mcp.json directly in project root
            let root_config = root.join("mcp.json");
            if root_config.exists() {
                return Self::load(&root_config);
            }
        }

        // Try user config directory
        if let Some(config_dir) = dirs::config_dir() {
            let user_config = config_dir.join("claude-visual").join("mcp.json");
            if user_config.exists() {
                return Self::load(&user_config);
            }
        }

        // Return empty config if no file found
        Ok(Self::default())
    }

    /// Save MCP configuration to a file
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get enabled servers
    pub fn enabled_servers(&self) -> impl Iterator<Item = (&String, &McpServerConfig)> {
        self.mcp_servers
            .iter()
            .filter(|(_, config)| config.enabled)
    }

    /// Get server by name
    pub fn get_server(&self, name: &str) -> Option<&McpServerConfig> {
        self.mcp_servers.get(name)
    }

    /// Add or update a server configuration
    pub fn set_server(&mut self, name: String, config: McpServerConfig) {
        self.mcp_servers.insert(name, config);
    }

    /// Remove a server configuration
    pub fn remove_server(&mut self, name: &str) -> Option<McpServerConfig> {
        self.mcp_servers.remove(name)
    }

    /// Enable a server
    pub fn enable_server(&mut self, name: &str) -> bool {
        if let Some(server) = self.mcp_servers.get_mut(name) {
            server.enabled = true;
            true
        } else {
            false
        }
    }

    /// Disable a server
    pub fn disable_server(&mut self, name: &str) -> bool {
        if let Some(server) = self.mcp_servers.get_mut(name) {
            server.enabled = false;
            true
        } else {
            false
        }
    }
}

impl McpServerConfig {
    /// Create a new server configuration
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            env: HashMap::new(),
            enabled: true,
            description: None,
            auto_approve: Vec::new(),
        }
    }

    /// Add an argument
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add arguments
    pub fn with_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(|a| a.into()));
        self
    }

    /// Add an environment variable
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get the full command line
    pub fn command_line(&self) -> String {
        let mut parts = vec![self.command.clone()];
        parts.extend(self.args.clone());
        parts.join(" ")
    }
}
