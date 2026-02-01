//! Type definitions for Zed-compatible theme files

use serde::Deserialize;
use std::collections::HashMap;

/// Zed theme file structure
#[derive(Debug, Deserialize)]
pub struct ZedThemeFile {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub name: String,
    pub author: Option<String>,
    pub themes: Vec<ZedThemeVariant>,
}

/// A single theme variant (e.g., "Dark", "Light")
#[derive(Debug, Deserialize)]
pub struct ZedThemeVariant {
    pub name: String,
    pub appearance: String,
    pub style: ZedThemeStyle,
}

/// Theme style definitions
#[derive(Debug, Deserialize)]
pub struct ZedThemeStyle {
    // Background colors
    pub background: Option<String>,
    #[serde(rename = "editor.background")]
    pub editor_background: Option<String>,
    #[serde(rename = "element.background")]
    pub element_background: Option<String>,
    #[serde(rename = "element.hover")]
    pub element_hover: Option<String>,
    #[serde(rename = "element.active")]
    pub element_active: Option<String>,
    #[serde(rename = "element.selected")]
    pub element_selected: Option<String>,

    // Border colors
    pub border: Option<String>,
    #[serde(rename = "border.variant")]
    pub border_variant: Option<String>,
    #[serde(rename = "border.focused")]
    pub border_focused: Option<String>,

    // Text colors
    pub text: Option<String>,
    #[serde(rename = "text.muted")]
    pub text_muted: Option<String>,
    #[serde(rename = "text.placeholder")]
    pub text_placeholder: Option<String>,
    #[serde(rename = "text.disabled")]
    pub text_disabled: Option<String>,
    #[serde(rename = "text.accent")]
    pub text_accent: Option<String>,

    // Status colors
    #[serde(rename = "status.success")]
    pub status_success: Option<String>,
    #[serde(rename = "status.warning")]
    pub status_warning: Option<String>,
    #[serde(rename = "status.error")]
    pub status_error: Option<String>,
    #[serde(rename = "status.info")]
    pub status_info: Option<String>,

    // Syntax highlighting
    pub syntax: Option<HashMap<String, SyntaxStyle>>,

    // Players (cursors, selections)
    pub players: Option<Vec<PlayerStyle>>,
}

/// Syntax style for a token type
#[derive(Debug, Deserialize)]
pub struct SyntaxStyle {
    pub color: Option<String>,
    pub font_style: Option<String>,
    pub font_weight: Option<i32>,
}

/// Player style (for collaborative editing)
#[derive(Debug, Deserialize)]
pub struct PlayerStyle {
    pub cursor: Option<String>,
    pub selection: Option<String>,
    pub background: Option<String>,
}

/// Theme metadata for loaded themes
#[derive(Debug, Clone)]
pub struct ThemeMetadata {
    /// Theme name
    pub name: String,
    /// Extension ID (None if built-in)
    pub extension_id: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// File path
    pub path: Option<std::path::PathBuf>,
}
