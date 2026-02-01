//! LSP UI Components
//!
//! User interface components for Language Server Protocol features.

pub mod code_actions;
pub mod completion;
pub mod diagnostics;
pub mod hover;
pub mod status;

pub use code_actions::{CodeActionsPanel, CodeActionItem, CodeActionKind, CodeActionsEvent};
pub use completion::{CompletionDropdown, CompletionDropdownEvent};
pub use diagnostics::DiagnosticsPanel;
pub use hover::HoverPanel;
pub use status::LspStatusBar;
