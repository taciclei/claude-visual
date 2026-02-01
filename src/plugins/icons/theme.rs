//! Icon theme implementation

use super::types::*;
use super::utils::parse_hex_color;
use gpui::Hsla;
use std::path::{Path, PathBuf};

impl IconTheme {
    /// Get the icon path for a file
    pub fn get_file_icon(&self, filename: &str) -> Option<PathBuf> {
        // Try exact filename match
        if let Some(icon_id) = self.manifest.file_icons.file_names.get(filename) {
            return self.resolve_icon(icon_id);
        }

        // Try extension match
        if let Some(ext) = Path::new(filename).extension().and_then(|e| e.to_str()) {
            if let Some(icon_id) = self.manifest.file_icons.file_extensions.get(ext) {
                return self.resolve_icon(icon_id);
            }
        }

        // Fall back to default file icon
        self.manifest
            .file_icons
            .file
            .as_ref()
            .and_then(|id| self.resolve_icon(id))
    }

    /// Get the icon path for a folder
    pub fn get_folder_icon(&self, folder_name: &str, expanded: bool) -> Option<PathBuf> {
        // Try exact folder name match
        if expanded {
            if let Some(icon_id) = self.manifest.file_icons.folder_names_expanded.get(folder_name) {
                return self.resolve_icon(icon_id);
            }
        }

        if let Some(icon_id) = self.manifest.file_icons.folder_names.get(folder_name) {
            return self.resolve_icon(icon_id);
        }

        // Fall back to default folder icon
        let icon_id = if expanded {
            self.manifest.file_icons.folder_expanded.as_ref()
        } else {
            self.manifest.file_icons.folder.as_ref()
        };

        icon_id.and_then(|id| self.resolve_icon(id))
    }

    /// Get the icon path for a UI element
    pub fn get_ui_icon(&self, kind: UiIconKind) -> Option<PathBuf> {
        let name = kind.default_name();
        self.manifest
            .ui_icons
            .get(name)
            .and_then(|id| self.resolve_icon(id))
    }

    /// Get the icon path for a language
    pub fn get_language_icon(&self, language_id: &str) -> Option<PathBuf> {
        self.manifest
            .file_icons
            .language_ids
            .get(language_id)
            .and_then(|id| self.resolve_icon(id))
    }

    /// Resolve an icon ID to a full path
    fn resolve_icon(&self, icon_id: &str) -> Option<PathBuf> {
        self.manifest.icon_definitions.get(icon_id).map(|def| {
            if def.path.is_absolute() {
                def.path.clone()
            } else {
                self.base_path.join(&def.path)
            }
        })
    }

    /// Get icon color if specified
    pub fn get_icon_color(&self, icon_id: &str) -> Option<Hsla> {
        self.manifest
            .icon_definitions
            .get(icon_id)
            .and_then(|def| def.color.as_ref())
            .and_then(|hex| parse_hex_color(hex))
    }

    /// Get metadata reference
    pub fn metadata(&self) -> &IconThemeMetadata {
        &self.metadata
    }

    /// Get manifest reference
    pub fn manifest(&self) -> &IconThemeManifest {
        &self.manifest
    }
}
