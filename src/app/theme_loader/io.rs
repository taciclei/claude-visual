//! Theme I/O operations

use std::path::Path;

use crate::app::theme::Theme;

use super::types::{ThemeFormat, ThemeLoadError, ThemeLoadResult};

/// Load a theme from path asynchronously
pub(crate) async fn load_theme_from_path(path: &Path) -> ThemeLoadResult {
    let path = path.to_path_buf();

    tokio::task::spawn_blocking(move || load_theme_sync(&path))
        .await
        .map_err(|e| ThemeLoadError::ReadError(e.to_string()))?
}

/// Load a theme synchronously
fn load_theme_sync(path: &Path) -> ThemeLoadResult {
    let format = ThemeFormat::from_path(path)
        .ok_or_else(|| ThemeLoadError::InvalidFormat("Unknown file format".to_string()))?;

    let content =
        std::fs::read_to_string(path).map_err(|e| ThemeLoadError::ReadError(e.to_string()))?;

    let theme: Theme = match format {
        ThemeFormat::Json => {
            serde_json::from_str(&content).map_err(|e| ThemeLoadError::ParseError(e.to_string()))?
        }
        ThemeFormat::Toml => {
            toml::from_str(&content).map_err(|e| ThemeLoadError::ParseError(e.to_string()))?
        }
    };

    Ok(theme)
}
