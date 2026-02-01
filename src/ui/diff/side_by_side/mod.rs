//! Side-by-Side Diff View
//!
//! A two-pane diff viewer showing old and new versions side by side.

mod types;
mod core;
mod render;

pub use types::{SideBySideDiffEvent, DiffDisplayMode};
pub use core::SideBySideDiffView;
