//! Tree component for hierarchical data display

mod directory_listing;
mod file_tree;
mod tree_actions;
mod tree_render;
mod tree_state;
mod types;

// Re-export public types
pub use types::{DirectoryEntry, FileTreeItem, TreeEvent, TreeNode, TreeStyle};

// Re-export components
pub use directory_listing::DirectoryListing;
pub use file_tree::FileTree;
pub use tree_state::Tree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let node = TreeNode::new("1", "Root")
            .icon("📁")
            .child(TreeNode::new("1.1", "Child"))
            .data("some data");

        assert_eq!(node.id, "1");
        assert_eq!(node.label, "Root");
        assert_eq!(node.icon, Some("📁".to_string()));
        assert_eq!(node.children.len(), 1);
        assert!(node.has_children());
    }

    #[test]
    fn test_file_tree_item() {
        let file = FileTreeItem::file("test.rs", 2);
        assert!(!file.is_dir);
        assert_eq!(file.depth, 2);

        let dir = FileTreeItem::dir("src", 1, true);
        assert!(dir.is_dir);
        assert!(dir.expanded);
    }

    #[test]
    fn test_directory_entry() {
        let entry = DirectoryEntry::file("readme.md")
            .size("1.2 KB")
            .modified("Jan 26");

        assert_eq!(entry.name, "readme.md");
        assert!(!entry.is_dir);
        assert_eq!(entry.size, Some("1.2 KB".to_string()));
    }
}
