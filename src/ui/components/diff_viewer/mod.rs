//! Diff viewer components for displaying file changes
//!
//! Provides unified diff, split diff, and inline change visualization.

mod inline;
mod split;
mod stats;
#[cfg(test)]
mod tests;
mod types;
mod unified;

pub use inline::*;
pub use split::*;
pub use stats::*;
pub use types::*;
pub use unified::*;
