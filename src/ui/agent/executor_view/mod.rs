//! Executor View UI Component
//!
//! Displays the agent executor status with controls for pause/resume/cancel.

mod events;
mod component;
mod controls;
mod approval;
mod render;

pub use events::ExecutorViewEvent;
pub use component::ExecutorView;
