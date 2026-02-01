//! Sortable list components - Drag and drop reorderable lists
//!
//! Provides components for creating sortable, draggable list items.

mod kanban_column;
mod nested_list;
mod sortable_grid;
mod sortable_list;
mod types;

pub use kanban_column::KanbanColumn;
pub use nested_list::NestedSortableList;
pub use sortable_grid::SortableGrid;
pub use sortable_list::SortableList;
pub use types::{DragState, NestedSortableItem, SortableItem, SortableVariant};

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::*;

    #[test]
    fn test_sortable_item() {
        let item = SortableItem::new("1", "Task 1")
            .icon("📝")
            .disabled(false)
            .locked(true);

        assert!(item.locked);
        assert!(!item.disabled);
        assert!(item.icon.is_some());
    }

    #[test]
    fn test_sortable_list_variants() {
        let default = SortableList::new("sl").variant(SortableVariant::Default);
        let cards = SortableList::new("sl").variant(SortableVariant::Cards);

        assert_eq!(default.variant, SortableVariant::Default);
        assert_eq!(cards.variant, SortableVariant::Cards);
    }

    #[test]
    fn test_sortable_list() {
        let items = vec![
            SortableItem::new("1", "Item 1"),
            SortableItem::new("2", "Item 2"),
            SortableItem::new("3", "Item 3"),
        ];

        let list = SortableList::new("sl")
            .items(items)
            .show_handle(true)
            .show_numbers(true);

        assert_eq!(list.items.len(), 3);
        assert!(list.show_handle);
        assert!(list.show_numbers);
    }

    #[test]
    fn test_kanban_column() {
        let items = vec![
            SortableItem::new("1", "Task 1"),
            SortableItem::new("2", "Task 2"),
        ];

        let column = KanbanColumn::new("col", "To Do")
            .items(items)
            .max_items(5)
            .color(hsla(0.6, 0.7, 0.5, 1.0));

        assert_eq!(column.items.len(), 2);
        assert_eq!(column.max_items, Some(5));
    }

    #[test]
    fn test_sortable_grid() {
        let items = vec![
            SortableItem::new("1", "App 1").icon("📱"),
            SortableItem::new("2", "App 2").icon("💻"),
        ];

        let grid = SortableGrid::new("sg")
            .items(items)
            .columns(4)
            .item_size(80.0);

        assert_eq!(grid.items.len(), 2);
        assert_eq!(grid.columns, 4);
    }

    #[test]
    fn test_nested_sortable() {
        let items = vec![NestedSortableItem::new("1", "Parent")
            .children(vec![
                NestedSortableItem::new("1.1", "Child 1"),
                NestedSortableItem::new("1.2", "Child 2"),
            ])
            .collapsed(false)];

        let list = NestedSortableList::new("nsl")
            .items(items)
            .indent_size(24.0);

        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].children.len(), 2);
    }
}
