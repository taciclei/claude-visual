//! Context Management
//!
//! Manages context items (files, snippets, etc.) that can be attached to AI conversations.

mod types;
mod item;
mod manager;
mod utils;

// Re-export public types
pub use types::{ContextError, ContextItem, ContextItemType};
pub use manager::ContextManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_context_item() {
        let item = ContextItem::file("/src/main.rs", "fn main() {}");
        assert_eq!(item.item_type, ContextItemType::File);
        assert_eq!(item.name, "main.rs");
        assert_eq!(item.language, Some("rs".to_string()));
    }

    #[test]
    fn test_snippet_context_item() {
        let item = ContextItem::snippet("/src/lib.rs", "pub fn hello() {}", 10, 15);
        assert_eq!(item.item_type, ContextItemType::Snippet);
        assert_eq!(item.start_line, Some(10));
        assert_eq!(item.end_line, Some(15));
    }
}
