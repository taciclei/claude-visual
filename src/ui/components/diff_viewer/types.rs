//! Type definitions for diff viewer components

use gpui::*;

/// Diff view mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DiffViewMode {
    #[default]
    Unified,
    Split,
    Inline,
}

/// Line change type in diff
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DiffLineType {
    #[default]
    Context,
    Added,
    Removed,
    Modified,
    Header,
    Hunk,
}

impl DiffLineType {
    pub(super) fn background(&self) -> gpui::Hsla {
        match self {
            Self::Added => rgba(0x22c55e1a).into(),
            Self::Removed => rgba(0xef44441a).into(),
            Self::Modified => rgba(0xeab3081a).into(),
            Self::Header => rgba(0x3b82f61a).into(),
            Self::Hunk => rgba(0xa855f71a).into(),
            Self::Context => rgba(0x00000000).into(),
        }
    }

    pub(super) fn text_color(&self) -> gpui::Hsla {
        match self {
            Self::Added => rgb(0x22c55e).into(),
            Self::Removed => rgb(0xef4444).into(),
            Self::Modified => rgb(0xeab308).into(),
            Self::Header | Self::Hunk => rgba(0x888888ff).into(),
            Self::Context => rgba(0xccccccff).into(),
        }
    }

    pub(super) fn prefix(&self) -> &str {
        match self {
            Self::Added => "+",
            Self::Removed => "-",
            Self::Context => " ",
            Self::Modified => "~",
            Self::Header | Self::Hunk => "",
        }
    }
}

/// Single diff line
#[derive(Debug, Clone)]
pub struct DiffLine {
    pub content: SharedString,
    pub line_type: DiffLineType,
    pub old_line_number: Option<usize>,
    pub new_line_number: Option<usize>,
}

impl DiffLine {
    pub fn new(content: impl Into<SharedString>, line_type: DiffLineType) -> Self {
        Self {
            content: content.into(),
            line_type,
            old_line_number: None,
            new_line_number: None,
        }
    }

    pub fn context(content: impl Into<SharedString>, old: usize, new: usize) -> Self {
        Self {
            content: content.into(),
            line_type: DiffLineType::Context,
            old_line_number: Some(old),
            new_line_number: Some(new),
        }
    }

    pub fn added(content: impl Into<SharedString>, new: usize) -> Self {
        Self {
            content: content.into(),
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(new),
        }
    }

    pub fn removed(content: impl Into<SharedString>, old: usize) -> Self {
        Self {
            content: content.into(),
            line_type: DiffLineType::Removed,
            old_line_number: Some(old),
            new_line_number: None,
        }
    }

    pub fn header(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            line_type: DiffLineType::Header,
            old_line_number: None,
            new_line_number: None,
        }
    }

    pub fn hunk(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            line_type: DiffLineType::Hunk,
            old_line_number: None,
            new_line_number: None,
        }
    }
}

/// Type of file change
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FileChangeType {
    #[default]
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
}

impl FileChangeType {
    pub(super) fn color(&self) -> gpui::Hsla {
        match self {
            Self::Added => rgb(0x22c55e).into(),
            Self::Deleted => rgb(0xef4444).into(),
            Self::Modified => rgb(0xeab308).into(),
            Self::Renamed => rgb(0xa855f7).into(),
            Self::Copied => rgb(0x3b82f6).into(),
            Self::TypeChanged => rgb(0xf97316).into(),
        }
    }

    pub(super) fn label(&self) -> &str {
        match self {
            Self::Added => "A",
            Self::Deleted => "D",
            Self::Modified => "M",
            Self::Renamed => "R",
            Self::Copied => "C",
            Self::TypeChanged => "T",
        }
    }
}
