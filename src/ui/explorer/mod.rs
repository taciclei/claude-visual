//! File Explorer Module
//!
//! File tree view with git integration and context attachment.

mod file_item;
mod preview;
mod tree;

pub use file_item::{FileEntry, FileType, GitStatus};
pub use preview::{FilePreviewEvent, FilePreviewPanel, PreviewState};
pub use tree::{DraggedFile, DraggedFiles, FileTree, FileTreeEvent};
