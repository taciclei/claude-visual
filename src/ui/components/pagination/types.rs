//! Shared types for pagination components

/// Pagination size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PaginationSize {
    /// Small buttons
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large buttons
    Large,
}

impl PaginationSize {
    pub(crate) fn button_size(&self) -> f32 {
        match self {
            PaginationSize::Small => 28.0,
            PaginationSize::Medium => 32.0,
            PaginationSize::Large => 40.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            PaginationSize::Small => 12.0,
            PaginationSize::Medium => 14.0,
            PaginationSize::Large => 16.0,
        }
    }
}

/// Pagination style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PaginationStyle {
    /// Default with borders
    #[default]
    Default,
    /// Pill-shaped buttons
    Pill,
    /// Simple text-only
    Simple,
    /// Outlined buttons
    Outlined,
}

/// Events emitted by Pagination
#[derive(Debug, Clone)]
pub enum PaginationEvent {
    /// Page changed
    PageChanged(usize),
    /// Page size changed
    PageSizeChanged(usize),
}
