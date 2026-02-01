//! Watermark and overlay components
//!
//! Provides watermark overlays, patterns, and security marks.

mod types;
mod watermark;
mod image_watermark;
mod security_watermark;
mod stamp_overlay;
mod pattern_overlay;

pub use types::*;
pub use watermark::*;
pub use image_watermark::*;
pub use security_watermark::*;
pub use stamp_overlay::*;
pub use pattern_overlay::*;

#[cfg(test)]
mod tests;
