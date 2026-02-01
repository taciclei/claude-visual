//! Display-related methods for FileEntry

use super::entry::FileEntry;

impl FileEntry {
    /// Get icon for file type
    pub fn icon(&self) -> &'static str {
        if self.is_directory() {
            if self.is_expanded {
                "📂"
            } else {
                "📁"
            }
        } else {
            match self.extension() {
                // Programming languages
                Some("rs") => "🦀",
                Some("js" | "mjs") => "📜",
                Some("ts" | "tsx") => "📘",
                Some("jsx") => "⚛️",
                Some("py") => "🐍",
                Some("go") => "🐹",
                Some("java") => "☕",
                Some("c" | "h") => "🔧",
                Some("cpp" | "cc" | "hpp") => "⚙️",
                Some("rb") => "💎",
                Some("php") => "🐘",
                Some("swift") => "🐦",
                Some("kt") => "🎯",
                // Web
                Some("html" | "htm") => "🌐",
                Some("css" | "scss" | "sass" | "less") => "🎨",
                Some("vue") => "💚",
                Some("svelte") => "🔥",
                // Config
                Some("json") => "📋",
                Some("yaml" | "yml") => "📑",
                Some("toml") => "⚙️",
                Some("xml") => "📰",
                Some("ini" | "cfg" | "conf") => "🔧",
                // Documents
                Some("md" | "markdown") => "📝",
                Some("txt") => "📄",
                Some("pdf") => "📕",
                Some("doc" | "docx") => "📘",
                // Images
                Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico") => "🖼️",
                // Media
                Some("mp3" | "wav" | "ogg" | "flac") => "🎵",
                Some("mp4" | "avi" | "mkv" | "mov" | "webm") => "🎬",
                // Archives
                Some("zip" | "tar" | "gz" | "rar" | "7z") => "📦",
                // Data
                Some("sql" | "db" | "sqlite") => "🗃️",
                Some("csv") => "📊",
                // Scripts
                Some("sh" | "bash" | "zsh") => "🐚",
                Some("ps1" | "bat" | "cmd") => "⌨️",
                // Lock files
                Some("lock") => "🔒",
                // Git
                Some("gitignore" | "gitattributes") => "🔧",
                // Docker
                Some("dockerfile") => "🐳",
                // Default
                _ => "📄",
            }
        }
    }

    /// Format file size
    pub fn formatted_size(&self) -> Option<String> {
        self.size.map(|size| {
            if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.1} KB", size as f64 / 1024.0)
            } else if size < 1024 * 1024 * 1024 {
                format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
            }
        })
    }
}
