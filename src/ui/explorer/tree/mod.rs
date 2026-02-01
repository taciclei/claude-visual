//! File Tree Component
//!
//! GPUI component for browsing files and directories.

mod core;
mod keyboard;
mod render;
mod types;

// Re-export public types
pub use core::FileTree;
pub use types::{DraggedFile, DraggedFiles, FileTreeEvent};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_tree_event() {
        let path = PathBuf::from("/test/file.rs");
        let event = FileTreeEvent::FileSelected(path.clone());
        assert!(matches!(event, FileTreeEvent::FileSelected(_)));
    }
}
