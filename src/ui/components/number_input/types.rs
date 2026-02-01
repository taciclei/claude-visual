//! Shared types for number input components

/// Number input size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum NumberInputSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl NumberInputSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            NumberInputSize::Small => 28.0,
            NumberInputSize::Medium => 36.0,
            NumberInputSize::Large => 44.0,
        }
    }

    pub(crate) fn button_size(&self) -> f32 {
        match self {
            NumberInputSize::Small => 20.0,
            NumberInputSize::Medium => 24.0,
            NumberInputSize::Large => 32.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            NumberInputSize::Small => 12.0,
            NumberInputSize::Medium => 14.0,
            NumberInputSize::Large => 16.0,
        }
    }
}

/// Events emitted by NumberInput
#[derive(Debug, Clone)]
pub enum NumberInputEvent {
    /// Value changed
    Changed(f64),
    /// Focus gained
    Focus,
    /// Focus lost
    Blur,
}
