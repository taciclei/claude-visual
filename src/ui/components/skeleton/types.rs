//! Shared types for skeleton components

/// Skeleton shape variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SkeletonShape {
    /// Rectangle (default)
    #[default]
    Rectangle,
    /// Circle
    Circle,
    /// Rounded rectangle
    Rounded,
    /// Text line (thin rectangle)
    Text,
}
