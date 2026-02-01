//! Core types for sortable lists

use gpui::prelude::*;
use gpui::*;

/// Sortable item data
#[derive(Clone)]
pub struct SortableItem {
    pub id: SharedString,
    pub content: SharedString,
    pub icon: Option<SharedString>,
    pub disabled: bool,
    pub locked: bool,
}

impl SortableItem {
    pub fn new(id: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            icon: None,
            disabled: false,
            locked: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }
}

/// Drag state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DragState {
    #[default]
    Idle,
    Dragging,
    DragOver,
}

/// Sortable list variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortableVariant {
    #[default]
    Default,
    Cards,
    Compact,
    Bordered,
}

/// Nested sortable list
#[derive(Clone)]
pub struct NestedSortableItem {
    pub id: SharedString,
    pub content: SharedString,
    pub children: Vec<NestedSortableItem>,
    pub collapsed: bool,
}

impl NestedSortableItem {
    pub fn new(id: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            children: Vec::new(),
            collapsed: false,
        }
    }

    pub fn children(mut self, children: Vec<NestedSortableItem>) -> Self {
        self.children = children;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}
