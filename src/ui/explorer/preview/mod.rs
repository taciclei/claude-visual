//! File Preview Panel
//!
//! Component for previewing file contents on hover or selection in the file explorer.

mod types;
mod core;
mod loader;
mod render;

// Re-export public types
pub use types::{PreviewState, FilePreviewEvent};
pub use core::FilePreviewPanel;
