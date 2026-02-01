//! Toolbar types and shared enums

use gpui::*;

/// Toolbar variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToolbarVariant {
    #[default]
    Default,
    Floating,
    Attached,
    Minimal,
}

/// Toolbar size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToolbarSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Toolbar position
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToolbarPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// A toolbar item
#[derive(Debug, Clone)]
pub struct ToolbarItem {
    pub id: SharedString,
    pub icon: SharedString,
    pub label: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub active: bool,
    pub disabled: bool,
}

impl ToolbarItem {
    pub fn new(id: impl Into<SharedString>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: None,
            tooltip: None,
            active: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
