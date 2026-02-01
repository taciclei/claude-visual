//! General built-in commands

use super::super::types::*;

/// Register general built-in commands (help, clear, export, theme, project, model)
pub(crate) fn register_general_commands(register: impl Fn(SlashCommand, CommandHandler)) {
    // /help - Show available commands
    register(
        SlashCommand {
            name: "help".to_string(),
            description: "Show available commands".to_string(),
            help: Some("Usage: /help [command]\n\nShow all available commands or detailed help for a specific command.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "command".to_string(),
                required: false,
                description: Some("Command to get help for".to_string()),
                completions: None,
            }],
        },
        Box::new(|args, _ctx| {
            if args.trim().is_empty() {
                CommandResult::Text(
                    "Available commands:\n\
                    \n  General:\n\
                      /help - Show this help\n\
                      /clear - Clear conversation\n\
                      /export - Export conversation\n\
                      /theme - Change theme\n\
                      /project - Switch project\n\
                      /model - Switch Claude model\n\
                    \n  MCP (Model Context Protocol):\n\
                      /mcp-servers - List connected servers\n\
                      /mcp-tools [server] - List available tools\n\
                      /mcp-resources [server] - List available resources\n\
                      /mcp-prompts [server] - List available prompts\n\
                      /mcp-tool <server> <tool> [args] - Execute a tool\n\
                      /mcp-read <server> <uri> - Read a resource\n\
                      /mcp-prompt <server> <prompt> [args] - Use a prompt".to_string()
                )
            } else {
                CommandResult::Text(format!("Help for /{}: (detailed help would go here)", args.trim()))
            }
        }),
    );

    // /clear - Clear conversation
    register(
        SlashCommand {
            name: "clear".to_string(),
            description: "Clear current conversation".to_string(),
            help: Some(
                "Usage: /clear\n\nClears all messages from the current conversation.".to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![],
        },
        Box::new(|_args, _ctx| CommandResult::Silent),
    );

    // /export - Export conversation
    register(
        SlashCommand {
            name: "export".to_string(),
            description: "Export conversation to file".to_string(),
            help: Some("Usage: /export [format]\n\nExport the current conversation. Formats: markdown (default), json".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "format".to_string(),
                required: false,
                description: Some("Export format".to_string()),
                completions: Some(vec!["markdown".to_string(), "json".to_string()]),
            }],
        },
        Box::new(|_args, _ctx| {
            CommandResult::Silent
        }),
    );

    // /theme - Change theme
    register(
        SlashCommand {
            name: "theme".to_string(),
            description: "Change color theme".to_string(),
            help: Some("Usage: /theme [name]\n\nSwitch to a different color theme. Use /theme list to see available themes.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "name".to_string(),
                required: false,
                description: Some("Theme name or 'list'".to_string()),
                completions: Some(vec!["dark".to_string(), "light".to_string(), "list".to_string()]),
            }],
        },
        Box::new(|args, _ctx| {
            let theme = args.trim();
            if theme.is_empty() || theme == "list" {
                CommandResult::Text("Available themes:\n  dark - Dark mode (default)\n  light - Light mode".to_string())
            } else {
                CommandResult::Text(format!("Switched to {} theme", theme))
            }
        }),
    );

    // /project - Switch project
    register(
        SlashCommand {
            name: "project".to_string(),
            description: "Switch to a different project".to_string(),
            help: Some(
                "Usage: /project [path]\n\nSwitch the working directory to a different project."
                    .to_string(),
            ),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "path".to_string(),
                required: false,
                description: Some("Project path".to_string()),
                completions: None,
            }],
        },
        Box::new(|args, ctx| {
            if args.trim().is_empty() {
                if let Some(project) = &ctx.project {
                    CommandResult::Text(format!("Current project: {}", project.display()))
                } else {
                    CommandResult::Text("No project selected".to_string())
                }
            } else {
                CommandResult::Silent
            }
        }),
    );

    // /model - Switch Claude model
    register(
        SlashCommand {
            name: "model".to_string(),
            description: "Switch Claude model".to_string(),
            help: Some("Usage: /model [name]\n\nSwitch to a different Claude model.".to_string()),
            extension_id: None,
            is_builtin: true,
            args: vec![CommandArg {
                name: "name".to_string(),
                required: false,
                description: Some("Model name".to_string()),
                completions: Some(vec![
                    "claude-3-opus".to_string(),
                    "claude-3-sonnet".to_string(),
                    "claude-3-haiku".to_string(),
                ]),
            }],
        },
        Box::new(|args, _ctx| {
            if args.trim().is_empty() {
                CommandResult::Text("Available models:\n  claude-3-opus - Most capable\n  claude-3-sonnet - Balanced\n  claude-3-haiku - Fast".to_string())
            } else {
                CommandResult::Text(format!("Switched to model: {}", args.trim()))
            }
        }),
    );
}
