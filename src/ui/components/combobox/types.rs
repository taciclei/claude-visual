//! Shared types for combobox components

use gpui::*;

/// Combobox size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComboboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Combobox mode - how filtering works
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComboboxMode {
    #[default]
    Filter,      // Filters existing options
    Search,      // Searches for options
    Create,      // Allows creating new options
}

/// Combobox item
#[derive(Debug, Clone)]
pub struct ComboboxItem {
    pub value: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<SharedString>,
    pub disabled: bool,
}

impl ComboboxItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            icon: None,
            disabled: false,
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
