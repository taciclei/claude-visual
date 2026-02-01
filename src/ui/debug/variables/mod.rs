//! Variables View
//!
//! UI for inspecting variables during debugging.

mod events;
mod types;
mod view;

pub use events::VariablesViewEvent;
pub use types::{ScopeItem, VariableItem};
pub use view::VariablesView;
