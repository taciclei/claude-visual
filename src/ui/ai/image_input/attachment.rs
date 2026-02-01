//! Image attachment data structure and utilities

use std::path::PathBuf;

/// An attached image
#[derive(Debug, Clone)]
pub struct ImageAttachment {
    /// File path (if from disk)
    pub path: Option<PathBuf>,
    /// File name for display
    pub name: String,
    /// Image data (base64 encoded)
    pub data: Option<String>,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size: usize,
    /// Thumbnail data (smaller base64 for preview)
    pub thumbnail: Option<String>,
}

impl ImageAttachment {
    /// Create from a file path
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy().to_string();
        let ext = path.extension()?.to_string_lossy().to_lowercase();

        let mime_type = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => return None, // Not a supported image
        }
        .to_string();

        // Get file size
        let metadata = std::fs::metadata(&path).ok()?;
        let size = metadata.len() as usize;

        Some(Self {
            path: Some(path),
            name,
            data: None, // Loaded on demand
            mime_type,
            size,
            thumbnail: None,
        })
    }

    /// Load image data from disk
    pub fn load_data(&mut self) -> Result<(), String> {
        if let Some(path) = &self.path {
            let data = std::fs::read(path).map_err(|e| e.to_string())?;
            self.data = Some(base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &data,
            ));
        }
        Ok(())
    }

    /// Format size for display
    pub fn format_size(&self) -> String {
        if self.size >= 1_000_000 {
            format!("{:.1} MB", self.size as f64 / 1_000_000.0)
        } else if self.size >= 1_000 {
            format!("{:.1} KB", self.size as f64 / 1_000.0)
        } else {
            format!("{} B", self.size)
        }
    }

    /// Check if image is too large (>20MB typical limit)
    pub fn is_too_large(&self) -> bool {
        self.size > 20_000_000
    }
}
