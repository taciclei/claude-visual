//! Diff data types

/// Line in a diff with line numbers
#[derive(Debug, Clone)]
pub enum DiffLine {
    /// Context line (unchanged) with old and new line numbers
    Context { content: String, old_line: usize, new_line: usize },
    /// Added line with new line number
    Added { content: String, new_line: usize },
    /// Removed line with old line number
    Removed { content: String, old_line: usize },
    /// Hunk header (@@...)
    HunkHeader { header: String, old_start: usize, old_count: usize, new_start: usize, new_count: usize },
}

/// A hunk in a diff (group of changes)
#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub header: String,
    pub lines: Vec<DiffLine>,
    pub collapsed: bool,
}

/// Events emitted by DiffBlockView
pub enum DiffBlockEvent {
    /// File was clicked
    FileClicked(String),
}
