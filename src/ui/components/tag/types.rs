//! Tag types and enums

use gpui::*;

/// Tag color variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TagColor {
    /// Default neutral color
    #[default]
    Default,
    /// Primary/accent
    Primary,
    /// Success (green)
    Success,
    /// Warning (yellow/orange)
    Warning,
    /// Error (red)
    Error,
    /// Info (blue)
    Info,
    /// Purple
    Purple,
    /// Pink
    Pink,
}

/// Tag size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TagSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl TagSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            TagSize::Small => 20.0,
            TagSize::Medium => 24.0,
            TagSize::Large => 28.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            TagSize::Small => 10.0,
            TagSize::Medium => 12.0,
            TagSize::Large => 14.0,
        }
    }

    pub(crate) fn padding_x(&self) -> f32 {
        match self {
            TagSize::Small => 6.0,
            TagSize::Medium => 8.0,
            TagSize::Large => 10.0,
        }
    }
}

/// Events emitted by Tag
#[derive(Debug, Clone)]
pub enum TagEvent {
    /// Tag was clicked
    Clicked,
    /// Tag close button was clicked
    Closed,
}

/// Tag item for use in TagGroup
#[derive(Clone)]
pub struct TagGroupItem {
    pub label: String,
    pub color: TagColor,
    pub icon: Option<String>,
}

impl TagGroupItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: TagColor::Default,
            icon: None,
        }
    }

    pub fn with_color(mut self, color: TagColor) -> Self {
        self.color = color;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
