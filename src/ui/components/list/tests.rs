//! Tests for list components

use super::*;

#[test]
fn test_list_item() {
    let item = ListItem::new("Primary text")
        .secondary("Secondary text")
        .leading("🔧")
        .trailing("→")
        .selected(true);

    assert_eq!(item.primary, "Primary text");
    assert!(item.secondary.is_some());
    assert!(item.selected);
}

#[test]
fn test_list() {
    let list = List::new()
        .item(ListItem::new("Item 1"))
        .item(ListItem::new("Item 2"))
        .size(ListSize::Compact)
        .style(ListStyle::Card)
        .selected(0);

    assert_eq!(list.items.len(), 2);
    assert_eq!(list.selected_index, Some(0));
}

#[test]
fn test_action_item() {
    let action = ActionItem::new("Delete").icon("🗑️").shortcut("⌘⌫").danger();

    assert_eq!(action.label, "Delete");
    assert!(action.danger);
}

#[test]
fn test_description_list() {
    let dl = DescriptionList::new()
        .item("Name", "John Doe")
        .item("Email", "john@example.com")
        .layout(DescriptionLayout::Grid);

    assert_eq!(dl.items.len(), 2);
}

#[test]
fn test_bullet_list() {
    let list = BulletList::new()
        .item("First item")
        .item("Second item")
        .bullet("→");

    assert_eq!(list.items.len(), 2);
    assert_eq!(list.bullet, "→");
}
