//! Settings UI module
//!
//! Visual editor for application settings.

mod components;
mod core;
mod dialogs;
mod render;
mod tabs;
mod types;

pub use core::SettingsModal;
pub use types::{SettingsModalEvent, SettingsTab};
