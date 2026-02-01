//! Diff visualization block component

mod component;
mod parser;
mod render;
mod render_content;
mod render_line;
mod types;

pub use component::DiffBlockView;
pub use types::{DiffBlockEvent, DiffHunk, DiffLine};
