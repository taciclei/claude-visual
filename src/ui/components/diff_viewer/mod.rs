//! Diff viewer components for displaying file changes
//!
//! Provides unified diff, split diff, and inline change visualization.

mod types;
mod unified;
mod split;
mod inline;
mod stats;
#[cfg(test)]
mod tests;

pub use types::*;
pub use unified::*;
pub use split::*;
pub use inline::*;
pub use stats::*;
