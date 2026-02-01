//! Variables View
//!
//! UI for inspecting variables during debugging.

mod types;
mod events;
mod view;

pub use types::{VariableItem, ScopeItem};
pub use events::VariablesViewEvent;
pub use view::VariablesView;
