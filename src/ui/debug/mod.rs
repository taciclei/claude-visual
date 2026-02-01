//! Debug UI Module
//!
//! User interface components for debugging integration.

mod breakpoints;
pub mod call_stack;
mod debug_panel;
mod variables;
mod watch;

pub use breakpoints::{BreakpointItem, BreakpointsList, BreakpointsListEvent};
pub use call_stack::{CallStackView, CallStackViewEvent, StackFrameItem, ThreadItem};
pub use debug_panel::{DebugContext, DebugPanel, DebugPanelEvent, DebugPromptType, DebugTab};
pub use variables::{ScopeItem, VariableItem, VariablesView, VariablesViewEvent};
pub use watch::{WatchChild, WatchExpression, WatchView, WatchViewEvent};
