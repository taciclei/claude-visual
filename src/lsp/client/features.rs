//! LSP Client feature implementations

use serde_json::{json, Value};

use crate::lsp::protocol::*;

use super::core::LspClient;

impl LspClient {
    /// Get completions at position
    pub async fn completion(
        &self,
        uri: &str,
        position: Position,
    ) -> Result<Vec<CompletionItem>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "position": position
        });

        let result: Value = self
            .request("textDocument/completion", Some(params))
            .await?;

        // Handle both CompletionList and CompletionItem[]
        if let Some(items) = result.get("items") {
            serde_json::from_value(items.clone()).map_err(|e| e.to_string())
        } else if result.is_array() {
            serde_json::from_value(result).map_err(|e| e.to_string())
        } else {
            Ok(vec![])
        }
    }

    /// Get hover information
    pub async fn hover(&self, uri: &str, position: Position) -> Result<Option<Hover>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "position": position
        });

        let result: Value = self.request("textDocument/hover", Some(params)).await?;

        if result.is_null() {
            return Ok(None);
        }

        serde_json::from_value(result)
            .map(Some)
            .map_err(|e| e.to_string())
    }

    /// Go to definition
    pub async fn definition(&self, uri: &str, position: Position) -> Result<Vec<Location>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "position": position
        });

        let result: Value = self
            .request("textDocument/definition", Some(params))
            .await?;

        // Handle Location, Location[], or null
        if result.is_null() {
            return Ok(vec![]);
        }

        if result.is_array() {
            serde_json::from_value(result).map_err(|e| e.to_string())
        } else {
            let loc: Location = serde_json::from_value(result).map_err(|e| e.to_string())?;
            Ok(vec![loc])
        }
    }

    /// Find references
    pub async fn references(
        &self,
        uri: &str,
        position: Position,
        include_declaration: bool,
    ) -> Result<Vec<Location>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "position": position,
            "context": { "includeDeclaration": include_declaration }
        });

        let result: Value = self
            .request("textDocument/references", Some(params))
            .await?;

        if result.is_null() {
            return Ok(vec![]);
        }

        serde_json::from_value(result).map_err(|e| e.to_string())
    }

    /// Get document symbols
    pub async fn document_symbols(&self, uri: &str) -> Result<Vec<DocumentSymbol>, String> {
        let params = json!({
            "textDocument": { "uri": uri }
        });

        let result: Value = self
            .request("textDocument/documentSymbol", Some(params))
            .await?;

        if result.is_null() {
            return Ok(vec![]);
        }

        serde_json::from_value(result).map_err(|e| e.to_string())
    }

    /// Get signature help
    pub async fn signature_help(
        &self,
        uri: &str,
        position: Position,
    ) -> Result<Option<SignatureHelp>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "position": position
        });

        let result: Value = self
            .request("textDocument/signatureHelp", Some(params))
            .await?;

        if result.is_null() {
            return Ok(None);
        }

        serde_json::from_value(result)
            .map(Some)
            .map_err(|e| e.to_string())
    }

    /// Get code actions
    pub async fn code_actions(
        &self,
        uri: &str,
        range: Range,
        diagnostics: Vec<Diagnostic>,
    ) -> Result<Vec<CodeAction>, String> {
        let params = json!({
            "textDocument": { "uri": uri },
            "range": range,
            "context": { "diagnostics": diagnostics }
        });

        let result: Value = self
            .request("textDocument/codeAction", Some(params))
            .await?;

        if result.is_null() {
            return Ok(vec![]);
        }

        serde_json::from_value(result).map_err(|e| e.to_string())
    }
}
