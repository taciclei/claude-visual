//! Context menu (right-click menu) component

pub mod edit_menu;
pub mod file_menu;
pub mod menu;
pub mod tab_menu;
pub mod types;

// Re-exports
pub use edit_menu::EditContextMenu;
pub use file_menu::FileContextMenu;
pub use menu::ContextMenu;
pub use tab_menu::TabContextMenu;
pub use types::{ContextMenuEvent, ContextMenuItem, ContextMenuItemType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_item() {
        let item = ContextMenuItem::action("test", "Test Action")
            .icon("🔧")
            .shortcut("⌘T")
            .danger();

        assert_eq!(item.id, "test");
        assert_eq!(item.label, "Test Action");
        assert!(item.icon.is_some());
        assert!(item.shortcut.is_some());
        assert!(item.danger);
    }

    #[test]
    fn test_checkbox_item() {
        let item = ContextMenuItem::checkbox("show_hidden", "Show Hidden Files", true);

        assert!(matches!(
            item.item_type,
            ContextMenuItemType::Checkbox(true)
        ));
    }

    #[test]
    fn test_radio_item() {
        let item = ContextMenuItem::radio("small", "Small", "size", true);

        assert!(matches!(
            item.item_type,
            ContextMenuItemType::Radio { selected: true, .. }
        ));
    }

    #[test]
    fn test_submenu() {
        let children = vec![
            ContextMenuItem::action("a", "Option A"),
            ContextMenuItem::action("b", "Option B"),
        ];
        let item = ContextMenuItem::submenu("options", "More Options", children);

        assert!(matches!(item.item_type, ContextMenuItemType::Submenu));
        assert_eq!(item.children.len(), 2);
    }
}
