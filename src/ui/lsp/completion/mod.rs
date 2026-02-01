//! Completion Dropdown
//!
//! Autocomplete dropdown for LSP completion suggestions.

mod dropdown;
mod render;
mod types;

pub use dropdown::CompletionDropdown;
pub use types::CompletionDropdownEvent;
