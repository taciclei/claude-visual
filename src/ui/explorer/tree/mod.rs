//! File Tree Component
//!
//! GPUI component for browsing files and directories.

mod types;
mod core;
mod keyboard;
mod render;

// Re-export public types
pub use types::{FileTreeEvent, DraggedFile, DraggedFiles};
pub use core::FileTree;

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
