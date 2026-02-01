//! Split view types and events

/// Split direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Horizontal split (panes side by side)
    Horizontal,
    /// Vertical split (panes stacked)
    Vertical,
}

/// Events emitted by the SplitView
pub enum SplitViewEvent {
    /// A pane was focused
    PaneFocused(usize),
    /// A pane was closed
    PaneClosed(usize),
    /// Split layout changed
    LayoutChanged,
}
