//! Slider types and enums

/// Slider size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SliderSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl SliderSize {
    pub(crate) fn track_height(&self) -> f32 {
        match self {
            SliderSize::Small => 4.0,
            SliderSize::Medium => 6.0,
            SliderSize::Large => 8.0,
        }
    }

    pub(crate) fn thumb_size(&self) -> f32 {
        match self {
            SliderSize::Small => 12.0,
            SliderSize::Medium => 16.0,
            SliderSize::Large => 20.0,
        }
    }
}

/// Events emitted by Slider
#[derive(Debug, Clone)]
pub enum SliderEvent {
    /// Value changed (value, is_dragging)
    Changed(f32, bool),
    /// Drag started
    DragStart,
    /// Drag ended
    DragEnd,
}

/// Events emitted by RangeSlider
#[derive(Debug, Clone)]
pub enum RangeSliderEvent {
    Changed(f32, f32),
}

/// Mark/tick on slider track
#[derive(Clone)]
pub struct SliderMark {
    pub value: f32,
    pub label: Option<String>,
}

impl SliderMark {
    pub fn new(value: f32) -> Self {
        Self { value, label: None }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}
