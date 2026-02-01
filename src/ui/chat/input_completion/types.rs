//! Type definitions for chat input completion

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::lsp::protocol::{CompletionItem, CompletionItemKind};
use super::utils::fuzzy_match;

/// Completion trigger types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionTrigger {
    /// Triggered by typing a character
    Character(char),
    /// Triggered manually (Ctrl+Space)
    Manual,
    /// Triggered by @ for mentions
    Mention,
    /// Triggered by / for commands
    Command,
    /// Triggered by file path
    FilePath,
    /// Triggered for code inside backticks
    Code,
}

impl CompletionTrigger {
    /// Check if character is a trigger
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '@' => Some(Self::Mention),
            '/' => Some(Self::Command),
            '.' | '/' | '\\' => Some(Self::FilePath),
            _ => None,
        }
    }
}

/// Completion item for chat input
#[derive(Debug, Clone)]
pub struct ChatCompletionItem {
    /// Display label
    pub(crate) label: String,
    /// Text to insert
    pub(crate) insert_text: String,
    /// Item kind
    pub(crate) kind: ChatCompletionKind,
    /// Detail/description
    pub(crate) detail: Option<String>,
    /// Documentation
    pub(crate) documentation: Option<String>,
    /// Filter text (for matching)
    pub(crate) filter_text: Option<String>,
    /// Sort text (for ordering)
    pub(crate) sort_text: Option<String>,
    /// Preview (for file mentions)
    pub(crate) preview: Option<String>,
    /// Icon
    pub(crate) icon: Option<String>,
}

impl ChatCompletionItem {
    /// Create a file completion
    pub fn file(path: PathBuf) -> Self {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let is_dir = path.is_dir();

        Self {
            label: name.clone(),
            insert_text: format!("@{}", path.display()),
            kind: if is_dir {
                ChatCompletionKind::Folder
            } else {
                ChatCompletionKind::File
            },
            detail: Some(path.parent()
                .map(|p| p.display().to_string())
                .unwrap_or_default()),
            documentation: None,
            filter_text: Some(name),
            sort_text: None,
            preview: None,
            icon: if is_dir { Some("folder".to_string()) } else { Some("file".to_string()) },
        }
    }

    /// Create a command completion
    pub fn command(name: &str, description: &str) -> Self {
        Self {
            label: format!("/{}", name),
            insert_text: format!("/{}", name),
            kind: ChatCompletionKind::Command,
            detail: Some(description.to_string()),
            documentation: None,
            filter_text: Some(name.to_string()),
            sort_text: None,
            preview: None,
            icon: Some("terminal".to_string()),
        }
    }

    /// Create a mention completion
    pub fn mention(mention_type: &str, value: &str, description: Option<&str>) -> Self {
        Self {
            label: format!("@{}:{}", mention_type, value),
            insert_text: format!("@{}:{}", mention_type, value),
            kind: ChatCompletionKind::Mention,
            detail: description.map(|s| s.to_string()),
            documentation: None,
            filter_text: Some(value.to_string()),
            sort_text: None,
            preview: None,
            icon: Some("at".to_string()),
        }
    }

    /// Create from LSP completion item
    pub fn from_lsp(item: &CompletionItem) -> Self {
        Self {
            label: item.label.clone(),
            insert_text: item.insert_text.clone().unwrap_or_else(|| item.label.clone()),
            kind: ChatCompletionKind::from_lsp(item.kind),
            detail: item.detail.clone(),
            documentation: item.documentation.as_ref().map(|d| match d {
                crate::lsp::protocol::Documentation::String(s) => s.clone(),
                crate::lsp::protocol::Documentation::MarkupContent(mc) => mc.value.clone(),
            }),
            filter_text: item.filter_text.clone(),
            sort_text: item.sort_text.clone(),
            preview: None,
            icon: None,
        }
    }

    /// Get match score against query
    pub fn match_score(&self, query: &str) -> Option<i32> {
        let filter = self.filter_text.as_ref().unwrap_or(&self.label);
        fuzzy_match(filter, query)
    }
}

/// Completion item kinds for chat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatCompletionKind {
    /// File path
    File,
    /// Folder
    Folder,
    /// Slash command
    Command,
    /// @ mention
    Mention,
    /// Code symbol
    Symbol,
    /// Keyword
    Keyword,
    /// Variable
    Variable,
    /// Function
    Function,
    /// Type
    Type,
    /// Snippet
    Snippet,
    /// Other
    Other,
}

impl ChatCompletionKind {
    /// Convert from LSP completion kind
    pub fn from_lsp(kind: Option<CompletionItemKind>) -> Self {
        match kind {
            Some(CompletionItemKind::File) => Self::File,
            Some(CompletionItemKind::Folder) => Self::Folder,
            Some(CompletionItemKind::Variable) => Self::Variable,
            Some(CompletionItemKind::Function) | Some(CompletionItemKind::Method) => Self::Function,
            Some(CompletionItemKind::Class) | Some(CompletionItemKind::Interface) | Some(CompletionItemKind::Struct) => Self::Type,
            Some(CompletionItemKind::Keyword) => Self::Keyword,
            Some(CompletionItemKind::Snippet) => Self::Snippet,
            _ => Self::Other,
        }
    }

    /// Get icon name
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Folder => "folder",
            Self::Command => "terminal",
            Self::Mention => "at-sign",
            Self::Symbol => "symbol",
            Self::Keyword => "key",
            Self::Variable => "variable",
            Self::Function => "function",
            Self::Type => "type",
            Self::Snippet => "snippet",
            Self::Other => "circle",
        }
    }

    /// Get sort priority (lower = higher priority)
    pub fn sort_priority(&self) -> u8 {
        match self {
            Self::Command => 0,
            Self::Mention => 1,
            Self::File => 2,
            Self::Folder => 3,
            Self::Function => 4,
            Self::Variable => 5,
            Self::Type => 6,
            Self::Keyword => 7,
            Self::Snippet => 8,
            Self::Symbol => 9,
            Self::Other => 10,
        }
    }
}

/// Chat completion configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionConfig {
    /// Enable completion
    pub enabled: bool,
    /// Trigger characters for automatic completion
    pub trigger_characters: Vec<char>,
    /// Minimum characters before triggering
    pub min_chars: usize,
    /// Maximum items to show
    pub max_items: usize,
    /// Debounce delay in ms
    pub debounce_ms: u64,
    /// Show file completions
    pub show_files: bool,
    /// Show command completions
    pub show_commands: bool,
    /// Show code completions (LSP)
    pub show_code_completions: bool,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            trigger_characters: vec!['@', '/', '.'],
            min_chars: 1,
            max_items: 10,
            debounce_ms: 100,
            show_files: true,
            show_commands: true,
            show_code_completions: true,
        }
    }
}

/// Completion state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionState {
    /// No completion active
    Inactive,
    /// Waiting for trigger
    Waiting,
    /// Loading completions
    Loading,
    /// Showing completions
    Active,
}

/// Result of accepting a completion
#[derive(Debug, Clone)]
pub struct CompletionResult {
    /// Text to insert
    pub insert_text: String,
    /// Range to replace (start, end)
    pub replace_range: (usize, usize),
    /// Completion kind
    pub kind: ChatCompletionKind,
}
