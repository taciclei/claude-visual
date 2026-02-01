//! Terminal Module
//!
//! Embedded terminal with PTY support for running commands.

mod parser;
mod pty;

pub use parser::{AnsiColor, AnsiEvent, AnsiParser, TextStyle};
pub use pty::{Pty, PtyConfig, PtyError, PtyEvent};
