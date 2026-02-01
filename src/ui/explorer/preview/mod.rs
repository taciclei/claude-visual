//! File Preview Panel
//!
//! Component for previewing file contents on hover or selection in the file explorer.

mod core;
mod loader;
mod render;
mod types;

// Re-export public types
pub use core::FilePreviewPanel;
pub use types::{FilePreviewEvent, PreviewState};
