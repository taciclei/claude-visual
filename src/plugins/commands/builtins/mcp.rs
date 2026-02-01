//! MCP (Model Context Protocol) built-in commands

use super::super::types::*;

/// Register MCP built-in commands
pub(crate) fn register_mcp_commands(register: impl Fn(SlashCommand, CommandHandler)) {
    // /mcp-servers - List MCP servers
    register(
        SlashCommand {
            name: "mcp-servers".to_string(),
            description: "List connected MCP servers".to_string(),
            help: Some(
                "Usage: /mcp-servers\n\nList all connected MCP servers and their status."
                    .to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![],
        },
        Box::new(|_args, _ctx| CommandResult::McpListServers),
    );

    // /mcp-tools - List MCP tools
    register(
        SlashCommand {
            name: "mcp-tools".to_string(),
            description: "List available MCP tools".to_string(),
            help: Some("Usage: /mcp-tools [server]\n\nList all available MCP tools. Optionally filter by server name.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "server".to_string(),
                required: false,
                description: Some("Server name to filter by".to_string()),
                completions: None,
            }],
        },
        Box::new(|args, _ctx| {
            let server = if args.trim().is_empty() {
                None
            } else {
                Some(args.trim().to_string())
            };
            CommandResult::McpListTools { server }
        }),
    );

    // /mcp-resources - List MCP resources
    register(
        SlashCommand {
            name: "mcp-resources".to_string(),
            description: "List available MCP resources".to_string(),
            help: Some("Usage: /mcp-resources [server]\n\nList all available MCP resources. Optionally filter by server name.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "server".to_string(),
                required: false,
                description: Some("Server name to filter by".to_string()),
                completions: None,
            }],
        },
        Box::new(|args, _ctx| {
            let server = if args.trim().is_empty() {
                None
            } else {
                Some(args.trim().to_string())
            };
            CommandResult::McpListResources { server }
        }),
    );

    // /mcp-prompts - List MCP prompts
    register(
        SlashCommand {
            name: "mcp-prompts".to_string(),
            description: "List available MCP prompts".to_string(),
            help: Some("Usage: /mcp-prompts [server]\n\nList all available MCP prompts. Optionally filter by server name.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "server".to_string(),
                required: false,
                description: Some("Server name to filter by".to_string()),
                completions: None,
            }],
        },
        Box::new(|args, _ctx| {
            let server = if args.trim().is_empty() {
                None
            } else {
                Some(args.trim().to_string())
            };
            CommandResult::McpListPrompts { server }
        }),
    );

    // /mcp-tool - Execute an MCP tool
    register(
        SlashCommand {
            name: "mcp-tool".to_string(),
            description: "Execute an MCP tool".to_string(),
            help: Some(
                "Usage: /mcp-tool <server> <tool> [args_json]\n\n\
                 Execute a tool from an MCP server.\n\n\
                 Examples:\n\
                   /mcp-tool filesystem read_file {\"path\": \"/tmp/test.txt\"}\n\
                   /mcp-tool github list_repos"
                    .to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![
                CommandArg {
                    name: "server".to_string(),
                    required: true,
                    description: Some("MCP server name".to_string()),
                    completions: None,
                },
                CommandArg {
                    name: "tool".to_string(),
                    required: true,
                    description: Some("Tool name".to_string()),
                    completions: None,
                },
                CommandArg {
                    name: "args".to_string(),
                    required: false,
                    description: Some("JSON arguments for the tool".to_string()),
                    completions: None,
                },
            ],
        },
        Box::new(|args, _ctx| {
            let parts: Vec<&str> = args.splitn(3, char::is_whitespace).collect();
            if parts.len() < 2 {
                return CommandResult::Error(
                    "Usage: /mcp-tool <server> <tool> [args_json]".to_string(),
                );
            }
            let server = parts[0].to_string();
            let tool = parts[1].to_string();
            let arguments = parts.get(2).map(|s| s.to_string());
            CommandResult::McpToolCall {
                server,
                tool,
                arguments,
            }
        }),
    );

    // /mcp-read - Read an MCP resource
    register(
        SlashCommand {
            name: "mcp-read".to_string(),
            description: "Read an MCP resource".to_string(),
            help: Some(
                "Usage: /mcp-read <server> <uri>\n\n\
                 Read a resource from an MCP server.\n\n\
                 Example:\n\
                   /mcp-read filesystem file:///home/user/doc.txt"
                    .to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![
                CommandArg {
                    name: "server".to_string(),
                    required: true,
                    description: Some("MCP server name".to_string()),
                    completions: None,
                },
                CommandArg {
                    name: "uri".to_string(),
                    required: true,
                    description: Some("Resource URI".to_string()),
                    completions: None,
                },
            ],
        },
        Box::new(|args, _ctx| {
            let parts: Vec<&str> = args.splitn(2, char::is_whitespace).collect();
            if parts.len() < 2 {
                return CommandResult::Error("Usage: /mcp-read <server> <uri>".to_string());
            }
            let server = parts[0].to_string();
            let uri = parts[1].to_string();
            CommandResult::McpResourceRead { server, uri }
        }),
    );

    // /mcp-prompt - Use an MCP prompt
    register(
        SlashCommand {
            name: "mcp-prompt".to_string(),
            description: "Use an MCP prompt".to_string(),
            help: Some(
                "Usage: /mcp-prompt <server> <prompt> [args_json]\n\n\
                 Get a prompt from an MCP server and insert it.\n\n\
                 Example:\n\
                   /mcp-prompt memory recall {\"topic\": \"rust patterns\"}"
                    .to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![
                CommandArg {
                    name: "server".to_string(),
                    required: true,
                    description: Some("MCP server name".to_string()),
                    completions: None,
                },
                CommandArg {
                    name: "prompt".to_string(),
                    required: true,
                    description: Some("Prompt name".to_string()),
                    completions: None,
                },
                CommandArg {
                    name: "args".to_string(),
                    required: false,
                    description: Some("JSON arguments for the prompt".to_string()),
                    completions: None,
                },
            ],
        },
        Box::new(|args, _ctx| {
            let parts: Vec<&str> = args.splitn(3, char::is_whitespace).collect();
            if parts.len() < 2 {
                return CommandResult::Error(
                    "Usage: /mcp-prompt <server> <prompt> [args_json]".to_string(),
                );
            }
            let server = parts[0].to_string();
            let prompt = parts[1].to_string();
            let arguments = parts.get(2).map(|s| s.to_string());
            CommandResult::McpPromptGet {
                server,
                prompt,
                arguments,
            }
        }),
    );
}
