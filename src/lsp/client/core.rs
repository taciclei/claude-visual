//! LSP Client core implementation

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, Mutex};

use crate::lsp::protocol::*;

use super::types::*;

/// LSP Client for communicating with a language server
pub struct LspClient {
    /// Server process
    pub(crate) process: Child,
    /// Request ID counter
    pub(crate) next_id: AtomicU64,
    /// Pending requests
    pub(crate) pending: Arc<Mutex<HashMap<u64, PendingRequest>>>,
    /// Stdin for sending messages
    pub(crate) stdin: Arc<Mutex<tokio::process::ChildStdin>>,
    /// Event sender
    pub(crate) event_tx: mpsc::UnboundedSender<LspEvent>,
    /// Server capabilities
    pub(crate) capabilities: Arc<Mutex<Option<ServerCapabilities>>>,
    /// Root URI
    pub(crate) root_uri: Option<String>,
    /// Whether initialized
    pub(crate) initialized: Arc<Mutex<bool>>,
}

impl LspClient {
    /// Create and start a new LSP client
    pub async fn new(
        config: LspClientConfig,
        event_tx: mpsc::UnboundedSender<LspEvent>,
    ) -> Result<Self, String> {
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(cwd) = &config.cwd {
            cmd.current_dir(cwd);
        }

        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        let mut process = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn LSP server: {}", e))?;

        let stdin = process
            .stdin
            .take()
            .ok_or("Failed to get stdin")?;
        let stdout = process
            .stdout
            .take()
            .ok_or("Failed to get stdout")?;

        let pending: Arc<Mutex<HashMap<u64, PendingRequest>>> = Arc::new(Mutex::new(HashMap::new()));
        let capabilities = Arc::new(Mutex::new(None));

        // Spawn reader task
        let pending_clone = pending.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut headers = String::new();

            loop {
                headers.clear();

                // Read headers
                let mut content_length: Option<usize> = None;
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line).await {
                        Ok(0) => return, // EOF
                        Ok(_) => {
                            if line == "\r\n" || line == "\n" {
                                break;
                            }
                            if line.to_lowercase().starts_with("content-length:") {
                                if let Some(len_str) = line.split(':').nth(1) {
                                    content_length = len_str.trim().parse().ok();
                                }
                            }
                        }
                        Err(_) => return,
                    }
                }

                // Read content
                let len = match content_length {
                    Some(l) => l,
                    None => continue,
                };

                let mut content = vec![0u8; len];
                if reader.read_exact(&mut content).await.is_err() {
                    return;
                }

                let content_str = match String::from_utf8(content) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                // Parse response
                if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&content_str) {
                    if let Some(id) = response.id {
                        // This is a response to a request
                        let mut pending = pending_clone.lock().await;
                        if let Some(req) = pending.remove(&id) {
                            let result = if let Some(err) = response.error {
                                Err(format!("LSP error {}: {}", err.code, err.message))
                            } else {
                                Ok(response.result.unwrap_or(Value::Null))
                            };
                            let _ = req.sender.send(result);
                        }
                    } else {
                        // This is a notification
                        if let Ok(notif) = serde_json::from_str::<Value>(&content_str) {
                            if let Some(method) = notif.get("method").and_then(|m| m.as_str()) {
                                match method {
                                    "textDocument/publishDiagnostics" => {
                                        if let Some(params) = notif.get("params") {
                                            if let Ok(diag_params) = serde_json::from_value::<PublishDiagnosticsParams>(params.clone()) {
                                                let _ = event_tx_clone.send(LspEvent::Diagnostics(
                                                    diag_params.uri,
                                                    diag_params.diagnostics,
                                                ));
                                            }
                                        }
                                    }
                                    "window/logMessage" => {
                                        if let Some(msg) = notif.get("params").and_then(|p| p.get("message")).and_then(|m| m.as_str()) {
                                            let _ = event_tx_clone.send(LspEvent::LogMessage(msg.to_string()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            process,
            next_id: AtomicU64::new(1),
            pending,
            stdin: Arc::new(Mutex::new(stdin)),
            event_tx,
            capabilities,
            root_uri: config.root_uri,
            initialized: Arc::new(Mutex::new(false)),
        })
    }

    /// Initialize the server
    pub async fn initialize(&self) -> Result<ServerCapabilities, String> {
        let params = InitializeParams {
            process_id: Some(std::process::id()),
            root_uri: self.root_uri.clone(),
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    completion: Some(CompletionClientCapabilities {
                        snippet_support: Some(true),
                    }),
                    hover: Some(HoverClientCapabilities {
                        content_format: Some(vec![MarkupKind::Markdown, MarkupKind::PlainText]),
                    }),
                }),
            },
        };

        let result: InitializeResult = self.request("initialize", Some(serde_json::to_value(params).unwrap())).await?;

        // Send initialized notification
        self.notify("initialized", Some(json!({}))).await?;

        // Store capabilities
        *self.capabilities.lock().await = Some(result.capabilities.clone());
        *self.initialized.lock().await = true;

        self.event_tx
            .send(LspEvent::Initialized(result.capabilities.clone()))
            .map_err(|e| e.to_string())?;

        Ok(result.capabilities)
    }

    /// Get server capabilities
    pub async fn capabilities(&self) -> Option<ServerCapabilities> {
        self.capabilities.lock().await.clone()
    }

    /// Check if server is initialized
    pub async fn is_initialized(&self) -> bool {
        *self.initialized.lock().await
    }

    /// Shutdown the server
    pub async fn shutdown(&mut self) -> Result<(), String> {
        // Send shutdown request
        let _: Value = self.request("shutdown", None).await?;

        // Send exit notification
        self.notify("exit", None).await?;

        // Wait for process to exit
        let _ = self.process.wait().await;

        self.event_tx
            .send(LspEvent::Exited(None))
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
