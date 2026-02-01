//! Terminal Module
//!
//! Embedded terminal with PTY support for running commands.

mod pty;
mod parser;

pub use pty::{Pty, PtyConfig, PtyError, PtyEvent};
pub use parser::{AnsiParser, AnsiEvent, AnsiColor, TextStyle};
