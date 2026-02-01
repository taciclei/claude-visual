//! Split node tree structure

use super::types::SplitDirection;
use crate::ui::split::Pane;

/// A node in the split tree
#[derive(Debug, Clone)]
pub enum SplitNode {
    /// A leaf node containing a pane
    Leaf(Pane),
    /// A split node containing children
    Split {
        direction: SplitDirection,
        children: Vec<SplitNode>,
    },
}

impl SplitNode {
    /// Create a new leaf pane
    pub fn leaf() -> Self {
        SplitNode::Leaf(Pane::new())
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
            SplitNode::Leaf(_) => 1,
            SplitNode::Split { children, .. } => children.iter().map(|c| c.pane_count()).sum(),
        }
    }

    /// Find the focused pane index
    pub fn focused_index(&self) -> Option<usize> {
        self.find_focused(0)
    }

    fn find_focused(&self, start_idx: usize) -> Option<usize> {
        match self {
            SplitNode::Leaf(pane) => {
                if pane.is_focused {
                    Some(start_idx)
                } else {
                    None
                }
            }
            SplitNode::Split { children, .. } => {
                let mut idx = start_idx;
                for child in children {
                    if let Some(result) = child.find_focused(idx) {
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
        self.set_focus_internal(target_idx, 0)
    }

    fn set_focus_internal(&mut self, target_idx: usize, start_idx: usize) -> bool {
        match self {
            SplitNode::Leaf(pane) => {
                let should_focus = start_idx == target_idx;
                pane.is_focused = should_focus;
                should_focus
            }
            SplitNode::Split { children, .. } => {
                let mut idx = start_idx;
                let mut found = false;
                for child in children.iter_mut() {
                    if child.set_focus_internal(target_idx, idx) {
                        found = true;
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
            SplitNode::Leaf(pane) => pane.is_focused = false,
            SplitNode::Split { children, .. } => {
                for child in children.iter_mut() {
                    child.clear_focus();
                }
            }
        }
    }

    /// Get pane at index
    pub fn get_pane(&self, target_idx: usize) -> Option<&Pane> {
        self.get_pane_internal(target_idx, 0)
    }

    fn get_pane_internal(&self, target_idx: usize, start_idx: usize) -> Option<&Pane> {
        match self {
            SplitNode::Leaf(pane) => {
                if start_idx == target_idx {
                    Some(pane)
                } else {
                    None
                }
            }
            SplitNode::Split { children, .. } => {
                let mut idx = start_idx;
                for child in children {
                    let count = child.pane_count();
                    if target_idx < idx + count {
                        return child.get_pane_internal(target_idx, idx);
                    }
                    idx += count;
                }
                None
            }
        }
    }
}
