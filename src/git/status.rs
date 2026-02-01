//! Git file status

use git2::Status;
use serde::{Deserialize, Serialize};

/// Status of a file in git
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatusKind {
    /// File is new/untracked
    New,
    /// File is modified
    Modified,
    /// File is deleted
    Deleted,
    /// File is renamed
    Renamed,
    /// File is copied
    Copied,
    /// File has type change
    TypeChange,
    /// File is unreadable
    Unreadable,
    /// File is ignored
    Ignored,
    /// File is in conflict
    Conflict,
}

/// File status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    /// File path relative to repo root
    pub path: String,
    /// Index (staged) status
    pub index_status: Option<FileStatusKind>,
    /// Working directory status
    pub workdir_status: Option<FileStatusKind>,
}

impl FileStatus {
    /// Create from git2 status
    pub fn from_git_status(path: &str, status: Status) -> Self {
        let index_status = Self::status_kind_from_index(status);
        let workdir_status = Self::status_kind_from_workdir(status);

        Self {
            path: path.to_string(),
            index_status,
            workdir_status,
        }
    }

    fn status_kind_from_index(status: Status) -> Option<FileStatusKind> {
        if status.is_index_new() {
            Some(FileStatusKind::New)
        } else if status.is_index_modified() {
            Some(FileStatusKind::Modified)
        } else if status.is_index_deleted() {
            Some(FileStatusKind::Deleted)
        } else if status.is_index_renamed() {
            Some(FileStatusKind::Renamed)
        } else if status.is_index_typechange() {
            Some(FileStatusKind::TypeChange)
        } else {
            None
        }
    }

    fn status_kind_from_workdir(status: Status) -> Option<FileStatusKind> {
        if status.is_wt_new() {
            Some(FileStatusKind::New)
        } else if status.is_wt_modified() {
            Some(FileStatusKind::Modified)
        } else if status.is_wt_deleted() {
            Some(FileStatusKind::Deleted)
        } else if status.is_wt_renamed() {
            Some(FileStatusKind::Renamed)
        } else if status.is_wt_typechange() {
            Some(FileStatusKind::TypeChange)
        } else if status.is_conflicted() {
            Some(FileStatusKind::Conflict)
        } else if status.is_ignored() {
            Some(FileStatusKind::Ignored)
        } else {
            None
        }
    }

    /// Check if file has any changes
    pub fn is_changed(&self) -> bool {
        self.index_status.is_some() || self.workdir_status.is_some()
    }

    /// Check if file is staged
    pub fn is_staged(&self) -> bool {
        self.index_status.is_some()
    }

    /// Check if file has unstaged changes
    pub fn is_unstaged(&self) -> bool {
        self.workdir_status.is_some()
    }

    /// Check if file is in conflict
    pub fn is_conflicted(&self) -> bool {
        self.workdir_status == Some(FileStatusKind::Conflict)
    }

    /// Get a single character representation for display
    pub fn status_char(&self) -> char {
        if let Some(status) = &self.index_status {
            match status {
                FileStatusKind::New => 'A',
                FileStatusKind::Modified => 'M',
                FileStatusKind::Deleted => 'D',
                FileStatusKind::Renamed => 'R',
                FileStatusKind::Copied => 'C',
                FileStatusKind::TypeChange => 'T',
                _ => ' ',
            }
        } else if let Some(status) = &self.workdir_status {
            match status {
                FileStatusKind::New => '?',
                FileStatusKind::Modified => 'm',
                FileStatusKind::Deleted => 'd',
                FileStatusKind::Conflict => 'U',
                FileStatusKind::Ignored => '!',
                _ => ' ',
            }
        } else {
            ' '
        }
    }
}
