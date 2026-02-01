//! Core types for the split view system

pub use crate::ui::split::split_view::SplitDirection;
use gpui::Point;

/// Events emitted by the SplitContainer
pub enum SplitContainerEvent {
    /// A pane was focused
    PaneFocused(usize),
    /// A pane was closed
    PaneClosed(usize),
    /// Split was requested
    SplitRequested(SplitDirection),
}

/// A single pane in a split view
#[derive(Debug, Clone)]
pub struct Pane {
    /// Unique identifier for this pane
    pub id: String,
    /// Weight for size calculation (relative to siblings)
    pub weight: f32,
    /// Whether this pane is focused
    pub is_focused: bool,
}

impl Pane {
    /// Create a new pane
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            weight: 1.0,
            is_focused: false,
        }
    }
}

impl Default for Pane {
    fn default() -> Self {
        Self::new()
    }
}

/// A node in the split tree
#[derive(Debug, Clone)]
pub enum SplitNode {
    /// A leaf node containing a pane
    Pane(Pane),
    /// A split node containing children
    Split {
        direction: SplitDirection,
        children: Vec<SplitNode>,
    },
}

impl SplitNode {
    /// Create a new leaf pane
    pub fn pane() -> Self {
        SplitNode::Pane(Pane::new())
    }

    /// Create a horizontal split
    pub fn horizontal(children: Vec<SplitNode>) -> Self {
        SplitNode::Split {
            direction: SplitDirection::Horizontal,
            children,
        }
    }

    /// Create a vertical split
    pub fn vertical(children: Vec<SplitNode>) -> Self {
        SplitNode::Split {
            direction: SplitDirection::Vertical,
            children,
        }
    }

    /// Count total panes in this node
    pub fn pane_count(&self) -> usize {
        match self {
            SplitNode::Pane(_) => 1,
            SplitNode::Split { children, .. } => children.iter().map(|c| c.pane_count()).sum(),
        }
    }

    /// Find the focused pane index
    pub fn focused_pane_index(&self) -> Option<usize> {
        self.find_focused_index(0).map(|(idx, _)| idx)
    }

    fn find_focused_index(&self, start_idx: usize) -> Option<(usize, &Pane)> {
        match self {
            SplitNode::Pane(pane) => {
                if pane.is_focused {
                    Some((start_idx, pane))
                } else {
                    None
                }
            }
            SplitNode::Split { children, .. } => {
                let mut idx = start_idx;
                for child in children {
                    if let Some(result) = child.find_focused_index(idx) {
                        return Some(result);
                    }
                    idx += child.pane_count();
                }
                None
            }
        }
    }

    /// Set focus on pane at index
    pub fn set_focus(&mut self, target_idx: usize) -> bool {
        self.set_focus_internal(target_idx, 0).is_some()
    }

    fn set_focus_internal(&mut self, target_idx: usize, start_idx: usize) -> Option<usize> {
        match self {
            SplitNode::Pane(pane) => {
                if start_idx == target_idx {
                    pane.is_focused = true;
                    Some(start_idx)
                } else {
                    pane.is_focused = false;
                    None
                }
            }
            SplitNode::Split { children, .. } => {
                let mut idx = start_idx;
                let mut found = None;
                for child in children.iter_mut() {
                    if let Some(result) = child.set_focus_internal(target_idx, idx) {
                        found = Some(result);
                    }
                    idx += child.pane_count();
                }
                found
            }
        }
    }

    /// Clear all focus
    pub fn clear_focus(&mut self) {
        match self {
            SplitNode::Pane(pane) => pane.is_focused = false,
            SplitNode::Split { children, .. } => {
                for child in children.iter_mut() {
                    child.clear_focus();
                }
            }
        }
    }
}

/// State for resize drag operation
#[derive(Debug, Clone)]
pub(crate) struct ResizeDrag {
    /// Direction of the split being resized
    pub(crate) direction: SplitDirection,
    /// Index of the divider being dragged (between child N and N+1)
    pub(crate) divider_index: usize,
    /// Initial mouse position
    pub(crate) start_pos: Point<gpui::Pixels>,
    /// Initial weights of the two panes
    pub(crate) initial_weights: (f32, f32),
}
