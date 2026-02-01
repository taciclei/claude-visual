//! Pane component for split views

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;

/// Events emitted by a Pane
pub enum PaneEvent {
    /// Pane was focused
    Focused,
    /// Pane requests to be closed
    CloseRequested,
    /// Pane requests a split
    SplitRequested(super::SplitDirection),
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

    /// Create a pane with a specific ID
    pub fn with_id(id: String) -> Self {
        Self {
            id,
            weight: 1.0,
            is_focused: false,
        }
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Set weight
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight.max(0.1);
    }
}

impl Default for Pane {
    fn default() -> Self {
        Self::new()
    }
}
