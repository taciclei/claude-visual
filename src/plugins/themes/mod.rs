//! Zed-compatible theme loader
//!
//! Loads theme files in Zed's JSON format and converts them
//! to the internal ThemeColors structure.

mod converter;
mod defaults;
mod loader;
mod parser;
mod types;

#[cfg(test)]
mod tests;

// Public exports
pub use loader::ThemeLoader;
pub use types::{
    PlayerStyle, SyntaxStyle, ThemeMetadata, ZedThemeFile, ZedThemeStyle, ZedThemeVariant,
};
