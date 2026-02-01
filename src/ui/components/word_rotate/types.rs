//! Shared types for word rotation components

/// Animation type for word rotation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RotateAnimation {
    #[default]
    Fade,
    SlideUp,
    SlideDown,
    SlideLeft,
    SlideRight,
    Flip,
    Blur,
    Scale,
}

/// Word rotation speed
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RotateSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}
