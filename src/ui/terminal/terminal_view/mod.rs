//! Terminal View Component
//!
//! GPUI component for displaying and interacting with the terminal.

mod types;
mod core;
mod handlers;
mod scroll;
mod selection;
mod utils;
mod traits;
mod render;
mod tests;

// Re-export public types
pub use types::TerminalViewEvent;
pub use core::TerminalView;
