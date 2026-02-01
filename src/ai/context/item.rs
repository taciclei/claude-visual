//! ContextItem implementation

use super::types::{ContextItem, ContextItemType};
use super::utils::{estimate_tokens, mime_to_language, uri_to_language};
use std::collections::HashMap;
use std::path::PathBuf;

impl ContextItem {
    /// Create a new file context item
    pub fn file(path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        let path = path.into();
        let content = content.into();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let language = path.extension().map(|e| e.to_string_lossy().to_string());
        let token_count = estimate_tokens(&content);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::File,
            name,
            path: Some(path),
            content,
            start_line: None,
            end_line: None,
            language,
            pinned: false,
            token_count,
            metadata: HashMap::new(),
            added_at: chrono::Utc::now(),
        }
    }

    /// Create a snippet context item
    pub fn snippet(
        path: impl Into<PathBuf>,
        content: impl Into<String>,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        let path = path.into();
        let content = content.into();
        let name = format!(
            "{}:{}–{}",
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "snippet".to_string()),
            start_line,
            end_line
        );
        let language = path.extension().map(|e| e.to_string_lossy().to_string());
        let token_count = estimate_tokens(&content);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::Snippet,
            name,
            path: Some(path),
            content,
            start_line: Some(start_line),
            end_line: Some(end_line),
            language,
            pinned: false,
            token_count,
            metadata: HashMap::new(),
            added_at: chrono::Utc::now(),
        }
    }

    /// Create a diff context item
    pub fn diff(name: impl Into<String>, diff_content: impl Into<String>) -> Self {
        let content = diff_content.into();
        let token_count = estimate_tokens(&content);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::Diff,
            name: name.into(),
            path: None,
            content,
            start_line: None,
            end_line: None,
            language: Some("diff".to_string()),
            pinned: false,
            token_count,
            metadata: HashMap::new(),
            added_at: chrono::Utc::now(),
        }
    }

    /// Create a web content context item
    pub fn web(url: impl Into<String>, content: impl Into<String>) -> Self {
        let url = url.into();
        let content = content.into();
        let token_count = estimate_tokens(&content);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::Web,
            name: url.clone(),
            path: None,
            content,
            start_line: None,
            end_line: None,
            language: None,
            pinned: false,
            token_count,
            metadata: {
                let mut m = HashMap::new();
                m.insert("url".to_string(), url);
                m
            },
            added_at: chrono::Utc::now(),
        }
    }

    /// Create an MCP resource context item
    pub fn mcp_resource(
        server: impl Into<String>,
        uri: impl Into<String>,
        name: impl Into<String>,
        content: impl Into<String>,
        mime_type: Option<String>,
    ) -> Self {
        let server = server.into();
        let uri = uri.into();
        let name = name.into();
        let content = content.into();
        let token_count = estimate_tokens(&content);

        // Detect language from mime type or URI
        let language = mime_type
            .as_ref()
            .and_then(|m| mime_to_language(m))
            .or_else(|| uri_to_language(&uri));

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::McpResource,
            name,
            path: None,
            content,
            start_line: None,
            end_line: None,
            language,
            pinned: false,
            token_count,
            metadata: {
                let mut m = HashMap::new();
                m.insert("server".to_string(), server);
                m.insert("uri".to_string(), uri);
                if let Some(mime) = mime_type {
                    m.insert("mime_type".to_string(), mime);
                }
                m
            },
            added_at: chrono::Utc::now(),
        }
    }

    /// Create an MCP prompt result context item
    pub fn mcp_prompt(
        server: impl Into<String>,
        prompt_name: impl Into<String>,
        result: impl Into<String>,
    ) -> Self {
        let server = server.into();
        let prompt_name = prompt_name.into();
        let result = result.into();
        let token_count = estimate_tokens(&result);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_type: ContextItemType::McpPrompt,
            name: format!("Prompt: {}", prompt_name),
            path: None,
            content: result,
            start_line: None,
            end_line: None,
            language: None,
            pinned: false,
            token_count,
            metadata: {
                let mut m = HashMap::new();
                m.insert("server".to_string(), server);
                m.insert("prompt_name".to_string(), prompt_name);
                m
            },
            added_at: chrono::Utc::now(),
        }
    }

    /// Pin this item
    pub fn pin(mut self) -> Self {
        self.pinned = true;
        self
    }

    /// Set language hint
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Format as context string for AI prompt
    pub fn format_for_prompt(&self) -> String {
        let mut output = String::new();

        match &self.item_type {
            ContextItemType::File => {
                output.push_str(&format!("File: {}\n", self.name));
                if let Some(lang) = &self.language {
                    output.push_str(&format!("```{}\n{}\n```", lang, self.content));
                } else {
                    output.push_str(&format!("```\n{}\n```", self.content));
                }
            }
            ContextItemType::Snippet => {
                if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
                    output.push_str(&format!(
                        "Snippet from {} (lines {}-{}):\n",
                        self.name, start, end
                    ));
                } else {
                    output.push_str(&format!("Snippet from {}:\n", self.name));
                }
                if let Some(lang) = &self.language {
                    output.push_str(&format!("```{}\n{}\n```", lang, self.content));
                } else {
                    output.push_str(&format!("```\n{}\n```", self.content));
                }
            }
            ContextItemType::Diff => {
                output.push_str(&format!("Diff: {}\n", self.name));
                output.push_str(&format!("```diff\n{}\n```", self.content));
            }
            ContextItemType::Web => {
                let url = self
                    .metadata
                    .get("url")
                    .map(|s| s.as_str())
                    .unwrap_or(&self.name);
                output.push_str(&format!("Web content from {}:\n{}", url, self.content));
            }
            ContextItemType::McpResource => {
                let server = self
                    .metadata
                    .get("server")
                    .map(|s| s.as_str())
                    .unwrap_or("unknown");
                let uri = self
                    .metadata
                    .get("uri")
                    .map(|s| s.as_str())
                    .unwrap_or(&self.name);
                output.push_str(&format!("MCP Resource from {} (URI: {}):\n", server, uri));
                if let Some(lang) = &self.language {
                    output.push_str(&format!("```{}\n{}\n```", lang, self.content));
                } else {
                    output.push_str(&self.content);
                }
            }
            ContextItemType::McpPrompt => {
                let server = self
                    .metadata
                    .get("server")
                    .map(|s| s.as_str())
                    .unwrap_or("unknown");
                let prompt = self
                    .metadata
                    .get("prompt_name")
                    .map(|s| s.as_str())
                    .unwrap_or(&self.name);
                output.push_str(&format!(
                    "MCP Prompt \"{}\" from {}:\n{}",
                    prompt, server, self.content
                ));
            }
            _ => {
                output.push_str(&format!(
                    "{}: {}\n{}",
                    match &self.item_type {
                        ContextItemType::Directory => "Directory",
                        ContextItemType::SearchResults => "Search Results",
                        ContextItemType::Image => "Image",
                        ContextItemType::Custom(t) => t,
                        _ => "Content",
                    },
                    self.name,
                    self.content
                ));
            }
        }

        output
    }
}
