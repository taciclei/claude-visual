//! Document management operations

use std::path::PathBuf;

use super::language::Language;
use super::state::LspManager;
use super::types::OpenDocument;

impl LspManager {
    /// Open a document
    pub async fn open_document(&self, path: &PathBuf, content: &str) -> Result<(), String> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let language = Language::from_extension(ext).ok_or("Unknown language")?;

        // Ensure server is running
        self.start_server(language).await?;

        let uri = format!("file://{}", path.display());

        // Track document
        {
            let mut docs = self.open_documents.lock().await;
            docs.insert(
                uri.clone(),
                OpenDocument {
                    uri: uri.clone(),
                    language,
                    version: 1,
                },
            );
        }

        // Notify server
        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&language) {
            let c = client.lock().await;
            c.did_open(&uri, language.language_id(), 1, content).await?;
        }

        Ok(())
    }

    /// Close a document
    pub async fn close_document(&self, path: &PathBuf) -> Result<(), String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let mut docs = self.open_documents.lock().await;
            docs.remove(&uri)
        };

        if let Some(doc) = doc {
            let clients = self.clients.lock().await;
            if let Some(client) = clients.get(&doc.language) {
                let c = client.lock().await;
                c.did_close(&uri).await?;
            }
        }

        Ok(())
    }

    /// Update document content
    pub async fn update_document(&self, path: &PathBuf, content: &str) -> Result<(), String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let mut docs = self.open_documents.lock().await;
            if let Some(doc) = docs.get_mut(&uri) {
                doc.version += 1;
                doc.clone()
            } else {
                return Err("Document not open".to_string());
            }
        };

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            c.did_change(&uri, doc.version, content).await?;
        }

        Ok(())
    }
}
