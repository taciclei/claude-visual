//! Tests for MCP server configuration editor

use super::types::EditingServerConfig;
use crate::mcp::McpServerConfig;
use std::collections::HashMap;

#[test]
fn test_editing_config_new() {
    let config = EditingServerConfig::new_server();
    assert!(config.is_new);
    assert!(config.name.is_empty());
    assert!(config.command.is_empty());
    assert!(config.enabled);
}

#[test]
fn test_editing_config_from_config() {
    let server_config = McpServerConfig {
        command: "npx".to_string(),
        args: vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-fs".to_string(),
        ],
        env: {
            let mut env = HashMap::new();
            env.insert("DEBUG".to_string(), "true".to_string());
            env
        },
        enabled: true,
        description: Some("File system server".to_string()),
        auto_approve: vec!["read_*".to_string()],
    };

    let editing = EditingServerConfig::from_config("filesystem".to_string(), &server_config);
    assert_eq!(editing.name, "filesystem");
    assert_eq!(editing.command, "npx");
    assert!(editing.args.contains("-y"));
    assert!(editing.env.contains("DEBUG=true"));
    assert_eq!(editing.description, "File system server");
    assert!(editing.auto_approve.contains("read_*"));
    assert!(!editing.is_new);
}

#[test]
fn test_editing_config_to_config() {
    let editing = EditingServerConfig {
        name: "test".to_string(),
        command: "python".to_string(),
        args: "server.py\n--port\n8080".to_string(),
        env: "API_KEY=secret\nDEBUG=1".to_string(),
        enabled: true,
        description: "Test server".to_string(),
        auto_approve: "read_*\nlist_*".to_string(),
        is_new: false,
    };

    let config = editing.to_config();
    assert_eq!(config.command, "python");
    assert_eq!(config.args, vec!["server.py", "--port", "8080"]);
    assert_eq!(config.env.get("API_KEY"), Some(&"secret".to_string()));
    assert_eq!(config.env.get("DEBUG"), Some(&"1".to_string()));
    assert!(config.enabled);
    assert_eq!(config.description, Some("Test server".to_string()));
    assert_eq!(config.auto_approve, vec!["read_*", "list_*"]);
}

#[test]
fn test_validate() {
    let mut config = EditingServerConfig::new_server();

    // Empty name
    assert!(config.validate().is_err());

    config.name = "test".to_string();
    // Empty command
    assert!(config.validate().is_err());

    config.command = "npx".to_string();
    // Now valid
    assert!(config.validate().is_ok());

    // Invalid env format
    config.env = "INVALID".to_string();
    assert!(config.validate().is_err());

    config.env = "VALID=value".to_string();
    assert!(config.validate().is_ok());
}
