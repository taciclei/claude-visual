//! Table component implementation

use std::sync::Arc;

use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Table component
pub struct Table {
    pub(super) app_state: Arc<AppState>,
    /// Column definitions
    pub(super) columns: Vec<TableColumn>,
    /// Row data
    pub(super) rows: Vec<TableRow>,
    /// Selected row indices
    pub(super) selected: Vec<usize>,
    /// Current sort state
    pub(super) sort: Option<TableSort>,
    /// Size variant
    pub(super) size: TableSize,
    /// Style variant
    pub(super) style: TableStyle,
    /// Whether to show header
    pub(super) show_header: bool,
    /// Whether to allow row selection
    pub(super) selectable: bool,
    /// Whether to allow multi-selection
    pub(super) multi_select: bool,
    /// Whether to show row hover
    pub(super) hoverable: bool,
    /// Hovered row index
    pub(super) hovered_row: Option<usize>,
}

impl Table {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            columns: Vec::new(),
            rows: Vec::new(),
            selected: Vec::new(),
            sort: None,
            size: TableSize::default(),
            style: TableStyle::default(),
            show_header: true,
            selectable: true,
            multi_select: false,
            hoverable: true,
            hovered_row: None,
        }
    }

    /// Set columns
    pub fn set_columns(&mut self, columns: Vec<TableColumn>, cx: &mut Context<Self>) {
        self.columns = columns;
        cx.notify();
    }

    /// Set rows
    pub fn set_rows(&mut self, rows: Vec<TableRow>, cx: &mut Context<Self>) {
        self.rows = rows;
        self.selected.clear();
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: TableSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set style
    pub fn set_style(&mut self, style: TableStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set show header
    pub fn set_show_header(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_header = show;
        cx.notify();
    }

    /// Set selectable
    pub fn set_selectable(&mut self, selectable: bool, cx: &mut Context<Self>) {
        self.selectable = selectable;
        if !selectable {
            self.selected.clear();
        }
        cx.notify();
    }

    /// Set multi-select
    pub fn set_multi_select(&mut self, multi: bool, cx: &mut Context<Self>) {
        self.multi_select = multi;
        if !multi && self.selected.len() > 1 {
            self.selected = vec![self.selected[0]];
        }
        cx.notify();
    }

    /// Set hoverable
    pub fn set_hoverable(&mut self, hoverable: bool, cx: &mut Context<Self>) {
        self.hoverable = hoverable;
        if !hoverable {
            self.hovered_row = None;
        }
        cx.notify();
    }

    /// Select row
    pub fn select_row(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.selectable || index >= self.rows.len() || self.rows[index].disabled {
            return;
        }

        if self.multi_select {
            if !self.selected.contains(&index) {
                self.selected.push(index);
            }
        } else {
            self.selected = vec![index];
        }
        cx.emit(TableEvent::SelectionChanged(self.selected.clone()));
        cx.notify();
    }

    /// Deselect row
    pub fn deselect_row(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected.retain(|&i| i != index);
        cx.emit(TableEvent::SelectionChanged(self.selected.clone()));
        cx.notify();
    }

    /// Toggle row selection
    pub fn toggle_row(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.selected.contains(&index) {
            self.deselect_row(index, cx);
        } else {
            self.select_row(index, cx);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selected.clear();
        cx.emit(TableEvent::SelectionChanged(vec![]));
        cx.notify();
    }

    /// Get selected rows
    pub fn selected(&self) -> &[usize] {
        &self.selected
    }

    /// Sort by column
    pub fn sort_by(&mut self, column: &str, cx: &mut Context<Self>) {
        let direction = match &self.sort {
            Some(sort) if sort.column == column => match sort.direction {
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::Ascending,
            },
            _ => SortDirection::Ascending,
        };

        let sort = TableSort {
            column: column.to_string(),
            direction,
        };

        self.sort = Some(sort.clone());
        cx.emit(TableEvent::SortChanged(sort));
        cx.notify();
    }

    /// Handle row click
    pub(super) fn handle_row_click(&mut self, index: usize, cx: &mut Context<Self>) {
        cx.emit(TableEvent::RowClick(index));
        if self.selectable && self.rows.get(index).map(|r| r.selectable).unwrap_or(false) {
            self.toggle_row(index, cx);
        }
    }
}

impl EventEmitter<TableEvent> for Table {}
