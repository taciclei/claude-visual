//! Extensions panel for browsing and managing extensions
//!
//! This panel displays installed extensions and allows users to
//! enable, disable, and configure them.

mod types;
mod panel;
mod render_item;
mod render_tabs;
mod installed_tab;
mod available_tab;
mod updates_tab;

pub use types::*;
pub use panel::*;
