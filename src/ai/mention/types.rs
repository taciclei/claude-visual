//! Mention types

use std::path::PathBuf;

/// A mention in the input text
#[derive(Debug, Clone, PartialEq)]
pub struct Mention {
    /// Type of mention
    pub kind: MentionKind,
    /// Start position in text (byte offset)
    pub start: usize,
    /// End position in text (byte offset)
    pub end: usize,
    /// The raw text of the mention
    pub raw: String,
}

/// Kind of mention
#[derive(Debug, Clone, PartialEq)]
pub enum MentionKind {
    /// File reference: @file:path/to/file.rs or @path/to/file.rs
    File(PathBuf),
    /// Snippet reference: @snippet:name
    Snippet(String),
    /// URL reference: @url:https://example.com
    Url(String),
    /// Symbol reference: @symbol:function_name
    Symbol(String),
    /// Line range reference: @file:path.rs:10-20
    FileRange {
        path: PathBuf,
        start_line: usize,
        end_line: Option<usize>,
    },
}

/// A partial mention being typed
#[derive(Debug, Clone)]
pub struct PartialMention {
    /// Kind of mention (if determinable)
    pub kind: PartialMentionKind,
    /// Start position in text
    pub start: usize,
    /// The prefix typed so far (after @)
    pub prefix: String,
}

/// Kind of partial mention
#[derive(Debug, Clone)]
pub enum PartialMentionKind {
    File,
    Snippet,
    Url,
    Symbol,
    Unknown,
}
