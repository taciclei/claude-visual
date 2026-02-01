//! Core MCP tools panel implementation

use super::types::{McpToolsPanelEvent, PendingToolCall, ToolApprovalStatus, ToolItem};
use crate::app::state::AppState;
use crate::mcp::McpTool;
use gpui::*;
use std::sync::Arc;

impl EventEmitter<McpToolsPanelEvent> for McpToolsPanel {}

/// MCP Tools Panel for viewing and approving tools
pub struct McpToolsPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Available tools from all connected servers
    pub(crate) tools: Vec<ToolItem>,
    /// Pending tool calls awaiting approval
    pub(crate) pending_calls: Vec<PendingToolCall>,
    /// Search filter
    pub(crate) filter_text: String,
    /// Selected tool index
    pub(crate) selected_tool: Option<usize>,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl McpToolsPanel {
    /// Create a new MCP tools panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            tools: Vec::new(),
            pending_calls: Vec::new(),
            filter_text: String::new(),
            selected_tool: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Update the list of available tools
    pub fn set_tools(&mut self, tools: Vec<ToolItem>, cx: &mut Context<Self>) {
        self.tools = tools;
        cx.notify();
    }

    /// Add a tool from a server
    pub fn add_tool(&mut self, server: String, tool: McpTool, cx: &mut Context<Self>) {
        self.tools.push(ToolItem {
            server,
            tool,
            approval: ToolApprovalStatus::Pending,
        });
        cx.notify();
    }

    /// Remove all tools from a server
    pub fn remove_server_tools(&mut self, server: &str, cx: &mut Context<Self>) {
        self.tools.retain(|t| t.server != server);
        cx.notify();
    }

    /// Add a pending tool call
    pub fn add_pending_call(&mut self, call: PendingToolCall, cx: &mut Context<Self>) {
        self.pending_calls.push(call);
        cx.notify();
    }

    /// Remove a pending call by ID
    pub fn remove_pending_call(&mut self, call_id: &str, cx: &mut Context<Self>) {
        self.pending_calls.retain(|c| c.id != call_id);
        cx.notify();
    }

    /// Get pending calls
    pub fn pending_calls(&self) -> &[PendingToolCall] {
        &self.pending_calls
    }

    /// Get filtered tools
    pub(crate) fn filtered_tools(&self) -> Vec<&ToolItem> {
        self.tools
            .iter()
            .filter(|t| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    let filter = self.filter_text.to_lowercase();
                    t.tool.name.to_lowercase().contains(&filter)
                        || t.tool
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter))
                            .unwrap_or(false)
                        || t.server.to_lowercase().contains(&filter)
                }
            })
            .collect()
    }

    /// Set filter text
    pub fn set_filter(&mut self, text: String, cx: &mut Context<Self>) {
        self.filter_text = text;
        cx.notify();
    }
}

impl Focusable for McpToolsPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
