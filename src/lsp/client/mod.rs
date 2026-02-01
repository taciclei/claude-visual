//! LSP Client
//!
//! Client for communicating with Language Server Protocol servers.

mod core;
mod document;
mod features;
mod messaging;
mod types;

#[cfg(test)]
mod tests;

pub use core::LspClient;
pub use types::{LspClientConfig, LspEvent};
