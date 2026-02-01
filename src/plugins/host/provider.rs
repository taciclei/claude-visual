//! Extension provider traits and capabilities

use std::sync::Arc;

use crate::plugins::ExtensionManifest;

/// Extension capabilities that can be provided
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionCapability {
    /// Language support (syntax, LSP)
    Language,
    /// Theme definitions
    Theme,
    /// Slash commands
    SlashCommand,
    /// Snippets
    Snippet,
}

/// Trait for extension providers
pub trait ExtensionProvider: Send + Sync {
    /// Get the capabilities this extension provides
    fn capabilities(&self) -> &[ExtensionCapability];

    /// Get extension manifest
    fn manifest(&self) -> &ExtensionManifest;
}

/// Arc wrapper for thread-safe extension sharing
pub type SharedExtension = Arc<dyn ExtensionProvider>;
