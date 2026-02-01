//! Completion Dropdown
//!
//! Autocomplete dropdown for LSP completion suggestions.

mod types;
mod dropdown;
mod render;

pub use types::CompletionDropdownEvent;
pub use dropdown::CompletionDropdown;
