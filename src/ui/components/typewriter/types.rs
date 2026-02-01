//! Shared types and enums for typewriter components

/// Typewriter animation style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TypewriterStyle {
    #[default]
    Classic,
    Modern,
    Terminal,
    Glitch,
}

/// Cursor style for typewriter
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TypewriterCursor {
    #[default]
    Line,
    Block,
    Underscore,
    None,
}

/// Animation direction
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AnimationDirection {
    #[default]
    Forward,
    Backward,
    Loop,
    PingPong,
}

/// Typing indicator variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TypingIndicatorVariant {
    #[default]
    Dots,
    Wave,
    Pulse,
    Bounce,
}

/// Typing indicator size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TypingIndicatorSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Text animation effect
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextEffect {
    #[default]
    FadeIn,
    SlideUp,
    SlideDown,
    Scale,
    Blur,
    Gradient,
    Highlight,
    Scramble,
}

/// Character reveal style
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RevealStyle {
    #[default]
    Fade,
    Drop,
    Slide,
    Flip,
    Glow,
}
