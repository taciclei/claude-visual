//! Core LSP types for text document positions and ranges

use serde::{Deserialize, Serialize};

/// Position in a text document (0-indexed)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: u32,
    /// Character offset (0-indexed, UTF-16 code units)
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// A range in a text document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn point(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
}

/// A location in a document
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// Document URI
    pub uri: String,
    /// Range in the document
    pub range: Range,
}

/// Text document identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    /// Document URI
    pub uri: String,
}

/// Versioned text document identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedTextDocumentIdentifier {
    /// Document URI
    pub uri: String,
    /// Document version
    pub version: i32,
}

/// Text document position params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
    /// The text document
    pub text_document: TextDocumentIdentifier,
    /// The position inside the text document
    pub position: Position,
}

/// Text edit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    /// Range to replace
    pub range: Range,
    /// New text
    pub new_text: String,
}
