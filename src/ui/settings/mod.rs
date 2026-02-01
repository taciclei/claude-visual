//! Settings UI module
//!
//! Visual editor for application settings.

mod types;
mod core;
mod render;
mod tabs;
mod components;
mod dialogs;

pub use types::{SettingsTab, SettingsModalEvent};
pub use core::SettingsModal;
