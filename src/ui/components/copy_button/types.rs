//! Shared types for copy button components

/// Copy button size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CopyButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Copy button variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CopyButtonVariant {
    #[default]
    Default,
    Ghost,
    Outline,
    Subtle,
}

/// Copy button state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CopyState {
    #[default]
    Idle,
    Copying,
    Copied,
    Error,
}

/// Position of copy button in code block
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CopyCodePosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}
