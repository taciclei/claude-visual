//! Popover component for floating content

mod types;
mod popover;
mod content;
mod menu;

pub use types::*;
pub use popover::Popover;
pub use content::PopoverContent;
pub use menu::{MenuPopover, MenuPopoverItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_content() {
        let content = PopoverContent::new("Hello")
            .with_title("Title")
            .width(300.0);

        assert_eq!(content.title, Some("Title".to_string()));
        assert_eq!(content.content, "Hello");
        assert_eq!(content.width, 300.0);
    }

    #[test]
    fn test_menu_popover_item() {
        let item = MenuPopoverItem::new("Copy")
            .icon("📋")
            .shortcut("⌘C")
            .disabled();

        assert_eq!(item.label, "Copy");
        assert_eq!(item.icon, Some("📋".to_string()));
        assert_eq!(item.shortcut, Some("⌘C".to_string()));
        assert!(item.disabled);
    }
}
