//! Shared types for split pane components

/// Split pane orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SplitOrientation {
    /// Horizontal split (left | right)
    #[default]
    Horizontal,
    /// Vertical split (top | bottom)
    Vertical,
}

/// Events emitted by SplitPane
#[derive(Debug, Clone)]
pub enum SplitPaneEvent {
    /// Pane was resized
    Resized { position: f32 },
    /// Pane was collapsed
    Collapsed,
    /// Pane was expanded
    Expanded,
}

/// Sidebar position for collapsible layouts
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarPosition {
    #[default]
    Left,
    Right,
}
