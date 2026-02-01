//! LSP Client document operations

use serde_json::json;

use super::core::LspClient;

impl LspClient {
    /// Open a document
    pub async fn did_open(
        &self,
        uri: &str,
        language_id: &str,
        version: i32,
        text: &str,
    ) -> Result<(), String> {
        self.notify(
            "textDocument/didOpen",
            Some(json!({
                "textDocument": {
                    "uri": uri,
                    "languageId": language_id,
                    "version": version,
                    "text": text
                }
            })),
        )
        .await
    }

    /// Close a document
    pub async fn did_close(&self, uri: &str) -> Result<(), String> {
        self.notify(
            "textDocument/didClose",
            Some(json!({
                "textDocument": {
                    "uri": uri
                }
            })),
        )
        .await
    }

    /// Notify document change
    pub async fn did_change(&self, uri: &str, version: i32, text: &str) -> Result<(), String> {
        self.notify(
            "textDocument/didChange",
            Some(json!({
                "textDocument": {
                    "uri": uri,
                    "version": version
                },
                "contentChanges": [{ "text": text }]
            })),
        )
        .await
    }
}
