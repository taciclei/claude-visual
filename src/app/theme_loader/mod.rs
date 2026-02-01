//! Async Theme Loading
//!
//! Provides asynchronous theme loading to improve startup performance.
//! Themes are loaded from disk in the background while a default theme is used.

mod async_helpers;
mod builder;
mod discovery;
mod io;
mod loader;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use async_helpers::{load_theme_async, load_theme_with_channel};
pub use builder::ThemeBuilder;
pub use loader::ThemeLoader;
pub use types::{
    PreloadResult, ThemeFormat, ThemeLoadCallback, ThemeLoadError, ThemeLoadResult,
    ThemeLoadState, ThemeMetadata,
};
