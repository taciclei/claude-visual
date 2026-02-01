//! Theme management with accessibility support

mod colors;
mod defaults;
mod theme;
mod types;

#[cfg(test)]
mod tests;

// Re-export main types
pub use theme::Theme;
pub use types::{AccessibilitySettings, SyntaxColors, ThemeColors, ThemeVariant};

// Re-export helper functions
pub(crate) use colors::{contrast_ratio, hsla, meets_wcag_aa, meets_wcag_aaa, relative_luminance};
