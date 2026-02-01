//! Shared types for resizable components

/// Resize direction
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ResizeDirection {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

/// Handle position for resize
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HandlePosition {
    Start,
    #[default]
    End,
    Both,
}

/// Handle style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HandleStyle {
    #[default]
    Line,
    Dots,
    Grip,
    Hidden,
}

/// Corner position for corner resize handles
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    #[default]
    BottomRight,
}
