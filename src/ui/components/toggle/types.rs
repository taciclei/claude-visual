//! Shared types for toggle components

/// Toggle size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Toggle variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Subtle,
}

/// Text style type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextStyleType {
    #[default]
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Link,
}
