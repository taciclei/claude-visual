//! File type and Git status enumerations

use serde::{Deserialize, Serialize};

/// File type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    /// Directory
    Directory,
    /// Regular file
    File,
    /// Symbolic link
    Symlink,
    /// Git-specific types
    GitIgnored,
    GitSubmodule,
}

/// Git status for a file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitStatus {
    /// Clean/unmodified
    Clean,
    /// Modified (M)
    Modified,
    /// Added/staged (A)
    Added,
    /// Deleted (D)
    Deleted,
    /// Renamed (R)
    Renamed,
    /// Untracked (?)
    Untracked,
    /// Ignored
    Ignored,
    /// Conflicted (U)
    Conflicted,
}

impl GitStatus {
    /// Get status character
    pub fn char(&self) -> char {
        match self {
            GitStatus::Clean => ' ',
            GitStatus::Modified => 'M',
            GitStatus::Added => 'A',
            GitStatus::Deleted => 'D',
            GitStatus::Renamed => 'R',
            GitStatus::Untracked => '?',
            GitStatus::Ignored => '!',
            GitStatus::Conflicted => 'U',
        }
    }

    /// Get status color (as RGB)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            GitStatus::Clean => (128, 128, 128),      // Gray
            GitStatus::Modified => (209, 154, 102),   // Yellow/Orange
            GitStatus::Added => (98, 181, 67),        // Green
            GitStatus::Deleted => (224, 108, 117),    // Red
            GitStatus::Renamed => (97, 175, 239),     // Blue
            GitStatus::Untracked => (152, 195, 121),  // Light green
            GitStatus::Ignored => (92, 99, 112),      // Dim gray
            GitStatus::Conflicted => (198, 120, 221), // Purple
        }
    }
}
