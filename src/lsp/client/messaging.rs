//! LSP Client messaging implementation

use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot;

use super::core::LspClient;
use super::types::*;

impl LspClient {
    /// Send a request and wait for response
    pub(crate) async fn request<T: for<'de> Deserialize<'de>>(&self, method: &str, params: Option<Value>) -> Result<T, String> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id,
            method: method.to_string(),
            params,
        };

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending.lock().await;
            pending.insert(id, PendingRequest { sender: tx });
        }

        self.send_message(&request).await?;

        let result = rx.await.map_err(|_| "Request cancelled")?;
        let value = result?;

        serde_json::from_value(value).map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Send a notification (no response expected)
    pub(crate) async fn notify(&self, method: &str, params: Option<Value>) -> Result<(), String> {
        let notification = JsonRpcNotification {
            jsonrpc: "2.0",
            method: method.to_string(),
            params,
        };

        self.send_message(&notification).await
    }

    /// Send a message to the server
    async fn send_message<T: Serialize>(&self, message: &T) -> Result<(), String> {
        let content = serde_json::to_string(message).map_err(|e| e.to_string())?;
        let header = format!("Content-Length: {}\r\n\r\n", content.len());

        let mut stdin = self.stdin.lock().await;
        stdin
            .write_all(header.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdin
            .write_all(content.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
