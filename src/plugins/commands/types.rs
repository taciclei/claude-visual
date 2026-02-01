//! Core types for the slash commands API

use std::path::PathBuf;

/// A registered slash command
#[derive(Debug, Clone)]
pub struct SlashCommand {
    /// Command name (without the leading /)
    pub name: String,
    /// Short description for autocomplete
    pub description: String,
    /// Long help text
    pub help: Option<String>,
    /// Extension that registered this command
    pub extension_id: Option<String>,
    /// Whether this is a built-in command
    pub is_builtin: bool,
    /// Argument specification
    pub args: Vec<CommandArg>,
}

/// Command argument specification
#[derive(Debug, Clone)]
pub struct CommandArg {
    /// Argument name
    pub name: String,
    /// Whether this argument is required
    pub required: bool,
    /// Description for help
    pub description: Option<String>,
    /// Possible values for autocomplete
    pub completions: Option<Vec<String>>,
}

/// Result of executing a command
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// Text output to display in chat
    Text(String),
    /// Markdown output to render
    Markdown(String),
    /// Code output with language hint
    Code { language: String, content: String },
    /// Error message
    Error(String),
    /// No visible output
    Silent,
    /// MCP tool call request (server, tool, args_json)
    McpToolCall {
        server: String,
        tool: String,
        arguments: Option<String>,
    },
    /// MCP resource read request (server, uri)
    McpResourceRead { server: String, uri: String },
    /// MCP prompt request (server, prompt, args_json)
    McpPromptGet {
        server: String,
        prompt: String,
        arguments: Option<String>,
    },
    /// List MCP servers
    McpListServers,
    /// List MCP tools (optionally filtered by server)
    McpListTools { server: Option<String> },
    /// List MCP resources (optionally filtered by server)
    McpListResources { server: Option<String> },
    /// List MCP prompts (optionally filtered by server)
    McpListPrompts { server: Option<String> },
}

/// Command execution context
pub struct CommandContext {
    /// Current working directory
    pub cwd: Option<PathBuf>,
    /// Current project path
    pub project: Option<PathBuf>,
    /// User who invoked the command
    pub user: Option<String>,
}

/// Handler function type for built-in commands
pub type CommandHandler = Box<dyn Fn(&str, &CommandContext) -> CommandResult + Send + Sync>;
