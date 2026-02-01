//! Shared types for floating action components

use gpui::*;

/// FAB size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FabSize {
    Sm,
    #[default]
    Md,
    Lg,
    Extended,
}

impl FabSize {
    pub(crate) fn dimensions(&self) -> (f32, f32) {
        match self {
            Self::Sm => (40.0, 40.0),
            Self::Md => (56.0, 56.0),
            Self::Lg => (72.0, 72.0),
            Self::Extended => (0.0, 48.0), // Width is auto
        }
    }

    pub(crate) fn icon_size(&self) -> f32 {
        match self {
            Self::Sm => 18.0,
            Self::Md => 24.0,
            Self::Lg => 32.0,
            Self::Extended => 20.0,
        }
    }
}

/// FAB variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FabVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Surface,
}

impl FabVariant {
    pub(crate) fn colors(&self) -> (Hsla, Hsla) {
        match self {
            Self::Primary => (hsla(0.6, 0.7, 0.5, 1.0), hsla(0.0, 0.0, 1.0, 1.0)),
            Self::Secondary => (hsla(0.6, 0.3, 0.3, 1.0), hsla(0.6, 0.7, 0.5, 1.0)),
            Self::Tertiary => (hsla(0.0, 0.0, 0.2, 1.0), hsla(0.6, 0.7, 0.5, 1.0)),
            Self::Surface => (hsla(0.0, 0.0, 0.15, 1.0), hsla(0.0, 0.0, 0.9, 1.0)),
        }
    }
}

/// FAB position
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FabPosition {
    #[default]
    BottomRight,
    BottomLeft,
    BottomCenter,
    TopRight,
    TopLeft,
}

/// Speed dial item
#[derive(Clone)]
pub struct SpeedDialItem {
    pub id: SharedString,
    pub icon: SharedString,
    pub label: Option<SharedString>,
    pub disabled: bool,
}

impl SpeedDialItem {
    pub fn new(id: impl Into<SharedString>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: None,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Speed dial direction
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpeedDialDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
