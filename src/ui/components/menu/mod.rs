//! Menu and Popover components

mod action_menu;
mod context_menu;
mod menu;
mod types;

pub use action_menu::ActionMenu;
pub use context_menu::ContextMenu;
pub use menu::Menu;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_builder() {
        let item = MenuItemData::new("copy", "Copy")
            .with_icon("📋")
            .with_shortcut("⌘C")
            .destructive();

        assert_eq!(item.id, "copy");
        assert_eq!(item.label, "Copy");
        assert_eq!(item.icon, Some("📋".to_string()));
        assert_eq!(item.shortcut, Some("⌘C".to_string()));
        assert!(item.destructive);
    }

    #[test]
    fn test_menu_separator() {
        let sep = MenuItemData::separator();
        assert!(matches!(sep.item_type, MenuItemType::Separator));
    }

    #[test]
    fn test_menu_header() {
        let header = MenuItemData::header("Actions");
        assert!(matches!(header.item_type, MenuItemType::Header));
        assert_eq!(header.label, "Actions");
    }
}
