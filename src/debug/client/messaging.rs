//! DAP Message Handling
//!
//! Reading and writing DAP protocol messages.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::ChildStdout;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use serde_json::Value as JsonValue;
use tokio::sync::{mpsc, Mutex};

use super::super::protocol::*;
use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Read messages from adapter
    pub(crate) fn read_messages(
        stdout: ChildStdout,
        pending_requests: Arc<Mutex<HashMap<i64, PendingRequest>>>,
        event_sender: mpsc::UnboundedSender<DapEvent>,
    ) {
        let mut reader = BufReader::new(stdout);
        let mut headers = String::new();

        loop {
            headers.clear();

            // Read headers
            let mut content_length: Option<usize> = None;

            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => return, // EOF
                    Ok(_) => {
                        let line = line.trim();
                        if line.is_empty() {
                            break;
                        }
                        if let Some(len_str) = line.strip_prefix("Content-Length: ") {
                            content_length = len_str.parse().ok();
                        }
                    }
                    Err(_) => return,
                }
            }

            // Read content
            if let Some(length) = content_length {
                let mut content = vec![0u8; length];
                if reader.read_exact(&mut content).is_err() {
                    return;
                }

                // Parse message
                if let Ok(content_str) = String::from_utf8(content) {
                    if let Ok(message) = serde_json::from_str::<DapMessage>(&content_str) {
                        match message {
                            DapMessage::Response(response) => {
                                // Handle response
                                let pending = pending_requests.blocking_lock();
                                if let Some(request) = pending.get(&response.request_seq) {
                                    // Move out of pending to satisfy borrow checker
                                    drop(pending);
                                    let mut pending = pending_requests.blocking_lock();
                                    if let Some(request) = pending.remove(&response.request_seq) {
                                        let _ = request.sender.send(Ok(response));
                                    }
                                }
                            }
                            DapMessage::Event(event) => {
                                let _ = event_sender.send(event);
                            }
                            DapMessage::Request(_) => {
                                // Reverse requests from adapter (e.g., runInTerminal)
                                // TODO: Handle these
                            }
                        }
                    }
                }
            }
        }
    }

    /// Send a request to the adapter
    pub async fn send_request(
        &mut self,
        command: &str,
        arguments: Option<JsonValue>,
    ) -> Result<DapResponse, DapClientError> {
        let stdin = self.stdin.as_mut().ok_or(DapClientError::NotInitialized)?;

        let seq = self.seq.fetch_add(1, Ordering::SeqCst);

        let request = DapRequest::new(seq, command, arguments);
        let message = DapMessage::Request(request);

        let content = serde_json::to_string(&message)
            .map_err(|e| DapClientError::ProtocolError(e.to_string()))?;

        let header = format!("Content-Length: {}\r\n\r\n", content.len());

        stdin
            .write_all(header.as_bytes())
            .map_err(|e| DapClientError::IoError(e.to_string()))?;
        stdin
            .write_all(content.as_bytes())
            .map_err(|e| DapClientError::IoError(e.to_string()))?;
        stdin.flush().map_err(|e| DapClientError::IoError(e.to_string()))?;

        // Wait for response
        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(seq, PendingRequest { sender: tx });
        }

        // Wait with timeout
        match tokio::time::timeout(
            std::time::Duration::from_millis(self.config.timeout_ms),
            rx,
        )
        .await
        {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => Err(DapClientError::ProtocolError("Channel closed".to_string())),
            Err(_) => {
                // Remove from pending
                let mut pending = self.pending_requests.lock().await;
                pending.remove(&seq);
                Err(DapClientError::Timeout)
            }
        }
    }
}
