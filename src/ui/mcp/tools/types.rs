//! Types for MCP tools panel

use crate::mcp::McpTool;
use serde_json::Value;
use std::collections::HashMap;

/// Approval status for a tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolApprovalStatus {
    /// Not yet approved
    Pending,
    /// Approved for this session only
    ApprovedSession,
    /// Permanently approved (auto-approve)
    ApprovedPermanent,
    /// Denied
    Denied,
}

/// Tool item with server info
#[derive(Debug, Clone)]
pub struct ToolItem {
    /// Server name this tool belongs to
    pub(crate) server: String,
    /// Tool definition
    pub(crate) tool: McpTool,
    /// Approval status
    pub(crate) approval: ToolApprovalStatus,
}

/// Pending tool call awaiting approval
#[derive(Debug, Clone)]
pub struct PendingToolCall {
    /// Unique ID for this call
    pub(crate) id: String,
    /// Server name
    pub(crate) server: String,
    /// Tool name
    pub(crate) tool_name: String,
    /// Tool arguments
    pub(crate) arguments: Option<HashMap<String, Value>>,
    /// Timestamp when requested
    pub(crate) requested_at: std::time::Instant,
}

/// Events emitted by the MCP tools panel
pub enum McpToolsPanelEvent {
    /// Approve a tool call
    ApproveToolCall { call_id: String, permanent: bool },
    /// Deny a tool call
    DenyToolCall(String),
    /// Change approval status for a tool
    SetToolApproval {
        server: String,
        tool_name: String,
        status: ToolApprovalStatus,
    },
    /// Request to execute a tool manually
    ExecuteTool {
        server: String,
        tool_name: String,
        arguments: Option<HashMap<String, Value>>,
    },
}
