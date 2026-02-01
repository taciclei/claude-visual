//! Shared types for sheet components

/// Sheet position
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetPosition {
    /// Bottom (default)
    #[default]
    Bottom,
    /// Top
    Top,
    /// Left
    Left,
    /// Right
    Right,
}

/// Sheet size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSize {
    /// Auto (content height)
    #[default]
    Auto,
    /// Small (30%)
    Small,
    /// Medium (50%)
    Medium,
    /// Large (75%)
    Large,
    /// Full
    Full,
    /// Custom percentage
    Custom(f32),
}

impl SheetSize {
    pub(crate) fn percentage(&self) -> f32 {
        match self {
            SheetSize::Auto => 0.0, // Will be auto
            SheetSize::Small => 0.3,
            SheetSize::Medium => 0.5,
            SheetSize::Large => 0.75,
            SheetSize::Full => 1.0,
            SheetSize::Custom(pct) => pct.clamp(0.0, 1.0),
        }
    }
}

/// Events emitted by Sheet
#[derive(Debug, Clone)]
pub enum SheetEvent {
    /// Sheet opened
    Opened,
    /// Sheet closed
    Closed,
    /// Drag handle released
    DragReleased(f32), // Height percentage
    /// Backdrop clicked
    BackdropClicked,
}

/// Action for ActionSheet
#[derive(Clone)]
pub struct SheetAction {
    pub label: String,
    pub destructive: bool,
    pub disabled: bool,
}

impl SheetAction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            destructive: false,
            disabled: false,
        }
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Share item for ShareSheet
#[derive(Clone)]
pub struct ShareItem {
    pub label: String,
    pub icon: String,
}
