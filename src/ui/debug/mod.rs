//! Debug UI Module
//!
//! User interface components for debugging integration.

mod debug_panel;
mod breakpoints;
mod variables;
pub mod call_stack;
mod watch;

pub use debug_panel::{DebugContext, DebugPanel, DebugPanelEvent, DebugPromptType, DebugTab};
pub use breakpoints::{BreakpointsList, BreakpointItem, BreakpointsListEvent};
pub use variables::{VariablesView, VariableItem, ScopeItem, VariablesViewEvent};
pub use call_stack::{CallStackView, StackFrameItem, ThreadItem, CallStackViewEvent};
pub use watch::{WatchView, WatchExpression, WatchChild, WatchViewEvent};
