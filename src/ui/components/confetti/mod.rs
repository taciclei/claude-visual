//! Confetti and celebration animation components
//!
//! Provides particle effects, confetti bursts, and celebration animations.

mod balloon;
mod confetti;
mod emoji_burst;
mod firework;
mod party_popper;
mod sparkle;
mod types;

pub use balloon::*;
pub use confetti::*;
pub use emoji_burst::*;
pub use firework::*;
pub use party_popper::*;
pub use sparkle::*;
pub use types::*;

#[cfg(test)]
mod tests;
