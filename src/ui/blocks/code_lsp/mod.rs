//! Code Block LSP Integration
//!
//! Provides LSP features for code blocks including go-to-definition,
//! hover information, and symbol highlighting.

mod core;
#[cfg(test)]
mod tests;
mod tokenizer;
mod types;

// Re-export public types
pub use types::{ClickableToken, CodeLspConfig, CodeLspEvent, CodeToken, TokenType};

pub use core::CodeLspIntegration;
