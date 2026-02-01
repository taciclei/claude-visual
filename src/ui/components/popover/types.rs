//! Shared types for popover components

/// Popover placement options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PopoverPlacement {
    /// Above the trigger
    Top,
    /// Above and aligned to start
    TopStart,
    /// Above and aligned to end
    TopEnd,
    /// Below the trigger (default)
    #[default]
    Bottom,
    /// Below and aligned to start
    BottomStart,
    /// Below and aligned to end
    BottomEnd,
    /// To the left
    Left,
    /// To the left and aligned to start
    LeftStart,
    /// To the left and aligned to end
    LeftEnd,
    /// To the right
    Right,
    /// To the right and aligned to start
    RightStart,
    /// To the right and aligned to end
    RightEnd,
}

/// Popover trigger modes
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PopoverTrigger {
    /// Open on click (default)
    #[default]
    Click,
    /// Open on hover
    Hover,
    /// Manually controlled
    Manual,
}

/// Events emitted by Popover
#[derive(Debug, Clone)]
pub enum PopoverEvent {
    /// Popover opened
    Opened,
    /// Popover closed
    Closed,
}
