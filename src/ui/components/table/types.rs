//! Table type definitions

use gpui::prelude::*;
use gpui::*;

/// Table column alignment
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ColumnAlign {
    /// Left aligned (default)
    #[default]
    Left,
    /// Center aligned
    Center,
    /// Right aligned
    Right,
}

/// Table size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TableSize {
    /// Compact with less padding
    Compact,
    /// Default size
    #[default]
    Default,
    /// Comfortable with more padding
    Comfortable,
}

impl TableSize {
    pub fn row_height(&self) -> f32 {
        match self {
            TableSize::Compact => 32.0,
            TableSize::Default => 40.0,
            TableSize::Comfortable => 48.0,
        }
    }

    pub fn cell_padding(&self) -> f32 {
        match self {
            TableSize::Compact => 8.0,
            TableSize::Default => 12.0,
            TableSize::Comfortable => 16.0,
        }
    }
}

/// Table style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TableStyle {
    /// Default with borders
    #[default]
    Default,
    /// Striped rows
    Striped,
    /// Bordered cells
    Bordered,
    /// Minimal without borders
    Minimal,
}

/// Column definition
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Column key/id
    pub key: String,
    /// Column header text
    pub header: String,
    /// Column width (None = auto)
    pub width: Option<f32>,
    /// Minimum width
    pub min_width: Option<f32>,
    /// Maximum width
    pub max_width: Option<f32>,
    /// Text alignment
    pub align: ColumnAlign,
    /// Whether column is sortable
    pub sortable: bool,
    /// Whether column is resizable
    pub resizable: bool,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, header: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            header: header.into(),
            width: None,
            min_width: None,
            max_width: None,
            align: ColumnAlign::default(),
            sortable: false,
            resizable: true,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn min_width(mut self, min: f32) -> Self {
        self.min_width = Some(min);
        self
    }

    pub fn max_width(mut self, max: f32) -> Self {
        self.max_width = Some(max);
        self
    }

    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn not_resizable(mut self) -> Self {
        self.resizable = false;
        self
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Table sort state
#[derive(Debug, Clone)]
pub struct TableSort {
    pub column: String,
    pub direction: SortDirection,
}

/// Events emitted by Table
#[derive(Debug, Clone)]
pub enum TableEvent {
    /// Row clicked
    RowClick(usize),
    /// Row double-clicked
    RowDoubleClick(usize),
    /// Row selected/deselected
    SelectionChanged(Vec<usize>),
    /// Sort changed
    SortChanged(TableSort),
    /// Column resized
    ColumnResized { column: String, width: f32 },
}

/// Table row data
#[derive(Debug, Clone)]
pub struct TableRow {
    /// Cell values by column key
    pub cells: Vec<String>,
    /// Whether row is selectable
    pub selectable: bool,
    /// Whether row is disabled
    pub disabled: bool,
}

impl TableRow {
    pub fn new(cells: Vec<impl Into<String>>) -> Self {
        Self {
            cells: cells.into_iter().map(|c| c.into()).collect(),
            selectable: true,
            disabled: false,
        }
    }

    pub fn not_selectable(mut self) -> Self {
        self.selectable = false;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
