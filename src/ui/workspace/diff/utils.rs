//! Utility functions for diff processing

use crate::ui::workspace::core::Workspace;

impl Workspace {
    /// Get language from file extension for syntax highlighting
    pub(in crate::ui::workspace) fn detect_language_from_path(&self, path: &str) -> Option<&'static str> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())?;

        match ext.to_lowercase().as_str() {
            "rs" => Some("rust"),
            "js" | "mjs" | "cjs" => Some("javascript"),
            "ts" | "mts" | "cts" => Some("typescript"),
            "tsx" => Some("tsx"),
            "jsx" => Some("javascript"),
            "py" | "pyw" => Some("python"),
            "json" => Some("json"),
            "toml" => Some("toml"),
            "sh" | "bash" | "zsh" => Some("bash"),
            "yaml" | "yml" => Some("yaml"),
            "md" | "markdown" => Some("markdown"),
            "html" | "htm" => Some("html"),
            "css" | "scss" | "sass" => Some("css"),
            "go" => Some("go"),
            "rb" => Some("ruby"),
            "c" | "h" => Some("c"),
            "cpp" | "cc" | "cxx" | "hpp" => Some("cpp"),
            _ => None,
        }
    }

    /// Prepare lines for side-by-side display
    pub(in crate::ui::workspace) fn prepare_side_by_side_lines(&self, lines: &[&str]) -> Vec<(String, String)> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            if line.starts_with('-') {
                // Check if next line is an addition (change pair)
                if i + 1 < lines.len() && lines[i + 1].starts_with('+') {
                    result.push((line.to_string(), lines[i + 1].to_string()));
                    i += 2;
                    continue;
                }
                // Deletion only
                result.push((line.to_string(), String::new()));
            } else if line.starts_with('+') {
                // Addition only (no matching deletion before)
                result.push((String::new(), line.to_string()));
            } else {
                // Context line - same on both sides
                result.push((line.to_string(), line.to_string()));
            }
            i += 1;
        }

        result
    }
}
