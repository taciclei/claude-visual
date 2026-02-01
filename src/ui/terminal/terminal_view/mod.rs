//! Terminal View Component
//!
//! GPUI component for displaying and interacting with the terminal.

mod core;
mod handlers;
mod render;
mod scroll;
mod selection;
mod tests;
mod traits;
mod types;
mod utils;

// Re-export public types
pub use core::TerminalView;
pub use types::TerminalViewEvent;
