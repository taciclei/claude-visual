//! JSON-RPC messaging for MCP client

use super::super::protocol::*;
use super::types::McpClient;
use std::io::{BufRead, Write};
use std::sync::atomic::Ordering;

/// Send a JSON-RPC request and wait for response
pub(super) fn send_request<P, R>(
    client: &mut McpClient,
    method: &str,
    params: Option<P>,
) -> Result<R, McpError>
where
    P: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let id = client.request_id.fetch_add(1, Ordering::SeqCst);

    let params_value = params
        .map(|p| serde_json::to_value(p))
        .transpose()
        .map_err(|e| McpError::Protocol(format!("Failed to serialize params: {}", e)))?;

    let request = JsonRpcRequest::new(id, method, params_value);
    let request_json = serde_json::to_string(&request)
        .map_err(|e| McpError::Protocol(format!("Failed to serialize request: {}", e)))?;

    // Write request
    {
        let stdin = client
            .stdin
            .as_ref()
            .ok_or_else(|| McpError::Connection("No stdin available".into()))?;
        let mut stdin = stdin
            .lock()
            .map_err(|_| McpError::Connection("Failed to lock stdin".into()))?;

        writeln!(stdin, "{}", request_json)
            .map_err(|e| McpError::Io(format!("Failed to write request: {}", e)))?;
        stdin
            .flush()
            .map_err(|e| McpError::Io(format!("Failed to flush: {}", e)))?;
    }

    // Read response
    let stdout = client
        .stdout
        .as_mut()
        .ok_or_else(|| McpError::Connection("No stdout available".into()))?;

    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = stdout
            .read_line(&mut line)
            .map_err(|e| McpError::Io(format!("Failed to read response: {}", e)))?;

        if bytes_read == 0 {
            return Err(McpError::Connection("Server closed connection".into()));
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Try to parse as response
        let response: JsonRpcResponse = serde_json::from_str(line)
            .map_err(|e| McpError::Protocol(format!("Failed to parse response: {}", e)))?;

        // Check if this is our response
        if response.id != id {
            // Could be a notification or response to different request
            continue;
        }

        // Check for error
        if let Some(error) = response.error {
            return Err(McpError::Server {
                code: error.code,
                message: error.message,
            });
        }

        // Parse result
        let result = response
            .result
            .ok_or_else(|| McpError::Protocol("Missing result in response".into()))?;

        return serde_json::from_value(result)
            .map_err(|e| McpError::Protocol(format!("Failed to parse result: {}", e)));
    }
}

/// Send a JSON-RPC notification (no response expected)
pub(super) fn send_notification<P>(
    client: &mut McpClient,
    method: &str,
    params: Option<P>,
) -> Result<(), McpError>
where
    P: serde::Serialize,
{
    let params_value = params
        .map(|p| serde_json::to_value(p))
        .transpose()
        .map_err(|e| McpError::Protocol(format!("Failed to serialize params: {}", e)))?;

    let notification = JsonRpcNotification::new(method, params_value);
    let notification_json = serde_json::to_string(&notification)
        .map_err(|e| McpError::Protocol(format!("Failed to serialize notification: {}", e)))?;

    let stdin = client
        .stdin
        .as_ref()
        .ok_or_else(|| McpError::Connection("No stdin available".into()))?;
    let mut stdin = stdin
        .lock()
        .map_err(|_| McpError::Connection("Failed to lock stdin".into()))?;

    writeln!(stdin, "{}", notification_json)
        .map_err(|e| McpError::Io(format!("Failed to write notification: {}", e)))?;
    stdin
        .flush()
        .map_err(|e| McpError::Io(format!("Failed to flush: {}", e)))?;

    Ok(())
}
