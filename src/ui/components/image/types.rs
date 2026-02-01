//! Type definitions for image components

/// Image fit options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ImageFit {
    #[default]
    Cover,
    Contain,
    Fill,
    None,
    ScaleDown,
}

/// Image loading state
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ImageState {
    #[default]
    Loading,
    Loaded,
    Error,
}

/// Image shape options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ImageShape {
    #[default]
    Rectangle,
    Rounded,
    Circle,
    Square,
}

/// Caption position for Figure component
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CaptionPosition {
    #[default]
    Bottom,
    Top,
    Overlay,
}
