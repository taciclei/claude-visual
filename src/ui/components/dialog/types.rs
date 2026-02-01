//! Shared types for dialog components

/// Dialog size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogSize {
    /// Small (320px)
    Small,
    /// Medium (default, 420px)
    #[default]
    Medium,
    /// Large (560px)
    Large,
    /// Full width
    FullWidth,
}

impl DialogSize {
    pub(crate) fn width(&self) -> f32 {
        match self {
            DialogSize::Small => 320.0,
            DialogSize::Medium => 420.0,
            DialogSize::Large => 560.0,
            DialogSize::FullWidth => 0.0, // Will use percentage
        }
    }
}

/// Dialog button style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DialogButtonStyle {
    /// Primary action
    #[default]
    Primary,
    /// Secondary/cancel
    Secondary,
    /// Destructive/danger
    Destructive,
    /// Ghost/text only
    Ghost,
}

/// Dialog button
#[derive(Clone)]
pub struct DialogButton {
    pub id: String,
    pub label: String,
    pub style: DialogButtonStyle,
    pub disabled: bool,
    pub loading: bool,
}

impl DialogButton {
    pub fn primary(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            style: DialogButtonStyle::Primary,
            disabled: false,
            loading: false,
        }
    }

    pub fn secondary(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            style: DialogButtonStyle::Secondary,
            disabled: false,
            loading: false,
        }
    }

    pub fn destructive(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            style: DialogButtonStyle::Destructive,
            disabled: false,
            loading: false,
        }
    }

    pub fn ghost(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            style: DialogButtonStyle::Ghost,
            disabled: false,
            loading: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn loading(mut self) -> Self {
        self.loading = true;
        self
    }
}

/// Events emitted by Dialog
#[derive(Debug, Clone)]
pub enum DialogEvent {
    /// Button clicked
    ButtonClicked(String),
    /// Dialog closed (escape or backdrop)
    Closed,
    /// Backdrop clicked
    BackdropClicked,
}

/// Alert type for AlertDialog
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AlertType {
    pub(crate) fn icon(&self) -> &str {
        match self {
            AlertType::Info => "ℹ️",
            AlertType::Success => "✓",
            AlertType::Warning => "⚠️",
            AlertType::Error => "✕",
        }
    }
}
