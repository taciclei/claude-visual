//! Select component types and enums

use gpui::*;
use gpui::prelude::*;

/// Select size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Select variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Filled,
}

/// Option item for select
#[derive(Debug, Clone)]
pub struct SelectOption {
    pub value: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub disabled: bool,
    pub icon: Option<SharedString>,
}

impl SelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
            icon: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Option group for grouped selects
#[derive(Debug, Clone)]
pub struct SelectGroup {
    pub label: SharedString,
    pub options: Vec<SelectOption>,
}

impl SelectGroup {
    pub fn new(label: impl Into<SharedString>, options: Vec<SelectOption>) -> Self {
        Self {
            label: label.into(),
            options,
        }
    }
}
