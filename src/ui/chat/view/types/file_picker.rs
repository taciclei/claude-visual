//! File picker types

/// File picker item
#[derive(Debug, Clone)]
pub struct FilePickerItem {
    /// File path relative to project root
    pub path: String,
    /// File name
    pub name: String,
    /// Whether it's a directory
    pub is_dir: bool,
    /// File extension (for icon selection)
    pub extension: Option<String>,
    /// File size in bytes
    pub size: Option<u64>,
    /// Last modified time
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
}

impl FilePickerItem {
    /// Get icon for this file type
    pub fn icon(&self) -> &'static str {
        if self.is_dir {
            return "📁";
        }
        match self.extension.as_deref() {
            Some("rs") => "🦀",
            Some("js") | Some("jsx") => "🟨",
            Some("ts") | Some("tsx") => "🔷",
            Some("py") => "🐍",
            Some("go") => "🐹",
            Some("rb") => "💎",
            Some("java") | Some("kt") => "☕",
            Some("c") | Some("cpp") | Some("h") | Some("hpp") => "⚙️",
            Some("md") | Some("mdx") => "📝",
            Some("json") => "📋",
            Some("yaml") | Some("yml") => "⚙️",
            Some("toml") => "⚙️",
            Some("html") => "🌐",
            Some("css") | Some("scss") | Some("sass") => "🎨",
            Some("sql") => "🗃️",
            Some("sh") | Some("bash") | Some("zsh") => "💻",
            Some("dockerfile") => "🐳",
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => "🖼️",
            _ => "📄",
        }
    }
}
