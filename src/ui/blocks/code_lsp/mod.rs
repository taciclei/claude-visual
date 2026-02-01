//! Code Block LSP Integration
//!
//! Provides LSP features for code blocks including go-to-definition,
//! hover information, and symbol highlighting.

mod types;
mod tokenizer;
mod core;
#[cfg(test)]
mod tests;

// Re-export public types
pub use types::{
    CodeLspEvent,
    CodeToken,
    TokenType,
    CodeLspConfig,
    ClickableToken,
};

pub use core::CodeLspIntegration;
