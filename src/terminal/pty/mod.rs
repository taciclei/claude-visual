//! PTY (Pseudo-Terminal) Support
//!
//! Provides PTY functionality for running interactive shell sessions.

mod config;
mod error;
mod event;
mod key;
mod session;

// Public re-exports for external API
pub use config::PtyConfig;
pub use error::PtyError;
pub use event::PtyEvent;
pub use key::TerminalKey;
pub use session::Pty;
