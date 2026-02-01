//! Type definitions for lazy loading

use gpui::*;

/// State of lazy loading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyState {
    /// Not yet initialized (placeholder shown)
    Pending,
    /// Currently loading content
    Loading,
    /// Content loaded and rendered
    Loaded,
    /// Failed to load
    Error,
}

/// Events emitted by lazy blocks
#[derive(Debug, Clone)]
pub enum LazyBlockEvent {
    /// Block became visible
    BecameVisible,
    /// Block became hidden
    BecameHidden,
    /// Content loaded successfully
    ContentLoaded,
    /// Loading failed
    LoadingFailed(String),
}

/// Simple color palette for rendering
pub(crate) struct SimpleColors {
    pub(crate) surface: Hsla,
    pub(crate) border: Hsla,
    pub(crate) error: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
    }
}

/// Position of a block in the container
#[derive(Debug, Clone)]
pub(crate) struct BlockPosition {
    /// Block ID
    pub(crate) id: String,
    /// Top position (relative to container)
    pub(crate) top: f32,
    /// Height
    pub(crate) height: f32,
    /// Whether currently visible
    pub(crate) is_visible: bool,
}
