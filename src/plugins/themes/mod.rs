//! Zed-compatible theme loader
//!
//! Loads theme files in Zed's JSON format and converts them
//! to the internal ThemeColors structure.

mod types;
mod parser;
mod defaults;
mod converter;
mod loader;

#[cfg(test)]
mod tests;

// Public exports
pub use types::{
    ZedThemeFile,
    ZedThemeVariant,
    ZedThemeStyle,
    SyntaxStyle,
    PlayerStyle,
    ThemeMetadata,
};
pub use loader::ThemeLoader;
