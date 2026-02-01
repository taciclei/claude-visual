//! File loading logic for preview

use std::path::PathBuf;
use super::types::{PreviewState, MAX_PREVIEW_SIZE, MAX_PREVIEW_LINES};

/// Load file preview content
pub(crate) fn load_file_preview(path: &PathBuf) -> PreviewState {
    // Get file metadata
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            return PreviewState::Error {
                path: path.clone(),
                message: format!("Cannot read file: {}", e),
            }
        }
    };

    // Check if directory
    if metadata.is_dir() {
        return PreviewState::Error {
            path: path.clone(),
            message: "Cannot preview directories".to_string(),
        };
    }

    let file_size = metadata.len();

    // Check file size
    if file_size > MAX_PREVIEW_SIZE {
        return PreviewState::TooLarge {
            path: path.clone(),
            file_size,
        };
    }

    // Read file content
    let content = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return PreviewState::Error {
                path: path.clone(),
                message: format!("Cannot read file: {}", e),
            }
        }
    };

    // Check if binary (contains null bytes or high ratio of non-text)
    if is_binary(&content) {
        return PreviewState::Binary {
            path: path.clone(),
            file_size,
        };
    }

    // Convert to string
    let text = String::from_utf8_lossy(&content);

    // Truncate to max lines
    let lines: Vec<&str> = text.lines().take(MAX_PREVIEW_LINES).collect();
    let truncated = text.lines().count() > MAX_PREVIEW_LINES;
    let line_count = text.lines().count();

    let preview_content = if truncated {
        format!("{}\n...(truncated)", lines.join("\n"))
    } else {
        lines.join("\n")
    };

    // Detect language from extension
    let language = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| detect_language(ext));

    PreviewState::Loaded {
        path: path.clone(),
        content: preview_content,
        line_count,
        file_size,
        language,
    }
}

/// Check if content is binary
pub(crate) fn is_binary(content: &[u8]) -> bool {
    // Check for null bytes
    if content.contains(&0) {
        return true;
    }

    // Check ratio of non-printable characters
    let non_text = content
        .iter()
        .filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13)
        .count();

    non_text as f64 / content.len() as f64 > 0.1
}

/// Detect language from file extension
pub(crate) fn detect_language(ext: &str) -> String {
    match ext.to_lowercase().as_str() {
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "tsx" => "typescript",
        "jsx" => "javascript",
        "py" => "python",
        "rb" => "ruby",
        "go" => "go",
        "java" => "java",
        "kt" => "kotlin",
        "swift" => "swift",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "hh" => "cpp",
        "cs" => "csharp",
        "php" => "php",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" | "sass" => "scss",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "xml" => "xml",
        "md" | "markdown" => "markdown",
        "sh" | "bash" | "zsh" => "bash",
        "sql" => "sql",
        "dockerfile" => "dockerfile",
        _ => "text",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_binary() {
        assert!(is_binary(&[0, 1, 2, 3]));
        assert!(!is_binary(b"Hello, World!"));
        assert!(!is_binary(b"Line 1\nLine 2\r\nLine 3"));
    }

    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language("rs"), "rust");
        assert_eq!(detect_language("js"), "javascript");
        assert_eq!(detect_language("py"), "python");
        assert_eq!(detect_language("unknown"), "text");
    }
}
