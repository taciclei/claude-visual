//! Type definitions for checkbox components

use gpui::*;

/// Checkbox size options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CheckboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl CheckboxSize {
    pub(super) fn box_size(&self) -> f32 {
        match self {
            CheckboxSize::Small => 16.0,
            CheckboxSize::Medium => 20.0,
            CheckboxSize::Large => 24.0,
        }
    }

    pub(super) fn check_size(&self) -> f32 {
        match self {
            CheckboxSize::Small => 10.0,
            CheckboxSize::Medium => 12.0,
            CheckboxSize::Large => 14.0,
        }
    }

    pub(super) fn font_size(&self) -> f32 {
        match self {
            CheckboxSize::Small => 13.0,
            CheckboxSize::Medium => 14.0,
            CheckboxSize::Large => 16.0,
        }
    }
}

/// Checkbox state
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

/// Checkbox option for CheckboxGroup
#[derive(Clone)]
pub struct CheckboxOption {
    pub id: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub disabled: bool,
}

impl CheckboxOption {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            disabled: false,
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
}

/// Checkbox group orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CheckboxGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Checkbox card for card-style checkbox groups
#[derive(Clone)]
pub struct CheckboxCardOption {
    pub id: SharedString,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<SharedString>,
    pub price: Option<SharedString>,
    pub disabled: bool,
}

impl CheckboxCardOption {
    pub fn new(id: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            icon: None,
            price: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn price(mut self, price: impl Into<SharedString>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
