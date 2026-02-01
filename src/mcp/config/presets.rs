//! Predefined server configurations for common MCP servers

use super::types::McpServerConfig;
use std::path::{Path, PathBuf};

/// Filesystem MCP server
pub fn filesystem(allowed_dirs: Vec<PathBuf>) -> McpServerConfig {
    let dirs: Vec<String> = allowed_dirs
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-filesystem"])
        .with_args(dirs)
        .with_description("Filesystem access MCP server")
}

/// GitHub MCP server
pub fn github(token: Option<String>) -> McpServerConfig {
    let mut config = McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-github"])
        .with_description("GitHub API MCP server");

    if let Some(token) = token {
        config = config.with_env("GITHUB_PERSONAL_ACCESS_TOKEN", token);
    }

    config
}

/// SQLite MCP server
pub fn sqlite(db_path: &Path) -> McpServerConfig {
    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-sqlite"])
        .with_arg(db_path.to_string_lossy().to_string())
        .with_description("SQLite database MCP server")
}

/// Brave Search MCP server
pub fn brave_search(api_key: String) -> McpServerConfig {
    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-brave-search"])
        .with_env("BRAVE_API_KEY", api_key)
        .with_description("Brave Search MCP server")
}

/// Fetch MCP server (HTTP requests)
pub fn fetch() -> McpServerConfig {
    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-fetch"])
        .with_description("HTTP fetch MCP server")
}

/// Memory MCP server (knowledge graph)
pub fn memory() -> McpServerConfig {
    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-memory"])
        .with_description("Memory/Knowledge graph MCP server")
}

/// Puppeteer MCP server (browser automation)
pub fn puppeteer() -> McpServerConfig {
    McpServerConfig::new("npx")
        .with_args(["-y", "@modelcontextprotocol/server-puppeteer"])
        .with_description("Puppeteer browser automation MCP server")
}
