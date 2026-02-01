//! MCP Protocol definitions
//!
//! Defines JSON-RPC 2.0 types and MCP-specific structures.

mod error;
mod init;
mod jsonrpc;
mod prompts;
mod resources;
mod tools;

// Re-export all public types
pub use error::{McpError, McpErrorCode};
pub use init::{
    ClientInfo, InitializeParams, InitializeResult, LoggingCapability, McpCapabilities,
    PromptsCapability, ResourcesCapability, RootsCapability, SamplingCapability,
    ServerCapabilities, ServerInfo, ToolsCapability, MCP_PROTOCOL_VERSION,
};
pub use jsonrpc::{
    JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, JSONRPC_VERSION,
};
pub use prompts::{
    GetPromptResult, ListPromptsResult, McpPrompt, PromptArgument, PromptContent, PromptMessage,
};
pub use resources::{ListResourcesResult, McpResource, ResourceContents, ResourceReference};
pub use tools::{
    CallToolParams, CallToolResult, ListToolsResult, McpTool, ToolContent, ToolInputSchema,
};
