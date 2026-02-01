//! Context Window Indicator
//!
//! Shows token usage and context window status in the chat header.

mod colors;
mod usage;
mod events;
mod state;
mod view;

pub use colors::{SimpleColors, default_colors};
pub use usage::ContextUsage;
pub use events::ContextIndicatorEvent;
pub use state::ContextIndicator;
