//! Shared types for dropdown components

/// Dropdown size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DropdownSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl DropdownSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            DropdownSize::Small => 28.0,
            DropdownSize::Medium => 36.0,
            DropdownSize::Large => 44.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            DropdownSize::Small => 12.0,
            DropdownSize::Medium => 14.0,
            DropdownSize::Large => 16.0,
        }
    }

    pub(crate) fn padding_x(&self) -> f32 {
        match self {
            DropdownSize::Small => 8.0,
            DropdownSize::Medium => 12.0,
            DropdownSize::Large => 16.0,
        }
    }
}

/// A dropdown option
#[derive(Clone, Debug)]
pub struct DropdownOption {
    /// Unique identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/emoji
    pub icon: Option<String>,
    /// Whether option is disabled
    pub disabled: bool,
}

impl DropdownOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon: None,
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Events emitted by Dropdown
#[derive(Debug, Clone)]
pub enum DropdownEvent {
    /// Selection changed
    Changed(String),
    /// Dropdown opened
    Opened,
    /// Dropdown closed
    Closed,
}

/// Events for OptionList
#[derive(Debug, Clone)]
pub enum OptionListEvent {
    Changed(Option<String>),
}
