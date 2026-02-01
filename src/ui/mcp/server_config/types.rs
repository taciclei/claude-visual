//! Type definitions for MCP server configuration editor

use crate::mcp::McpServerConfig;
use std::collections::HashMap;

/// Server configuration being edited
#[derive(Debug, Clone)]
pub struct EditingServerConfig {
    /// Server name
    pub(crate) name: String,
    /// Command to execute
    pub(crate) command: String,
    /// Arguments (one per line)
    pub(crate) args: String,
    /// Environment variables (KEY=VALUE per line)
    pub(crate) env: String,
    /// Whether the server is enabled
    pub(crate) enabled: bool,
    /// Description
    pub(crate) description: String,
    /// Auto-approve patterns (one per line)
    pub(crate) auto_approve: String,
    /// Whether this is a new server
    pub(crate) is_new: bool,
}

impl EditingServerConfig {
    /// Create from existing config
    pub fn from_config(name: String, config: &McpServerConfig) -> Self {
        Self {
            name,
            command: config.command.clone(),
            args: config.args.join("\n"),
            env: config
                .env
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("\n"),
            enabled: config.enabled,
            description: config.description.clone().unwrap_or_default(),
            auto_approve: config.auto_approve.join("\n"),
            is_new: false,
        }
    }

    /// Create a new empty config
    pub fn new_server() -> Self {
        Self {
            name: String::new(),
            command: String::new(),
            args: String::new(),
            env: String::new(),
            enabled: true,
            description: String::new(),
            auto_approve: String::new(),
            is_new: true,
        }
    }

    /// Convert to McpServerConfig
    pub fn to_config(&self) -> McpServerConfig {
        McpServerConfig {
            command: self.command.clone(),
            args: self
                .args
                .lines()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
                .collect(),
            env: self
                .env
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() {
                        return None;
                    }
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
            enabled: self.enabled,
            description: if self.description.is_empty() {
                None
            } else {
                Some(self.description.clone())
            },
            auto_approve: self
                .auto_approve
                .lines()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Server name is required".to_string());
        }
        if self.command.trim().is_empty() {
            return Err("Command is required".to_string());
        }
        // Validate env format
        for line in self.env.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.contains('=') {
                return Err(format!(
                    "Invalid environment variable format: {}. Use KEY=VALUE",
                    line
                ));
            }
        }
        Ok(())
    }
}

/// Field being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditingField {
    Name,
    Command,
    Args,
    Env,
    Description,
    AutoApprove,
}

/// Events emitted by the server config editor
pub enum ServerConfigEditorEvent {
    /// Configuration saved
    Save {
        original_name: Option<String>,
        name: String,
        config: McpServerConfig,
    },
    /// Configuration cancelled
    Cancel,
    /// Request to delete server
    Delete(String),
}
