//! Type definitions for code block LSP integration

use std::path::PathBuf;
use crate::lsp::protocol::{Position, Location, Range};

/// Events emitted by code block LSP features
#[derive(Debug, Clone)]
pub enum CodeLspEvent {
    /// Go to definition at location
    GoToDefinition(Location),
    /// Go to type definition
    GoToTypeDefinition(Location),
    /// Go to implementation
    GoToImplementation(Location),
    /// Find references
    FindReferences(Vec<Location>),
    /// Hover information available
    HoverAvailable { position: Position, content: String },
    /// Symbol highlighted
    SymbolHighlighted { symbol: String, range: Range },
    /// Error occurred
    Error(String),
}

/// LSP token at a specific position
#[derive(Debug, Clone)]
pub struct CodeToken {
    /// Start byte offset
    pub(crate) start: usize,
    /// End byte offset
    pub(crate) end: usize,
    /// Token text
    pub(crate) text: String,
    /// Line number (0-indexed)
    pub(crate) line: usize,
    /// Column (0-indexed)
    pub(crate) column: usize,
    /// Token type (keyword, identifier, etc.)
    pub(crate) token_type: TokenType,
}

/// Token types for LSP interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Regular identifier (variable, function name)
    Identifier,
    /// Type name
    Type,
    /// Keyword
    Keyword,
    /// String literal
    String,
    /// Number literal
    Number,
    /// Comment
    Comment,
    /// Operator
    Operator,
    /// Punctuation
    Punctuation,
    /// Unknown
    Unknown,
}

impl TokenType {
    /// Check if token type supports go-to-definition
    pub fn supports_goto(&self) -> bool {
        matches!(self, TokenType::Identifier | TokenType::Type)
    }

    /// Check if token type supports hover
    pub fn supports_hover(&self) -> bool {
        matches!(self, TokenType::Identifier | TokenType::Type | TokenType::Keyword)
    }
}

/// LSP-enabled code block configuration
#[derive(Debug, Clone)]
pub struct CodeLspConfig {
    /// Enable go-to-definition on Ctrl+Click
    pub(crate) enable_goto_definition: bool,
    /// Enable hover on mouse over
    pub(crate) enable_hover: bool,
    /// Enable symbol highlighting on selection
    pub(crate) enable_symbol_highlight: bool,
    /// Hover delay in milliseconds
    pub(crate) hover_delay_ms: u64,
    /// File path for LSP context (for virtual files in chat)
    pub(crate) virtual_file_path: Option<PathBuf>,
    /// File extension hint for language detection
    pub(crate) language_hint: Option<String>,
}

impl Default for CodeLspConfig {
    fn default() -> Self {
        Self {
            enable_goto_definition: true,
            enable_hover: true,
            enable_symbol_highlight: true,
            hover_delay_ms: 300,
            virtual_file_path: None,
            language_hint: None,
        }
    }
}

impl CodeLspConfig {
    /// Create config with goto only
    pub fn goto_only() -> Self {
        Self {
            enable_goto_definition: true,
            enable_hover: false,
            enable_symbol_highlight: false,
            ..Default::default()
        }
    }

    /// Create config with all features
    pub fn full() -> Self {
        Self::default()
    }

    /// Create minimal config
    pub fn minimal() -> Self {
        Self {
            enable_goto_definition: false,
            enable_hover: false,
            enable_symbol_highlight: false,
            ..Default::default()
        }
    }
}

/// Clickable token element for go-to-definition
pub struct ClickableToken {
    /// Token index
    pub index: usize,
    /// Token text
    pub text: String,
    /// Whether token supports goto
    pub supports_goto: bool,
    /// Token range
    pub range: Range,
}

impl ClickableToken {
    /// Create from CodeToken
    pub fn from_token(token: &CodeToken, index: usize) -> Self {
        Self {
            index,
            text: token.text.clone(),
            supports_goto: token.token_type.supports_goto(),
            range: Range {
                start: Position {
                    line: token.line as u32,
                    character: token.column as u32,
                },
                end: Position {
                    line: token.line as u32,
                    character: (token.column + token.text.len()) as u32,
                },
            },
        }
    }
}
