//! Type definitions for MCP resources panel

use crate::mcp::{McpPrompt, McpResource};

/// Resource item with server info
#[derive(Debug, Clone)]
pub struct ResourceItem {
    /// Server name this resource belongs to
    pub server: String,
    /// Resource definition
    pub resource: McpResource,
}

/// Prompt item with server info
#[derive(Debug, Clone)]
pub struct PromptItem {
    /// Server name this prompt belongs to
    pub server: String,
    /// Prompt definition
    pub prompt: McpPrompt,
}

/// Events emitted by the MCP resources panel
pub enum McpResourcesPanelEvent {
    /// Request to read a resource
    ReadResource { server: String, uri: String },
    /// Request to attach a resource to context
    AttachResource { server: String, uri: String },
    /// Request to use a prompt
    UsePrompt { server: String, prompt_name: String },
    /// Refresh resources/prompts
    Refresh,
}

/// Active tab in the panel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcesTab {
    Resources,
    Prompts,
}
