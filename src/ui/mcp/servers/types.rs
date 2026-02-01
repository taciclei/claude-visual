//! Types for MCP servers panel

use crate::mcp::McpServerConfig;
use gpui::*;

/// Connection status for an MCP server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerConnectionStatus {
    /// Server is disconnected
    Disconnected,
    /// Server is connecting
    Connecting,
    /// Server is connected and ready
    Connected,
    /// Server connection failed
    Failed,
}

impl ServerConnectionStatus {
    /// Get the status color
    pub fn color(&self) -> Hsla {
        match self {
            Self::Disconnected => hsla(0.0, 0.0, 0.5, 1.0), // Gray
            Self::Connecting => hsla(45.0 / 360.0, 0.8, 0.5, 1.0), // Yellow
            Self::Connected => hsla(120.0 / 360.0, 0.6, 0.4, 1.0), // Green
            Self::Failed => hsla(0.0, 0.7, 0.5, 1.0),       // Red
        }
    }

    /// Get status label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting...",
            Self::Connected => "Connected",
            Self::Failed => "Failed",
        }
    }
}

/// Server item in the list
#[derive(Debug, Clone)]
pub struct ServerItem {
    /// Server name
    pub(crate) name: String,
    /// Server configuration
    pub(crate) config: McpServerConfig,
    /// Connection status
    pub(crate) status: ServerConnectionStatus,
    /// Number of available tools
    pub(crate) tool_count: usize,
    /// Number of available resources
    pub(crate) resource_count: usize,
    /// Error message if connection failed
    pub(crate) error: Option<String>,
}

/// Events emitted by the MCP servers panel
pub enum McpServersPanelEvent {
    /// Request to connect to a server
    ConnectServer(String),
    /// Request to disconnect from a server
    DisconnectServer(String),
    /// Request to enable a server
    EnableServer(String),
    /// Request to disable a server
    DisableServer(String),
    /// Request to open server configuration
    ConfigureServer(String),
    /// Request to add a new server
    AddServer,
    /// Request to remove a server
    RemoveServer(String),
    /// Request to refresh server list
    Refresh,
    /// Send a Claude Code skill command
    SendSkillCommand(String),
}
