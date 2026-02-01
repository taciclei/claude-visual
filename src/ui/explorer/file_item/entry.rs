//! File entry struct and core methods

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::types::{FileType, GitStatus};

/// File entry in the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// File name
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// File type
    pub file_type: FileType,
    /// Git status
    pub git_status: Option<GitStatus>,
    /// File size in bytes (for files only)
    pub size: Option<u64>,
    /// Last modified timestamp
    pub modified: Option<u64>,
    /// Whether this is expanded (for directories)
    pub is_expanded: bool,
    /// Children (for directories)
    pub children: Vec<FileEntry>,
    /// Depth in tree
    pub depth: usize,
    /// Is selected
    pub is_selected: bool,
    /// Is hovered (for preview)
    pub is_hovered: bool,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.file_type().is_symlink() {
            FileType::Symlink
        } else {
            FileType::File
        };

        Ok(Self {
            name,
            path,
            file_type,
            git_status: None,
            size: if file_type == FileType::File {
                Some(metadata.len())
            } else {
                None
            },
            modified: metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs()),
            is_expanded: false,
            children: Vec::new(),
            depth: 0,
            is_selected: false,
            is_hovered: false,
        })
    }

    /// Create a directory entry with children
    pub fn directory(path: PathBuf, name: String, depth: usize) -> Self {
        Self {
            name,
            path,
            file_type: FileType::Directory,
            git_status: None,
            size: None,
            modified: None,
            is_expanded: false,
            children: Vec::new(),
            depth,
            is_selected: false,
            is_hovered: false,
        }
    }

    /// Create a file entry
    pub fn file(path: PathBuf, name: String, depth: usize) -> Self {
        let size = std::fs::metadata(&path).ok().map(|m| m.len());

        Self {
            name,
            path,
            file_type: FileType::File,
            git_status: None,
            size,
            modified: None,
            is_expanded: false,
            children: Vec::new(),
            depth,
            is_selected: false,
            is_hovered: false,
        }
    }

    /// Check if this is a directory
    pub fn is_directory(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Check if this is a file
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        self.path.extension().and_then(|e| e.to_str())
    }
}
