/// Label size options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LabelSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            LabelSize::Small => 11.0,
            LabelSize::Medium => 13.0,
            LabelSize::Large => 15.0,
        }
    }
}

/// Helper text variant options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HelperTextVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}
