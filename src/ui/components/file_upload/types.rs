//! Shared types for file upload components

/// Upload state
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum UploadState {
    /// Idle, ready for upload
    #[default]
    Idle,
    /// Dragging file over dropzone
    DragOver,
    /// Upload in progress
    Uploading,
    /// Upload successful
    Success,
    /// Upload failed
    Error,
}

/// File upload size limits
#[derive(Debug, Clone, Copy)]
pub struct FileSizeLimit {
    /// Max file size in bytes
    pub max_bytes: u64,
    /// Human readable size
    pub display: &'static str,
}

impl FileSizeLimit {
    pub const fn kb(kb: u64) -> Self {
        Self {
            max_bytes: kb * 1024,
            display: "KB",
        }
    }

    pub const fn mb(mb: u64) -> Self {
        Self {
            max_bytes: mb * 1024 * 1024,
            display: "MB",
        }
    }

    pub const fn gb(gb: u64) -> Self {
        Self {
            max_bytes: gb * 1024 * 1024 * 1024,
            display: "GB",
        }
    }
}

impl Default for FileSizeLimit {
    fn default() -> Self {
        Self::mb(10)
    }
}

/// Events emitted by file upload components
#[derive(Debug, Clone)]
pub enum FileUploadEvent {
    /// Files selected
    FilesSelected(Vec<String>),
    /// File dropped
    FileDropped(String),
    /// Upload started
    UploadStarted,
    /// Upload progress
    UploadProgress { filename: String, progress: f32 },
    /// Upload completed
    UploadCompleted(String),
    /// Upload failed
    UploadFailed { filename: String, error: String },
    /// File removed
    FileRemoved(String),
}
