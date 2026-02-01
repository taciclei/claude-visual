//! Watermark and overlay components
//!
//! Provides watermark overlays, patterns, and security marks.

mod image_watermark;
mod pattern_overlay;
mod security_watermark;
mod stamp_overlay;
mod types;
mod watermark;

pub use image_watermark::*;
pub use pattern_overlay::*;
pub use security_watermark::*;
pub use stamp_overlay::*;
pub use types::*;
pub use watermark::*;

#[cfg(test)]
mod tests;
