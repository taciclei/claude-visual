//! Context Window Indicator
//!
//! Shows token usage and context window status in the chat header.

mod colors;
mod events;
mod state;
mod usage;
mod view;

pub use colors::{default_colors, SimpleColors};
pub use events::ContextIndicatorEvent;
pub use state::ContextIndicator;
pub use usage::ContextUsage;
