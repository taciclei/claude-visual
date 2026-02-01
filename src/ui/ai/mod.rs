//! AI UI Module
//!
//! UI components for AI provider selection and context management.

pub mod context_indicator;
pub mod context_panel;
pub mod image_input;
pub mod model_selector;

pub use context_indicator::{ContextIndicator, ContextIndicatorEvent, ContextUsage};
pub use context_panel::{ContextPanel, ContextPanelEvent};
pub use image_input::{ImageAttachment, ImageInput, ImageInputEvent};
pub use model_selector::{ModelSelector, ModelSelectorEvent};
