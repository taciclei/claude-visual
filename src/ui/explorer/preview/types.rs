//! Types and constants for file preview

use std::path::PathBuf;

/// Maximum file size to preview (1MB)
pub(crate) const MAX_PREVIEW_SIZE: u64 = 1024 * 1024;

/// Maximum lines to show in preview
pub(crate) const MAX_PREVIEW_LINES: usize = 50;

/// Preview state
#[derive(Clone, Debug)]
pub enum PreviewState {
    /// No file selected
    Empty,
    /// Loading file content
    Loading(PathBuf),
    /// File loaded successfully
    Loaded {
        path: PathBuf,
        content: String,
        line_count: usize,
        file_size: u64,
        language: Option<String>,
    },
    /// File is binary
    Binary {
        path: PathBuf,
        file_size: u64,
    },
    /// File too large
    TooLarge {
        path: PathBuf,
        file_size: u64,
    },
    /// Failed to load
    Error {
        path: PathBuf,
        message: String,
    },
}

/// Events emitted by the preview panel
pub enum FilePreviewEvent {
    /// Open the file in editor
    OpenFile(PathBuf),
    /// Add to context
    AddToContext(PathBuf),
    /// Close preview
    Close,
}
