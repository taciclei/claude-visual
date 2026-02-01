//! Call Stack View
//!
//! UI for displaying the call stack during debugging.

mod types;
mod component;
mod render;

pub use types::{CallStackViewEvent, StackFrameItem, ThreadItem};
pub use component::CallStackView;
