//! Process management for MCP client

use super::super::protocol::McpError;
use super::types::McpClient;
use std::io::BufReader;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

impl McpClient {
    /// Start the MCP server process
    pub fn start(&mut self) -> Result<(), McpError> {
        start(self)
    }

    /// Stop the MCP server process
    pub fn stop(&mut self) -> Result<(), McpError> {
        stop(self)
    }
}

/// Start the MCP server process
pub(super) fn start(client: &mut McpClient) -> Result<(), McpError> {
    if client.process.is_some() {
        return Err(McpError::Connection("Server already running".into()));
    }

    let mut cmd = Command::new(&client.config.command);
    cmd.args(&client.config.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Set environment variables
    for (key, value) in &client.config.env {
        cmd.env(key, value);
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| McpError::Connection(format!("Failed to spawn server: {}", e)))?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| McpError::Connection("Failed to get stdin".into()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| McpError::Connection("Failed to get stdout".into()))?;

    client.process = Some(child);
    client.stdin = Some(Arc::new(Mutex::new(stdin)));
    client.stdout = Some(BufReader::new(stdout));

    Ok(())
}

/// Stop the MCP server process
pub(super) fn stop(client: &mut McpClient) -> Result<(), McpError> {
    if let Some(mut process) = client.process.take() {
        // Try to kill the process
        let _ = process.kill();
        let _ = process.wait();
    }

    client.stdin = None;
    client.stdout = None;
    client.initialized = false;
    client.server_info = None;
    client.capabilities = None;
    client.tools.clear();
    client.resources.clear();
    client.prompts.clear();

    Ok(())
}
