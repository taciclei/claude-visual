//! Theme loading types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::app::theme::Theme;

/// Theme loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeLoadState {
    /// Not started loading
    NotStarted,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Failed to load
    Failed,
}

/// Theme metadata (for listing without full load)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// Theme name
    pub name: String,
    /// Theme file path
    pub path: PathBuf,
    /// Is this a dark theme
    pub is_dark: bool,
    /// Theme author
    pub author: Option<String>,
    /// Theme version
    pub version: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// Theme file format (JSON/TOML)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeFormat {
    Json,
    Toml,
}

impl ThemeFormat {
    /// Detect format from file extension
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Toml => "toml",
        }
    }
}

/// Theme load result
pub type ThemeLoadResult = Result<Theme, ThemeLoadError>;

/// Theme loading error
#[derive(Debug, Clone, thiserror::Error)]
pub enum ThemeLoadError {
    #[error("Theme file not found: {0}")]
    NotFound(String),
    #[error("Failed to read theme file: {0}")]
    ReadError(String),
    #[error("Failed to parse theme: {0}")]
    ParseError(String),
    #[error("Invalid theme format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Result of preloading themes
#[derive(Debug)]
pub struct PreloadResult {
    /// Number of themes loaded
    pub loaded: usize,
    /// Errors encountered
    pub errors: Vec<(String, ThemeLoadError)>,
}

impl PreloadResult {
    /// Check if all themes loaded successfully
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Partial theme for metadata extraction
#[derive(Debug, Deserialize)]
pub(crate) struct PartialTheme {
    pub(crate) name: Option<String>,
    pub(crate) is_dark: Option<bool>,
    pub(crate) author: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) description: Option<String>,
}

/// Load callback for async notification
pub type ThemeLoadCallback = Box<dyn FnOnce(ThemeLoadResult) + Send + 'static>;
