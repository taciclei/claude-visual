//! Server lifecycle management

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::lsp::client::{LspClient, LspClientConfig, LspEvent};

use super::events::LspManagerEvent;
use super::language::Language;
use super::state::LspManager;

impl LspManager {
    /// Start a language server
    pub async fn start_server(&self, language: Language) -> Result<(), String> {
        // Check if already running
        {
            let clients = self.clients.lock().await;
            if clients.contains_key(&language) {
                return Ok(());
            }
        }

        // Get server config
        let config = self.get_server_config(language)?;

        // Create event channel for this server
        let (lsp_tx, mut lsp_rx) = mpsc::unbounded_channel();

        // Start client
        let client = LspClient::new(config, lsp_tx).await?;

        // Initialize
        let client = Arc::new(Mutex::new(client));
        {
            let mut c = client.lock().await;
            c.initialize().await?;
        }

        // Store client
        {
            let mut clients = self.clients.lock().await;
            clients.insert(language, client.clone());
        }

        // Forward events
        let event_tx = self.event_tx.clone();
        tokio::spawn(async move {
            while let Some(event) = lsp_rx.recv().await {
                match event {
                    LspEvent::Diagnostics(uri, diagnostics) => {
                        let _ = event_tx.send(LspManagerEvent::Diagnostics(uri, diagnostics));
                    }
                    LspEvent::Error(err) => {
                        let _ = event_tx.send(LspManagerEvent::ServerError(language, err));
                    }
                    LspEvent::Exited(_) => {
                        let _ = event_tx.send(LspManagerEvent::ServerStopped(language));
                    }
                    _ => {}
                }
            }
        });

        self.event_tx
            .send(LspManagerEvent::ServerStarted(language))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Get server config for a language
    fn get_server_config(&self, language: Language) -> Result<LspClientConfig, String> {
        match language {
            Language::Rust => Ok(LspClientConfig::rust_analyzer(self.root_dir.clone())),
            Language::TypeScript | Language::JavaScript => {
                Ok(LspClientConfig::typescript(self.root_dir.clone()))
            }
            Language::Python => Ok(LspClientConfig::pyright(self.root_dir.clone())),
            Language::Go => Ok(LspClientConfig {
                command: "gopls".to_string(),
                args: vec![],
                cwd: Some(self.root_dir.clone()),
                env: HashMap::new(),
                root_uri: Some(format!("file://{}", self.root_dir.display())),
            }),
            _ => Err(format!("No LSP server configured for {:?}", language)),
        }
    }

    /// Stop a language server
    pub async fn stop_server(&self, language: Language) -> Result<(), String> {
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.remove(&language) {
            let mut c = client.lock().await;
            c.shutdown().await?;
        }
        Ok(())
    }

    /// Stop all servers
    pub async fn stop_all(&self) {
        let mut clients = self.clients.lock().await;
        for (_, client) in clients.drain() {
            let mut c = client.lock().await;
            let _ = c.shutdown().await;
        }
    }
}
