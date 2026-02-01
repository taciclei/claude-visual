//! Call Stack View
//!
//! UI for displaying the call stack during debugging.

mod component;
mod render;
mod types;

pub use component::CallStackView;
pub use types::{CallStackViewEvent, StackFrameItem, ThreadItem};
