//! ANSI Escape Code Parser
//!
//! Parses ANSI escape sequences for terminal rendering.

mod color;
mod escape;
mod events;
mod sgr;
mod state;
mod style;
mod types;

#[cfg(test)]
mod tests;

// Re-export public types
pub use state::AnsiParser;
pub use types::{AnsiColor, AnsiEvent, TextStyle};
