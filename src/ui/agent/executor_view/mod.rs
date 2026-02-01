//! Executor View UI Component
//!
//! Displays the agent executor status with controls for pause/resume/cancel.

mod approval;
mod component;
mod controls;
mod events;
mod render;

pub use component::ExecutorView;
pub use events::ExecutorViewEvent;
