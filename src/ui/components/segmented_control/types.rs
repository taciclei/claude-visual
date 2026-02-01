//! Shared types for segmented control components

use gpui::*;

/// Segmented control size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SegmentedSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl SegmentedSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            Self::Xs => 24.0,
            Self::Sm => 28.0,
            Self::Md => 32.0,
            Self::Lg => 40.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Xs => 11.0,
            Self::Sm => 12.0,
            Self::Md => 13.0,
            Self::Lg => 14.0,
        }
    }

    pub(crate) fn padding(&self) -> f32 {
        match self {
            Self::Xs => 8.0,
            Self::Sm => 10.0,
            Self::Md => 12.0,
            Self::Lg => 16.0,
        }
    }
}

/// Segmented control variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SegmentedVariant {
    #[default]
    Filled,
    Outline,
    Ghost,
    Pills,
}

/// Segment item
#[derive(Clone)]
pub struct Segment {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<SharedString>,
    pub disabled: bool,
    pub badge: Option<SharedString>,
}

impl Segment {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            badge: None,
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

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}
