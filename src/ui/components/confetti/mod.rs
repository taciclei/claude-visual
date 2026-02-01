//! Confetti and celebration animation components
//!
//! Provides particle effects, confetti bursts, and celebration animations.

mod types;
mod confetti;
mod emoji_burst;
mod sparkle;
mod firework;
mod balloon;
mod party_popper;

pub use types::*;
pub use confetti::*;
pub use emoji_burst::*;
pub use sparkle::*;
pub use firework::*;
pub use balloon::*;
pub use party_popper::*;

#[cfg(test)]
mod tests;
