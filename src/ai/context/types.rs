//! Type definitions for context management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Context item type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextItemType {
    /// Full file content
    File,
    /// Code snippet with line range
    Snippet,
    /// Directory structure
    Directory,
    /// Git diff
    Diff,
    /// Search results
    SearchResults,
    /// URL/Web content
    Web,
    /// Image
    Image,
    /// MCP Resource
    McpResource,
    /// MCP Prompt result
    McpPrompt,
    /// Custom/Other
    Custom(String),
}

/// A single context item that can be attached to a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    /// Unique identifier
    pub id: String,
    /// Type of context item
    pub item_type: ContextItemType,
    /// Display name
    pub name: String,
    /// File path (if applicable)
    pub path: Option<PathBuf>,
    /// Content of the item
    pub content: String,
    /// Start line (for snippets)
    pub start_line: Option<usize>,
    /// End line (for snippets)
    pub end_line: Option<usize>,
    /// Language hint (for syntax highlighting)
    pub language: Option<String>,
    /// Whether this item is pinned (persists across messages)
    pub pinned: bool,
    /// Token count estimate
    pub(crate) token_count: usize,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// When the item was added
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Context errors
#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("Token limit exceeded: current {current} + item {item} > max {max}")]
    TokenLimitExceeded {
        current: usize,
        item: usize,
        max: usize,
    },
    #[error("Item not found: {0}")]
    ItemNotFound(String),
    #[error("Failed to read file: {0}")]
    FileReadError(String),
}
