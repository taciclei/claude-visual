//! Types and enums for code block component

use gpui::*;

/// Events emitted by CodeBlockView
pub enum CodeBlockEvent {
    /// Request to execute the code (for shell/bash)
    Execute(String),
    /// Request to save the code to a file
    SaveToFile(String),
    /// Request to explain the code
    ExplainCode(String, Option<String>),
    /// Request to improve/refactor the code
    ImproveCode(String, Option<String>),
    /// Request to add tests for the code
    AddTests(String, Option<String>),
}

/// Search match location
#[derive(Debug, Clone, PartialEq)]
pub struct SearchMatch {
    /// Line index (0-based)
    pub line: usize,
    /// Start column in line
    pub start: usize,
    /// End column in line
    pub end: usize,
}

/// Code block display mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CodeDisplayMode {
    /// Normal code display
    #[default]
    Normal,
    /// Diff view showing additions and removals
    Diff,
}

/// Line change type for diff display
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineChangeType {
    /// Unchanged line
    Context,
    /// Added line
    Added,
    /// Removed line
    Removed,
    /// Modified line (old version)
    ModifiedOld,
    /// Modified line (new version)
    ModifiedNew,
}

impl LineChangeType {
    /// Get the line prefix character
    pub fn prefix(&self) -> &'static str {
        match self {
            LineChangeType::Context => " ",
            LineChangeType::Added => "+",
            LineChangeType::Removed => "-",
            LineChangeType::ModifiedOld => "-",
            LineChangeType::ModifiedNew => "+",
        }
    }
}

/// A line in diff mode
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// The content of the line
    pub content: String,
    /// The type of change
    pub change_type: LineChangeType,
    /// Old line number (for context and removed)
    pub old_line_num: Option<usize>,
    /// New line number (for context and added)
    pub new_line_num: Option<usize>,
}

/// A range of highlighted lines
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightedRange {
    /// Start line (1-based, inclusive)
    pub start_line: usize,
    /// End line (1-based, inclusive)
    pub end_line: usize,
    /// Highlight style
    pub style: HighlightStyle,
    /// Optional label for the highlight
    pub label: Option<String>,
}

impl HighlightedRange {
    /// Create a new highlight for a single line
    pub fn single(line: usize, style: HighlightStyle) -> Self {
        Self {
            start_line: line,
            end_line: line,
            style,
            label: None,
        }
    }

    /// Create a new highlight for a range of lines
    pub fn range(start: usize, end: usize, style: HighlightStyle) -> Self {
        Self {
            start_line: start,
            end_line: end,
            style,
            label: None,
        }
    }

    /// Add a label to the highlight
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Check if a line (1-based) is within this range
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start_line && line <= self.end_line
    }
}

/// Style for line highlighting
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HighlightStyle {
    /// Referenced line (accent color)
    #[default]
    Reference,
    /// Error location (red)
    Error,
    /// Warning location (yellow/orange)
    Warning,
    /// Success/added (green)
    Success,
    /// Info/note (blue)
    Info,
    /// Custom emphasis (purple)
    Emphasis,
}
