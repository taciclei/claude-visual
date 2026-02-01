//! Split View System
//!
//! Provides split pane functionality for displaying multiple conversations
//! or views side by side.

mod pane;
mod split_view;

// Core split modules
mod container;
mod render;
#[cfg(test)]
mod tests;
mod types;

// Re-exports from existing modules
pub use pane::PaneEvent;
pub use split_view::{SplitDirection, SplitView, SplitViewEvent};

// Re-exports from new modules
pub use container::SplitContainer;
pub use types::{Pane, SplitContainerEvent, SplitNode};
