//! Type definitions for hover cards

/// Hover card position relative to trigger
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HoverCardPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

/// Hover card arrow position
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ArrowPosition {
    Start,
    #[default]
    Center,
    End,
    Hidden,
}
