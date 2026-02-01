//! Type definitions for toggle group components

use gpui::*;

/// Toggle group size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ToggleGroupSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ToggleGroupSize {
    pub fn height(&self) -> f32 {
        match self {
            ToggleGroupSize::Small => 28.0,
            ToggleGroupSize::Medium => 36.0,
            ToggleGroupSize::Large => 44.0,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            ToggleGroupSize::Small => 12.0,
            ToggleGroupSize::Medium => 14.0,
            ToggleGroupSize::Large => 16.0,
        }
    }

    pub fn padding(&self) -> f32 {
        match self {
            ToggleGroupSize::Small => 8.0,
            ToggleGroupSize::Medium => 12.0,
            ToggleGroupSize::Large => 16.0,
        }
    }
}

/// Toggle group style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ToggleGroupVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Pill,
}

/// A single toggle item in a group
#[derive(Clone)]
pub struct ToggleItem {
    pub value: SharedString,
    pub label: SharedString,
    pub icon: Option<SharedString>,
    pub disabled: bool,
}

impl ToggleItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
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
}

/// Button group item
#[derive(Clone)]
pub struct ButtonGroupItem {
    pub label: SharedString,
    pub icon: Option<SharedString>,
    pub active: bool,
    pub disabled: bool,
}

impl ButtonGroupItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            active: false,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
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

/// Button group style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonGroupVariant {
    #[default]
    Default,
    Outline,
    Ghost,
}

/// Icon toggle item
#[derive(Clone)]
pub struct IconToggleItem {
    pub value: SharedString,
    pub icon: SharedString,
    pub tooltip: Option<SharedString>,
    pub disabled: bool,
}

impl IconToggleItem {
    pub fn new(value: impl Into<SharedString>, icon: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            icon: icon.into(),
            tooltip: None,
            disabled: false,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
