//! SplitContainer - Main container for managing split views

use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;
use super::types::*;

/// Container for managing split views
pub struct SplitContainer {
    pub(crate) app_state: Arc<AppState>,
    /// Root of the split tree
    pub(crate) root: SplitNode,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
    /// Drag state for resizing
    pub(crate) resize_drag: Option<ResizeDrag>,
    /// Maximum number of splits allowed
    max_panes: usize,
}

impl EventEmitter<SplitContainerEvent> for SplitContainer {}

impl SplitContainer {
    /// Create a new split container with a single pane
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut root = SplitNode::pane();
        root.set_focus(0);

        Self {
            app_state,
            root,
            focus_handle: cx.focus_handle(),
            resize_drag: None,
            max_panes: 4, // Maximum 4 panes
        }
    }

    /// Get the root node
    pub fn root(&self) -> &SplitNode {
        &self.root
    }

    /// Get current pane count
    pub fn pane_count(&self) -> usize {
        self.root.pane_count()
    }

    /// Get the focused pane index
    pub fn focused_pane(&self) -> Option<usize> {
        self.root.focused_pane_index()
    }

    /// Focus a specific pane
    pub fn focus_pane(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.root.set_focus(index) {
            cx.emit(SplitContainerEvent::PaneFocused(index));
            cx.notify();
        }
    }

    /// Focus the next pane
    pub fn focus_next(&mut self, cx: &mut Context<Self>) {
        if let Some(current) = self.focused_pane() {
            let next = (current + 1) % self.pane_count();
            self.focus_pane(next, cx);
        }
    }

    /// Focus the previous pane
    pub fn focus_prev(&mut self, cx: &mut Context<Self>) {
        if let Some(current) = self.focused_pane() {
            let count = self.pane_count();
            let prev = if current == 0 { count - 1 } else { current - 1 };
            self.focus_pane(prev, cx);
        }
    }

    /// Split the focused pane
    pub fn split(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if self.pane_count() >= self.max_panes {
            tracing::info!("Maximum pane count ({}) reached", self.max_panes);
            return;
        }

        let focused_idx = match self.focused_pane() {
            Some(idx) => idx,
            None => return,
        };

        // For simplicity, we'll replace the root with a split containing
        // the old root and a new pane
        let old_root = std::mem::replace(&mut self.root, SplitNode::pane());
        let new_pane = SplitNode::pane();

        self.root = match direction {
            SplitDirection::Horizontal => SplitNode::horizontal(vec![old_root, new_pane]),
            SplitDirection::Vertical => SplitNode::vertical(vec![old_root, new_pane]),
        };

        // Keep focus on the original pane
        self.root.set_focus(focused_idx);

        cx.emit(SplitContainerEvent::SplitRequested(direction));
        cx.notify();
    }

    /// Split horizontally (Cmd+\)
    pub fn split_horizontal(&mut self, cx: &mut Context<Self>) {
        self.split(SplitDirection::Horizontal, cx);
    }

    /// Split vertically (Cmd+Shift+\)
    pub fn split_vertical(&mut self, cx: &mut Context<Self>) {
        self.split(SplitDirection::Vertical, cx);
    }

    /// Close the focused pane
    pub fn close_focused_pane(&mut self, cx: &mut Context<Self>) {
        if self.pane_count() <= 1 {
            tracing::info!("Cannot close the last pane");
            return;
        }

        let focused_idx = match self.focused_pane() {
            Some(idx) => idx,
            None => return,
        };

        // Remove the focused pane and simplify the tree
        self.remove_pane_at_index(focused_idx);

        // Focus the previous pane or the first one
        let new_focus = if focused_idx > 0 { focused_idx - 1 } else { 0 };
        self.root.set_focus(new_focus);

        cx.emit(SplitContainerEvent::PaneClosed(focused_idx));
        cx.notify();
    }

    /// Remove pane at index (internal helper)
    fn remove_pane_at_index(&mut self, _target_idx: usize) {
        // For simplicity, just reset to single pane if closing
        // A full implementation would walk the tree and remove the specific pane
        if self.pane_count() <= 1 {
            return;
        }

        // Simplified: just reduce to a single pane
        self.root = SplitNode::pane();
    }

    /// Get focus handle
    pub fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }

    /// Start a resize drag operation
    pub(crate) fn start_resize_drag(
        &mut self,
        direction: SplitDirection,
        divider_index: usize,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        // Get the initial weights of the panes on either side of the divider
        let weights = self.get_weights_at_divider(divider_index);

        self.resize_drag = Some(ResizeDrag {
            direction,
            divider_index,
            start_pos: position,
            initial_weights: weights,
        });
        cx.notify();
    }

    /// Update resize drag with new position
    pub(crate) fn update_resize_drag(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(drag) = &self.resize_drag else {
            return;
        };

        // Calculate delta based on direction
        let delta = match drag.direction {
            SplitDirection::Horizontal => position.x - drag.start_pos.x,
            SplitDirection::Vertical => position.y - drag.start_pos.y,
        };

        // Convert to weight change (rough approximation, 100px = 0.5 weight)
        let delta_f32: f32 = delta.into();
        let weight_delta = delta_f32 / 200.0;

        // Calculate new weights ensuring minimum weight of 0.2
        let min_weight = 0.2;
        let total = drag.initial_weights.0 + drag.initial_weights.1;
        let mut new_weight_0 = (drag.initial_weights.0 + weight_delta).max(min_weight);
        let mut new_weight_1 = (drag.initial_weights.1 - weight_delta).max(min_weight);

        // Normalize to maintain total weight
        let new_total = new_weight_0 + new_weight_1;
        if new_total > 0.0 {
            new_weight_0 = new_weight_0 / new_total * total;
            new_weight_1 = new_weight_1 / new_total * total;
        }

        // Apply new weights
        self.set_weights_at_divider(drag.divider_index, (new_weight_0, new_weight_1));
        cx.notify();
    }

    /// End resize drag operation
    pub(crate) fn end_resize_drag(&mut self, cx: &mut Context<Self>) {
        self.resize_drag = None;
        cx.notify();
    }

    /// Get weights of panes at divider index
    fn get_weights_at_divider(&self, divider_index: usize) -> (f32, f32) {
        match &self.root {
            SplitNode::Split { children, .. } => {
                if divider_index < children.len() {
                    let weight_0 = self.get_pane_weight(&children[divider_index]);
                    let weight_1 = if divider_index + 1 < children.len() {
                        self.get_pane_weight(&children[divider_index + 1])
                    } else {
                        1.0
                    };
                    (weight_0, weight_1)
                } else {
                    (1.0, 1.0)
                }
            }
            SplitNode::Pane(pane) => (pane.weight, 1.0),
        }
    }

    /// Get weight of a node
    fn get_pane_weight(&self, node: &SplitNode) -> f32 {
        match node {
            SplitNode::Pane(pane) => pane.weight,
            SplitNode::Split { children, .. } => {
                // For split nodes, use the average weight of children
                if children.is_empty() {
                    1.0
                } else {
                    children.iter().map(|c| self.get_pane_weight(c)).sum::<f32>()
                        / children.len() as f32
                }
            }
        }
    }

    /// Set weights at divider index
    fn set_weights_at_divider(&mut self, divider_index: usize, weights: (f32, f32)) {
        match &mut self.root {
            SplitNode::Split { children, .. } => {
                if divider_index < children.len() {
                    Self::set_node_weight_static(&mut children[divider_index], weights.0);
                }
                if divider_index + 1 < children.len() {
                    Self::set_node_weight_static(&mut children[divider_index + 1], weights.1);
                }
            }
            SplitNode::Pane(pane) => {
                pane.weight = weights.0;
            }
        }
    }

    /// Set weight of a node (static version)
    fn set_node_weight_static(node: &mut SplitNode, weight: f32) {
        match node {
            SplitNode::Pane(pane) => {
                pane.weight = weight;
            }
            SplitNode::Split { children, .. } => {
                // Distribute weight evenly among children
                let per_child = weight / children.len().max(1) as f32;
                for child in children.iter_mut() {
                    Self::set_node_weight_static(child, per_child);
                }
            }
        }
    }

    /// Set weight of a node
    fn set_node_weight(&mut self, node: &mut SplitNode, weight: f32) {
        Self::set_node_weight_static(node, weight);
    }
}
