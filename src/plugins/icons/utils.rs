//! Utility functions for icon themes

use super::types::*;
use gpui::Hsla;
use std::collections::HashMap;
use std::path::PathBuf;

/// Parse a hex color string to HSLA
pub(crate) fn parse_hex_color(hex: &str) -> Option<Hsla> {
    let hex = hex.trim_start_matches('#');

    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };

    // Convert RGB to HSL
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let a = a as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f32::EPSILON {
        return Some(Hsla { h: 0.0, s: 0.0, l, a });
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    Some(Hsla {
        h: h / 6.0,
        s,
        l,
        a,
    })
}

/// Built-in default icon theme with fallback icons
pub fn default_icon_theme() -> IconTheme {
    let mut icon_definitions = HashMap::new();

    // Default icons would be embedded or point to built-in resources
    // For now, we just provide empty paths as placeholders
    icon_definitions.insert(
        "_file".to_string(),
        IconDefinition {
            path: PathBuf::from("icons/file.svg"),
            color: None,
        },
    );
    icon_definitions.insert(
        "_folder".to_string(),
        IconDefinition {
            path: PathBuf::from("icons/folder.svg"),
            color: None,
        },
    );
    icon_definitions.insert(
        "_folder_open".to_string(),
        IconDefinition {
            path: PathBuf::from("icons/folder-open.svg"),
            color: None,
        },
    );

    let manifest = IconThemeManifest {
        id: "default".to_string(),
        label: "Default".to_string(),
        icon_definitions,
        file_icons: FileIcons {
            file: Some("_file".to_string()),
            folder: Some("_folder".to_string()),
            folder_expanded: Some("_folder_open".to_string()),
            ..Default::default()
        },
        ui_icons: HashMap::new(),
        light: None,
        high_contrast: None,
    };

    IconTheme {
        metadata: IconThemeMetadata {
            id: "default".to_string(),
            name: "Default".to_string(),
            extension_id: None,
            path: None,
        },
        manifest,
        base_path: PathBuf::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let color = parse_hex_color("#ff0000").unwrap();
        assert!((color.h - 0.0).abs() < 0.01);
        assert!((color.s - 1.0).abs() < 0.01);
        assert!((color.l - 0.5).abs() < 0.01);

        let color = parse_hex_color("00ff00").unwrap();
        assert!((color.h - 0.333).abs() < 0.01);

        let color = parse_hex_color("#0000ff80").unwrap();
        assert!((color.a - 0.5).abs() < 0.02);
    }

    #[test]
    fn test_default_icon_theme() {
        let theme = default_icon_theme();
        assert_eq!(theme.metadata.id, "default");
        assert!(theme.manifest.file_icons.file.is_some());
        assert!(theme.manifest.file_icons.folder.is_some());
    }
}
