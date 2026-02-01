//! Message types for Claude communication

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role of a message in the conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    ToolUse,
    ToolResult,
    Error,
    /// Claude's thinking/reasoning (extended thinking)
    Thinking,
    /// System message (session info, etc.)
    System,
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    /// Role of the message sender
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Tool name (for tool use/result messages)
    pub tool_name: Option<String>,
    /// Whether this is an error
    pub is_error: bool,
}

impl ClaudeMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error: false,
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error: false,
        }
    }

    /// Create a tool use message
    pub fn tool_use(name: impl Into<String>, input: serde_json::Value) -> Self {
        let name = name.into();
        Self {
            role: MessageRole::ToolUse,
            content: serde_json::to_string_pretty(&input).unwrap_or_default(),
            timestamp: Utc::now(),
            tool_name: Some(name),
            is_error: false,
        }
    }

    /// Create a tool result message
    pub fn tool_result(output: impl Into<String>, is_error: bool) -> Self {
        Self {
            role: MessageRole::ToolResult,
            content: output.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error,
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Error,
            content: message.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error: true,
        }
    }

    /// Create a thinking message (Claude's reasoning)
    pub fn thinking(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Thinking,
            content: content.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error: false,
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
            timestamp: Utc::now(),
            tool_name: None,
            is_error: false,
        }
    }
}

/// Session information from Claude CLI init
#[derive(Debug, Clone, Default)]
pub struct SessionInfo {
    /// Session ID
    pub session_id: String,
    /// Model being used
    pub model: String,
    /// Available tools
    pub tools: Vec<String>,
    /// Available slash commands
    pub slash_commands: Vec<String>,
    /// Available agents
    pub agents: Vec<String>,
    /// Available skills
    pub skills: Vec<String>,
    /// Current working directory
    pub cwd: String,
    /// Claude Code version
    pub version: String,
    /// Connected MCP servers
    pub mcp_servers: Vec<McpServerInfo>,
}

/// MCP (Model Context Protocol) server info
#[derive(Debug, Clone, Default)]
pub struct McpServerInfo {
    /// Server name
    pub name: String,
    /// Server status
    pub status: McpServerStatus,
    /// Number of tools provided
    pub tool_count: usize,
    /// Number of prompts/resources provided
    pub resource_count: usize,
}

/// MCP server connection status
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum McpServerStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Events from the Claude CLI stream
#[derive(Debug, Clone)]
pub enum ClaudeEvent {
    /// Assistant message started
    AssistantStart,
    /// Content block delta (streaming text)
    ContentBlockDelta { delta: String },
    /// Assistant message ended
    AssistantEnd,
    /// Tool use started
    ToolUse {
        name: String,
        input: serde_json::Value,
    },
    /// Tool result received
    ToolResult { output: String, is_error: bool },
    /// Error occurred
    Error { message: String },
    /// System init with session info
    SystemInit { info: SessionInfo },
    /// Claude thinking (extended thinking)
    Thinking { content: String },
    /// Session usage/cost information
    Usage {
        input_tokens: u64,
        output_tokens: u64,
        cost_usd: Option<f64>,
    },
    /// Task started (subagent)
    TaskStarted {
        description: String,
        task_id: Option<String>,
    },
    /// Task completed
    TaskCompleted {
        task_id: Option<String>,
        result: String,
    },
    /// Permission request from Claude CLI
    PermissionRequest {
        /// Unique ID for this request
        request_id: String,
        /// Tool requesting permission
        tool: String,
        /// What the tool wants to do
        action: String,
        /// Full command/operation being requested
        command: Option<String>,
    },
    /// Permission response acknowledgement
    PermissionResponse { request_id: String, granted: bool },
}

/// Permission response to send back to Claude CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResponsePayload {
    /// The request ID being responded to
    pub request_id: String,
    /// Whether permission is granted
    pub granted: bool,
    /// Optional message for denial reason
    pub reason: Option<String>,
}
