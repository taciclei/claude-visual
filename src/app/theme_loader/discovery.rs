//! Theme discovery functionality

use std::path::Path;

use super::types::{PartialTheme, ThemeFormat, ThemeLoadError, ThemeMetadata};

/// Discover themes synchronously (for use in blocking context)
pub(crate) fn discover_themes_sync(themes_dir: &Path) -> Result<Vec<ThemeMetadata>, ThemeLoadError> {
    let mut themes = Vec::new();

    if !themes_dir.exists() {
        return Ok(themes);
    }

    let entries = std::fs::read_dir(themes_dir)
        .map_err(|e| ThemeLoadError::ReadError(e.to_string()))?;

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Check if it's a theme file
        let format = match ThemeFormat::from_path(&path) {
            Some(f) => f,
            None => continue,
        };

        // Try to read metadata
        if let Ok(meta) = read_theme_metadata(&path, format) {
            themes.push(meta);
        }
    }

    // Sort by name
    themes.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(themes)
}

/// Read theme metadata without full parsing
pub(crate) fn read_theme_metadata(path: &Path, format: ThemeFormat) -> Result<ThemeMetadata, ThemeLoadError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ThemeLoadError::ReadError(e.to_string()))?;

    // Parse just the metadata fields
    let partial: PartialTheme = match format {
        ThemeFormat::Json => serde_json::from_str(&content)
            .map_err(|e| ThemeLoadError::ParseError(e.to_string()))?,
        ThemeFormat::Toml => toml::from_str(&content)
            .map_err(|e| ThemeLoadError::ParseError(e.to_string()))?,
    };

    Ok(ThemeMetadata {
        name: partial.name.unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        }),
        path: path.to_path_buf(),
        is_dark: partial.is_dark.unwrap_or(true),
        author: partial.author,
        version: partial.version,
        description: partial.description,
    })
}
