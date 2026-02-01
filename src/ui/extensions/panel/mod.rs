//! Extensions panel for browsing and managing extensions
//!
//! This panel displays installed extensions and allows users to
//! enable, disable, and configure them.

mod available_tab;
mod installed_tab;
mod panel;
mod render_item;
mod render_tabs;
mod types;
mod updates_tab;

pub use panel::*;
pub use types::*;
