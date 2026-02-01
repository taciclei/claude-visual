//! Side-by-Side Diff View
//!
//! A two-pane diff viewer showing old and new versions side by side.

mod core;
mod render;
mod types;

pub use core::SideBySideDiffView;
pub use types::{DiffDisplayMode, SideBySideDiffEvent};
