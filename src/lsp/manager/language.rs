//! Language detection and configuration

/// Supported language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Go,
    Json,
    Toml,
    Markdown,
}

impl Language {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "ts" | "tsx" => Some(Self::TypeScript),
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "py" | "pyi" => Some(Self::Python),
            "go" => Some(Self::Go),
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            "md" | "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    /// Get language ID for LSP
    pub fn language_id(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::Go => "go",
            Self::Json => "json",
            Self::Toml => "toml",
            Self::Markdown => "markdown",
        }
    }

    /// Get default server command
    pub fn server_command(&self) -> Option<&'static str> {
        match self {
            Self::Rust => Some("rust-analyzer"),
            Self::TypeScript | Self::JavaScript => Some("typescript-language-server"),
            Self::Python => Some("pyright-langserver"),
            Self::Go => Some("gopls"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
        assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension("py"), Some(Language::Python));
        assert_eq!(Language::from_extension("unknown"), None);
    }

    #[test]
    fn test_language_id() {
        assert_eq!(Language::Rust.language_id(), "rust");
        assert_eq!(Language::TypeScript.language_id(), "typescript");
    }
}
