//! Diff visualization block component

mod types;
mod parser;
mod component;
mod render;
mod render_content;
mod render_line;

pub use types::{DiffLine, DiffHunk, DiffBlockEvent};
pub use component::DiffBlockView;
