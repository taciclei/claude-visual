//! Shared types for gauge components

use gpui::*;

/// Gauge display style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GaugeStyle {
    #[default]
    Arc,
    Semicircle,
    Circle,
    Linear,
}

/// Gauge size preset
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GaugeSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl GaugeSize {
    pub fn diameter(&self) -> f32 {
        match self {
            Self::Xs => 64.0,
            Self::Sm => 96.0,
            Self::Md => 128.0,
            Self::Lg => 192.0,
            Self::Xl => 256.0,
        }
    }

    pub fn stroke_width(&self) -> f32 {
        match self {
            Self::Xs => 6.0,
            Self::Sm => 8.0,
            Self::Md => 10.0,
            Self::Lg => 12.0,
            Self::Xl => 16.0,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            Self::Xs => 14.0,
            Self::Sm => 18.0,
            Self::Md => 24.0,
            Self::Lg => 32.0,
            Self::Xl => 48.0,
        }
    }
}

/// Color zone for gauge values
#[derive(Debug, Clone)]
pub struct GaugeZone {
    pub from: f32,
    pub to: f32,
    pub color: gpui::Hsla,
    pub label: Option<SharedString>,
}

impl GaugeZone {
    pub fn new(from: f32, to: f32, color: gpui::Hsla) -> Self {
        Self {
            from,
            to,
            color,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}
