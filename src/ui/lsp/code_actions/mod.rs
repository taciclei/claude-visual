//! Code Actions Panel
//!
//! UI for displaying and selecting code actions (quick fixes, refactorings).

mod types;
mod core;
mod render;

pub use types::{
    CodeActionKind,
    CodeActionItem,
    CodeActionsEvent,
    CodeActionIndicator,
};

pub use core::CodeActionsPanel;
