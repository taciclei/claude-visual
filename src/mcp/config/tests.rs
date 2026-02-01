//! Tests for MCP configuration

#![cfg(test)]

use super::presets;
use super::types::{McpConfig, McpServerConfig};
use std::path::PathBuf;

#[test]
fn test_config_serialization() {
    let mut config = McpConfig::default();
    config.set_server(
        "filesystem".to_string(),
        McpServerConfig::new("npx")
            .with_args(["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]),
    );

    let json = serde_json::to_string_pretty(&config).unwrap();
    assert!(json.contains("mcpServers"));
    assert!(json.contains("filesystem"));
}

#[test]
fn test_config_deserialization() {
    let json = r#"{
        "mcpServers": {
            "test-server": {
                "command": "node",
                "args": ["server.js"],
                "env": {
                    "API_KEY": "secret"
                }
            }
        }
    }"#;

    let config: McpConfig = serde_json::from_str(json).unwrap();
    assert!(config.mcp_servers.contains_key("test-server"));

    let server = config.get_server("test-server").unwrap();
    assert_eq!(server.command, "node");
    assert_eq!(server.args, vec!["server.js"]);
    assert_eq!(server.env.get("API_KEY"), Some(&"secret".to_string()));
}

#[test]
fn test_enabled_servers() {
    let mut config = McpConfig::default();
    config.set_server(
        "enabled".to_string(),
        McpServerConfig::new("node").enabled(true),
    );
    config.set_server(
        "disabled".to_string(),
        McpServerConfig::new("node").enabled(false),
    );

    let enabled: Vec<_> = config.enabled_servers().collect();
    assert_eq!(enabled.len(), 1);
    assert_eq!(enabled[0].0, "enabled");
}

#[test]
fn test_presets() {
    let fs = presets::filesystem(vec![PathBuf::from("/tmp")]);
    assert_eq!(fs.command, "npx");
    assert!(fs.args.contains(&"-y".to_string()));

    let github = presets::github(Some("token123".to_string()));
    assert_eq!(github.env.get("GITHUB_PERSONAL_ACCESS_TOKEN"), Some(&"token123".to_string()));
}
