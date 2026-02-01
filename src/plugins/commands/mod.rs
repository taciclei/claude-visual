//! Slash Commands API for custom commands
//!
//! This module provides infrastructure for registering and executing
//! custom slash commands from extensions.

mod builtins;
mod registry;
mod types;

// Re-export public API
pub use registry::CommandRegistry;
pub use types::{CommandArg, CommandContext, CommandHandler, CommandResult, SlashCommand};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(CommandRegistry::parse_command("/help"), Some(("help", "")));
        assert_eq!(
            CommandRegistry::parse_command("/theme dark"),
            Some(("theme", "dark"))
        );
        assert_eq!(
            CommandRegistry::parse_command("/export markdown"),
            Some(("export", "markdown"))
        );
        assert_eq!(CommandRegistry::parse_command("hello"), None);
        assert_eq!(CommandRegistry::parse_command(""), None);
    }

    #[test]
    fn test_registry_builtins() {
        let registry = CommandRegistry::new();

        assert!(registry.get("help").is_some());
        assert!(registry.get("clear").is_some());
        assert!(registry.get("export").is_some());
        assert!(registry.get("theme").is_some());
        assert!(registry.get("project").is_some());
        assert!(registry.get("model").is_some());
        // MCP commands
        assert!(registry.get("mcp-servers").is_some());
        assert!(registry.get("mcp-tools").is_some());
        assert!(registry.get("mcp-resources").is_some());
        assert!(registry.get("mcp-prompts").is_some());
        assert!(registry.get("mcp-tool").is_some());
        assert!(registry.get("mcp-read").is_some());
        assert!(registry.get("mcp-prompt").is_some());
    }

    #[test]
    fn test_mcp_tool_command() {
        let registry = CommandRegistry::new();
        let ctx = CommandContext {
            cwd: None,
            project: None,
            user: None,
        };

        // Test valid command
        match registry.execute(
            "mcp-tool",
            "filesystem read_file {\"path\": \"/tmp/test.txt\"}",
            &ctx,
        ) {
            CommandResult::McpToolCall {
                server,
                tool,
                arguments,
            } => {
                assert_eq!(server, "filesystem");
                assert_eq!(tool, "read_file");
                assert!(arguments.is_some());
                assert!(arguments.unwrap().contains("path"));
            }
            _ => panic!("Expected McpToolCall result"),
        }

        // Test missing args
        match registry.execute("mcp-tool", "filesystem", &ctx) {
            CommandResult::Error(msg) => assert!(msg.contains("Usage")),
            _ => panic!("Expected Error result"),
        }
    }

    #[test]
    fn test_mcp_read_command() {
        let registry = CommandRegistry::new();
        let ctx = CommandContext {
            cwd: None,
            project: None,
            user: None,
        };

        match registry.execute("mcp-read", "filesystem file:///tmp/test.txt", &ctx) {
            CommandResult::McpResourceRead { server, uri } => {
                assert_eq!(server, "filesystem");
                assert_eq!(uri, "file:///tmp/test.txt");
            }
            _ => panic!("Expected McpResourceRead result"),
        }
    }

    #[test]
    fn test_mcp_list_commands() {
        let registry = CommandRegistry::new();
        let ctx = CommandContext {
            cwd: None,
            project: None,
            user: None,
        };

        // Test list servers
        match registry.execute("mcp-servers", "", &ctx) {
            CommandResult::McpListServers => {}
            _ => panic!("Expected McpListServers result"),
        }

        // Test list tools with filter
        match registry.execute("mcp-tools", "filesystem", &ctx) {
            CommandResult::McpListTools { server } => {
                assert_eq!(server, Some("filesystem".to_string()));
            }
            _ => panic!("Expected McpListTools result"),
        }

        // Test list tools without filter
        match registry.execute("mcp-tools", "", &ctx) {
            CommandResult::McpListTools { server } => {
                assert!(server.is_none());
            }
            _ => panic!("Expected McpListTools result"),
        }
    }

    #[test]
    fn test_execute_help() {
        let registry = CommandRegistry::new();
        let ctx = CommandContext {
            cwd: None,
            project: None,
            user: None,
        };

        match registry.execute("help", "", &ctx) {
            CommandResult::Text(text) => {
                assert!(text.contains("Available commands"));
                assert!(text.contains("MCP"));
                assert!(text.contains("/mcp-tool"));
            }
            _ => panic!("Expected Text result"),
        }
    }

    #[test]
    fn test_autocomplete() {
        let registry = CommandRegistry::new();

        let commands = registry.list_for_autocomplete("he");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "help");

        let commands = registry.list_for_autocomplete("");
        assert!(commands.len() >= 13); // At least the built-in commands including MCP

        // Test MCP autocomplete
        let commands = registry.list_for_autocomplete("mcp-");
        assert!(commands.len() >= 7); // All MCP commands
    }
}
