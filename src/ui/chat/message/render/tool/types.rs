//! Tool display types

/// Represents different types of tool content displays
#[derive(Debug, Clone)]
pub(super) enum ToolDisplay {
    FilePath {
        path: String,
        display: String,
    },
    Command {
        cmd: String,
        desc: String,
        display: String,
    },
    Pattern {
        pattern: String,
        path: Option<String>,
        display: String,
    },
    Prompt {
        display: String,
    },
    Edit {
        file_path: String,
        old_text: Option<String>,
        new_text: Option<String>,
    },
    Json(String),
    Plain(String),
}

impl ToolDisplay {
    /// Extract file path if available
    pub(super) fn file_path(&self) -> Option<String> {
        match self {
            ToolDisplay::FilePath { path, .. } => Some(path.clone()),
            ToolDisplay::Edit { file_path, .. } => Some(file_path.clone()),
            _ => None,
        }
    }

    /// Extract command if available
    pub(super) fn command(&self) -> Option<String> {
        match self {
            ToolDisplay::Command { cmd, .. } => Some(cmd.clone()),
            _ => None,
        }
    }

    /// Extract pattern if available
    pub(super) fn pattern(&self) -> Option<String> {
        match self {
            ToolDisplay::Pattern { pattern, .. } => Some(pattern.clone()),
            _ => None,
        }
    }
}
