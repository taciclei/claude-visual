//! Split View System
//!
//! Provides split pane functionality for displaying multiple conversations
//! or views side by side.

mod pane;
mod split_view;

// Core split modules
mod types;
mod container;
mod render;
#[cfg(test)]
mod tests;

// Re-exports from existing modules
pub use pane::PaneEvent;
pub use split_view::{SplitDirection, SplitView, SplitViewEvent};

// Re-exports from new modules
pub use types::{SplitContainerEvent, Pane, SplitNode};
pub use container::SplitContainer;
