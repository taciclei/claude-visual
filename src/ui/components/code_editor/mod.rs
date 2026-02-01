//! Code editor components
//!
//! Provides code editor with line numbers, gutter, and syntax highlighting.

mod types;
mod code_editor;
mod line_numbers;
mod diff_editor;
mod inline_code;

pub use types::*;
pub use code_editor::*;
pub use line_numbers::*;
pub use diff_editor::*;
pub use inline_code::*;

#[cfg(test)]
mod tests;
