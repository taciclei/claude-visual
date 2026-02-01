//! Shared types for switch components

/// Switch size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SwitchSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl SwitchSize {
    pub(crate) fn track_width(&self) -> f32 {
        match self {
            SwitchSize::Small => 32.0,
            SwitchSize::Medium => 44.0,
            SwitchSize::Large => 56.0,
        }
    }

    pub(crate) fn track_height(&self) -> f32 {
        match self {
            SwitchSize::Small => 18.0,
            SwitchSize::Medium => 24.0,
            SwitchSize::Large => 30.0,
        }
    }

    pub(crate) fn thumb_size(&self) -> f32 {
        match self {
            SwitchSize::Small => 14.0,
            SwitchSize::Medium => 20.0,
            SwitchSize::Large => 26.0,
        }
    }

    pub(crate) fn thumb_offset(&self) -> f32 {
        // Offset when on (from left edge)
        self.track_width() - self.thumb_size() - 2.0
    }
}

/// Events emitted by Switch
#[derive(Debug, Clone)]
pub enum SwitchEvent {
    /// Switch toggled
    Changed(bool),
}

/// Events emitted by Checkbox
#[derive(Debug, Clone)]
pub enum CheckboxEvent {
    Changed(bool),
}

/// Events emitted by RadioButton
#[derive(Debug, Clone)]
pub enum RadioButtonEvent {
    Selected(String),
}

/// Radio group option
#[derive(Clone)]
pub struct RadioGroupOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl RadioGroupOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Events emitted by RadioGroup
#[derive(Debug, Clone)]
pub enum RadioGroupEvent {
    Changed(String),
}
