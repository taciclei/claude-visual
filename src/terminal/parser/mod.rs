//! ANSI Escape Code Parser
//!
//! Parses ANSI escape sequences for terminal rendering.

mod types;
mod color;
mod style;
mod state;
mod events;
mod escape;
mod sgr;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{AnsiColor, AnsiEvent, TextStyle};
pub use state::AnsiParser;
