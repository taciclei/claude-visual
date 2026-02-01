//! Context file types

/// A recently accessed file for quick access
#[derive(Debug, Clone)]
pub struct RecentFile {
    /// File path
    pub path: String,
    /// File name (for display)
    pub name: String,
    /// When it was last accessed
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    /// File type for icon
    pub file_type: ContextFileType,
    /// Number of times accessed
    pub access_count: usize,
}

impl RecentFile {
    /// Create from a file path
    pub fn from_path(path: impl Into<String>) -> Self {
        let path = path.into();
        let name = std::path::Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        let ext = std::path::Path::new(&path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_type = ContextFileType::from_extension(&ext);

        Self {
            path,
            name,
            accessed_at: chrono::Utc::now(),
            file_type,
            access_count: 1,
        }
    }

    /// Update access timestamp
    pub fn touch(&mut self) {
        self.accessed_at = chrono::Utc::now();
        self.access_count += 1;
    }
}

/// A file in the current context
#[derive(Debug, Clone)]
pub struct ContextFile {
    /// File path
    pub path: String,
    /// File name (for display)
    pub name: String,
    /// Estimated token count
    pub tokens: u64,
    /// When it was added
    pub added_at: std::time::Instant,
    /// File type for icon
    pub file_type: ContextFileType,
}

/// Type of file in context
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextFileType {
    Code,
    Markdown,
    Config,
    Data,
    Image,
    Other,
}

impl ContextFileType {
    /// Get icon for file type
    pub fn icon(&self) -> &'static str {
        match self {
            ContextFileType::Code => "📄",
            ContextFileType::Markdown => "📝",
            ContextFileType::Config => "⚙️",
            ContextFileType::Data => "📊",
            ContextFileType::Image => "🖼️",
            ContextFileType::Other => "📁",
        }
    }

    /// Detect file type from extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" | "py" | "js" | "ts" | "tsx" | "jsx" | "go" | "java" | "c" | "cpp" | "h" | "hpp" | "rb" | "swift" | "kt" => ContextFileType::Code,
            "md" | "mdx" | "markdown" | "txt" => ContextFileType::Markdown,
            "json" | "yaml" | "yml" | "toml" | "ini" | "env" | "config" => ContextFileType::Config,
            "csv" | "sql" | "db" => ContextFileType::Data,
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => ContextFileType::Image,
            _ => ContextFileType::Other,
        }
    }
}

impl ContextFile {
    /// Create from a file path
    pub fn from_path(path: impl Into<String>) -> Self {
        let path = path.into();
        let name = std::path::Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        let ext = std::path::Path::new(&path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_type = ContextFileType::from_extension(&ext);

        Self {
            path,
            name,
            tokens: 0, // Will be estimated later
            added_at: std::time::Instant::now(),
            file_type,
        }
    }
}
