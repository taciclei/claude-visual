//! MCP (Model Context Protocol) server-related methods

use super::ChatView;
use gpui::*;

/// Maximum number of recent MCP tools to track
const MAX_RECENT_MCP_TOOLS: usize = 5;
/// Maximum number of favorite MCP tools
const MAX_FAVORITE_MCP_TOOLS: usize = 10;

impl ChatView {
    // ==================== MCP Servers ====================

    /// Toggle MCP servers panel
    pub fn toggle_mcp_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.mcp_panel = !self.panels.mcp_panel;
        self.panels.mcp_quick_tools = false;
        // Sync with legacy field
        self.show_mcp_panel = self.panels.mcp_panel;
        cx.notify();
    }

    /// Toggle MCP quick tools dropdown
    pub fn toggle_mcp_quick_tools(&mut self, cx: &mut Context<Self>) {
        self.panels.mcp_quick_tools = !self.panels.mcp_quick_tools;
        self.panels.mcp_panel = false;
        // Sync with legacy field
        self.show_mcp_panel = false;
        cx.notify();
    }

    /// Get MCP server count
    pub fn mcp_server_count(&self) -> usize {
        self.session_info
            .as_ref()
            .map(|info| info.mcp_servers.len())
            .unwrap_or(0)
    }

    /// Get total MCP tool count
    pub fn mcp_tool_count(&self) -> usize {
        self.session_info
            .as_ref()
            .map(|info| {
                // Count tools that look like MCP tools (prefixed with mcp__)
                info.tools
                    .iter()
                    .filter(|t| t.starts_with("mcp__") || t.contains(":"))
                    .count()
            })
            .unwrap_or(0)
    }

    /// Toggle MCP server expansion in panel
    pub fn toggle_mcp_server_expanded(&mut self, server_name: &str, cx: &mut Context<Self>) {
        if self.expanded_mcp_servers.contains(server_name) {
            self.expanded_mcp_servers.remove(server_name);
        } else {
            self.expanded_mcp_servers.insert(server_name.to_string());
        }
        cx.notify();
    }

    /// Check if an MCP server is expanded
    pub fn is_mcp_server_expanded(&self, server_name: &str) -> bool {
        self.expanded_mcp_servers.contains(server_name)
    }

    /// Get tools for MCP server (from session tools that match server prefix)
    pub fn get_mcp_server_tools(&self, server_name: &str) -> Vec<String> {
        // MCP tools are typically prefixed with server name, e.g., "mcp__serverName__toolName"
        self.session_info
            .as_ref()
            .map(|info| {
                info.tools
                    .iter()
                    .filter(|tool| {
                        let tool_lower = tool.to_lowercase();
                        let server_lower = server_name.to_lowercase();
                        tool_lower.contains(&format!("mcp__{}", server_lower))
                            || tool_lower.starts_with(&format!("{}:", server_lower))
                            || tool_lower.starts_with(&format!("{}_", server_lower))
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all MCP tools
    pub fn get_all_mcp_tools(&self) -> Vec<String> {
        self.session_info
            .as_ref()
            .map(|info| {
                info.tools
                    .iter()
                    .filter(|t| t.starts_with("mcp__") || t.contains(":"))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Use MCP server tool - insert prompt suggestion and track usage
    pub fn use_mcp_tool(&mut self, tool_name: &str, cx: &mut Context<Self>) {
        // Track recent usage
        self.track_mcp_tool_usage(tool_name);

        let prompt = format!("Use the {} tool to ", tool_name);
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.insert_text(&prompt, cx);
        });
        self.panels.mcp_panel = false;
        self.panels.mcp_quick_tools = false;
        cx.notify();
    }

    /// Track MCP tool usage for recent list
    fn track_mcp_tool_usage(&mut self, tool_name: &str) {
        // Remove if already in list
        self.recent_mcp_tools.retain(|t| t != tool_name);
        // Add to front
        self.recent_mcp_tools.insert(0, tool_name.to_string());
        // Trim to max size
        self.recent_mcp_tools.truncate(MAX_RECENT_MCP_TOOLS);
    }

    /// Toggle tool favorite status
    pub fn toggle_mcp_tool_favorite(&mut self, tool_name: &str, cx: &mut Context<Self>) {
        if self.favorite_mcp_tools.contains(&tool_name.to_string()) {
            self.favorite_mcp_tools.retain(|t| t != tool_name);
        } else if self.favorite_mcp_tools.len() < MAX_FAVORITE_MCP_TOOLS {
            self.favorite_mcp_tools.push(tool_name.to_string());
        }
        cx.notify();
    }

    /// Check if a tool is favorited
    pub fn is_mcp_tool_favorite(&self, tool_name: &str) -> bool {
        self.favorite_mcp_tools.contains(&tool_name.to_string())
    }

    /// Get recent MCP tools
    pub fn recent_mcp_tools(&self) -> &[String] {
        &self.recent_mcp_tools
    }

    /// Get favorite MCP tools
    pub fn favorite_mcp_tools(&self) -> &[String] {
        &self.favorite_mcp_tools
    }

    /// Get quick access tools (favorites first, then recent, up to a limit)
    pub fn get_quick_mcp_tools(&self) -> Vec<(String, bool)> {
        let mut tools: Vec<(String, bool)> = Vec::new();

        // Add favorites first (marked as favorite)
        for tool in &self.favorite_mcp_tools {
            if tools.len() < 8 {
                tools.push((tool.clone(), true));
            }
        }

        // Add recent tools (not marked as favorite)
        for tool in &self.recent_mcp_tools {
            if tools.len() < 8 && !self.favorite_mcp_tools.contains(tool) {
                tools.push((tool.clone(), false));
            }
        }

        tools
    }
}
