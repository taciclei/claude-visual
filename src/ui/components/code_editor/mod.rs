//! Code editor components
//!
//! Provides code editor with line numbers, gutter, and syntax highlighting.

mod code_editor;
mod diff_editor;
mod inline_code;
mod line_numbers;
mod types;

pub use code_editor::*;
pub use diff_editor::*;
pub use inline_code::*;
pub use line_numbers::*;
pub use types::*;

#[cfg(test)]
mod tests;
