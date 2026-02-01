//! Core logic for MCP servers panel

use std::sync::Arc;
use gpui::*;
use crate::app::state::AppState;
use crate::mcp::McpConfig;
use super::types::{ServerItem, ServerConnectionStatus, McpServersPanelEvent};

/// MCP Servers Panel for managing server connections
pub struct McpServersPanel {
    pub(crate) app_state: Arc<AppState>,
    /// MCP configuration
    pub(crate) config: McpConfig,
    /// Server items with status
    pub(crate) servers: Vec<ServerItem>,
    /// Currently selected server index
    pub(crate) selected_server: Option<usize>,
    /// Whether the panel is expanded
    pub(crate) expanded: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl EventEmitter<McpServersPanelEvent> for McpServersPanel {}

impl McpServersPanel {
    /// Create a new MCP servers panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Load MCP configuration
        let config = McpConfig::load_default(None).unwrap_or_default();

        // Build server items from config
        let servers = config
            .mcp_servers
            .iter()
            .map(|(name, server_config)| ServerItem {
                name: name.clone(),
                config: server_config.clone(),
                status: ServerConnectionStatus::Disconnected,
                tool_count: 0,
                resource_count: 0,
                error: None,
            })
            .collect();

        Self {
            app_state,
            config,
            servers,
            selected_server: None,
            expanded: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Refresh the server list from configuration
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.config = McpConfig::load_default(None).unwrap_or_default();

        // Update server items, preserving connection status for existing servers
        let old_servers: std::collections::HashMap<String, ServerItem> = self
            .servers
            .drain(..)
            .map(|s| (s.name.clone(), s))
            .collect();

        self.servers = self
            .config
            .mcp_servers
            .iter()
            .map(|(name, server_config)| {
                if let Some(old_server) = old_servers.get(name) {
                    ServerItem {
                        name: name.clone(),
                        config: server_config.clone(),
                        status: old_server.status,
                        tool_count: old_server.tool_count,
                        resource_count: old_server.resource_count,
                        error: old_server.error.clone(),
                    }
                } else {
                    ServerItem {
                        name: name.clone(),
                        config: server_config.clone(),
                        status: ServerConnectionStatus::Disconnected,
                        tool_count: 0,
                        resource_count: 0,
                        error: None,
                    }
                }
            })
            .collect();

        cx.notify();
    }

    /// Update a server's connection status
    pub fn update_server_status(
        &mut self,
        name: &str,
        status: ServerConnectionStatus,
        tool_count: Option<usize>,
        resource_count: Option<usize>,
        error: Option<String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(server) = self.servers.iter_mut().find(|s| s.name == name) {
            server.status = status;
            if let Some(count) = tool_count {
                server.tool_count = count;
            }
            if let Some(count) = resource_count {
                server.resource_count = count;
            }
            server.error = error;
            cx.notify();
        }
    }

    /// Get servers list
    pub fn servers(&self) -> &[ServerItem] {
        &self.servers
    }

    /// Toggle panel expansion
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Handle server click
    pub(crate) fn handle_server_click(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_server = Some(index);
        cx.notify();
    }
}

impl Focusable for McpServersPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
