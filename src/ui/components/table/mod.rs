//! Table component for displaying tabular data

mod types;
mod table_impl;
mod render;
mod data_table;
mod key_value_table;

pub use types::*;
pub use table_impl::Table;
pub use data_table::DataTable;
pub use key_value_table::KeyValueTable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_column() {
        let col = TableColumn::new("name", "Name")
            .width(200.0)
            .align(ColumnAlign::Center)
            .sortable();

        assert_eq!(col.key, "name");
        assert_eq!(col.header, "Name");
        assert_eq!(col.width, Some(200.0));
        assert_eq!(col.align, ColumnAlign::Center);
        assert!(col.sortable);
    }

    #[test]
    fn test_table_row() {
        let row = TableRow::new(vec!["Alice", "30", "Engineer"])
            .disabled();

        assert_eq!(row.cells.len(), 3);
        assert!(row.disabled);
    }

    #[test]
    fn test_data_table() {
        let table = DataTable::new(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .row(vec!["Bob", "25"])
            .striped();

        assert_eq!(table.headers.len(), 2);
        assert_eq!(table.rows.len(), 2);
        assert!(table.striped);
    }

    #[test]
    fn test_key_value_table() {
        let table = KeyValueTable::new()
            .item("Name", "Alice")
            .item("Age", "30")
            .label_width(100.0);

        assert_eq!(table.items.len(), 2);
        assert_eq!(table.label_width, 100.0);
    }
}
