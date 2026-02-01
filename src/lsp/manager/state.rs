//! LSP Manager state and initialization

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::lsp::client::LspClient;

use super::events::LspManagerEvent;
use super::language::Language;
use super::types::OpenDocument;

/// LSP Manager for managing multiple language servers
pub struct LspManager {
    /// Active clients by language
    pub(super) clients: Arc<Mutex<HashMap<Language, Arc<Mutex<LspClient>>>>>,
    /// Root directory
    pub(super) root_dir: PathBuf,
    /// Open documents by URI
    pub(super) open_documents: Arc<Mutex<HashMap<String, OpenDocument>>>,
    /// Event sender
    pub(super) event_tx: mpsc::UnboundedSender<LspManagerEvent>,
    /// Event receiver
    event_rx: Option<mpsc::UnboundedReceiver<LspManagerEvent>>,
}

impl LspManager {
    /// Create a new LSP manager
    pub fn new(root_dir: PathBuf) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            root_dir,
            open_documents: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            event_rx: Some(event_rx),
        }
    }

    /// Take the event receiver (can only be called once)
    pub fn take_event_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<LspManagerEvent>> {
        self.event_rx.take()
    }

    /// Get all running servers
    pub async fn running_servers(&self) -> Vec<Language> {
        let clients = self.clients.lock().await;
        clients.keys().copied().collect()
    }
}
