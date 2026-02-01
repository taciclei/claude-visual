//! Code Actions Panel
//!
//! UI for displaying and selecting code actions (quick fixes, refactorings).

mod core;
mod render;
mod types;

pub use types::{CodeActionIndicator, CodeActionItem, CodeActionKind, CodeActionsEvent};

pub use core::CodeActionsPanel;
