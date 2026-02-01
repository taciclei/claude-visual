//! Search input types

/// Search input size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SearchInputSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl SearchInputSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            SearchInputSize::Small => 28.0,
            SearchInputSize::Medium => 36.0,
            SearchInputSize::Large => 44.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            SearchInputSize::Small => 12.0,
            SearchInputSize::Medium => 14.0,
            SearchInputSize::Large => 16.0,
        }
    }

    pub(crate) fn icon_size(&self) -> f32 {
        match self {
            SearchInputSize::Small => 12.0,
            SearchInputSize::Medium => 14.0,
            SearchInputSize::Large => 16.0,
        }
    }
}

/// Events emitted by SearchInput
#[derive(Debug, Clone)]
pub enum SearchInputEvent {
    /// Search query changed
    Changed(String),
    /// Search submitted (Enter pressed)
    Submit(String),
    /// Search cleared
    Cleared,
    /// Focus gained
    Focus,
    /// Focus lost
    Blur,
}
