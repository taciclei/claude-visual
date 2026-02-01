//! LSP protocol operations

use std::path::PathBuf;

use crate::lsp::protocol::*;

use super::state::LspManager;

impl LspManager {
    /// Get completions
    pub async fn completions(&self, path: &PathBuf, line: u32, character: u32) -> Result<Vec<CompletionItem>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.completion(&uri, Position::new(line, character)).await;
        }

        Ok(vec![])
    }

    /// Get hover info
    pub async fn hover(&self, path: &PathBuf, line: u32, character: u32) -> Result<Option<Hover>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.hover(&uri, Position::new(line, character)).await;
        }

        Ok(None)
    }

    /// Go to definition
    pub async fn definition(&self, path: &PathBuf, line: u32, character: u32) -> Result<Vec<Location>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.definition(&uri, Position::new(line, character)).await;
        }

        Ok(vec![])
    }

    /// Find references
    pub async fn references(&self, path: &PathBuf, line: u32, character: u32) -> Result<Vec<Location>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.references(&uri, Position::new(line, character), true).await;
        }

        Ok(vec![])
    }

    /// Get document symbols
    pub async fn document_symbols(&self, path: &PathBuf) -> Result<Vec<DocumentSymbol>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.document_symbols(&uri).await;
        }

        Ok(vec![])
    }

    /// Get code actions
    pub async fn code_actions(
        &self,
        path: &PathBuf,
        range: Range,
        diagnostics: Vec<Diagnostic>,
    ) -> Result<Vec<CodeAction>, String> {
        let uri = format!("file://{}", path.display());

        let doc = {
            let docs = self.open_documents.lock().await;
            docs.get(&uri).cloned()
        };

        let doc = doc.ok_or("Document not open")?;

        let clients = self.clients.lock().await;
        if let Some(client) = clients.get(&doc.language) {
            let c = client.lock().await;
            return c.code_actions(&uri, range, diagnostics).await;
        }

        Ok(vec![])
    }
}
