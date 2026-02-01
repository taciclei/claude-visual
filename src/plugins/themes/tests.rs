//! Tests for theme loading

use super::loader::ThemeLoader;
use crate::app::theme::ThemeVariant;

#[test]
fn test_load_zed_theme() {
    let theme_json = r##"{
        "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
        "name": "Test Theme",
        "author": "Test Author",
        "themes": [
            {
                "name": "Dark",
                "appearance": "dark",
                "style": {
                    "background": "#1e1e2e",
                    "text": "#cdd6f4",
                    "text.muted": "#6c7086",
                    "border": "#313244",
                    "status.success": "#a6e3a1",
                    "status.warning": "#f9e2af",
                    "status.error": "#f38ba8",
                    "status.info": "#89b4fa",
                    "syntax": {
                        "keyword": { "color": "#cba6f7" },
                        "string": { "color": "#a6e3a1" },
                        "number": { "color": "#fab387" },
                        "comment": { "color": "#6c7086" },
                        "function": { "color": "#89b4fa" }
                    }
                }
            },
            {
                "name": "Light",
                "appearance": "light",
                "style": {
                    "background": "#eff1f5",
                    "text": "#4c4f69",
                    "text.muted": "#9ca0b0",
                    "border": "#dce0e8"
                }
            }
        ]
    }"##;

    let mut loader = ThemeLoader::new();
    let names = loader.load_json(theme_json).unwrap();

    assert_eq!(names.len(), 2);
    assert!(names.contains(&"Test Theme - Dark".to_string()));
    assert!(names.contains(&"Test Theme - Light".to_string()));

    let dark_theme = loader.get("Test Theme - Dark").unwrap();
    assert!(dark_theme.is_dark);
    assert_eq!(dark_theme.variant, ThemeVariant::Dark);

    let light_theme = loader.get("Test Theme - Light").unwrap();
    assert!(!light_theme.is_dark);
    assert_eq!(light_theme.variant, ThemeVariant::Light);
}

#[test]
fn test_theme_metadata() {
    let theme_json = r#"{
        "name": "Meta Theme",
        "author": "Meta Author",
        "themes": [
            {
                "name": "Dark",
                "appearance": "dark",
                "style": {}
            }
        ]
    }"#;

    let mut loader = ThemeLoader::new();
    loader
        .load_json_with_metadata(theme_json, Some("my-extension"), None)
        .unwrap();

    let metadata = loader.get_metadata("Meta Theme - Dark").unwrap();
    assert_eq!(metadata.extension_id.as_deref(), Some("my-extension"));
    assert_eq!(metadata.author.as_deref(), Some("Meta Author"));

    // Test list by extension
    let ext_themes = loader.list_by_extension("my-extension");
    assert_eq!(ext_themes.len(), 1);
    assert!(ext_themes.contains(&"Meta Theme - Dark"));
}

#[test]
fn test_unload_extension_themes() {
    let theme_json = r#"{
        "name": "Ext Theme",
        "themes": [
            { "name": "A", "appearance": "dark", "style": {} },
            { "name": "B", "appearance": "light", "style": {} }
        ]
    }"#;

    let mut loader = ThemeLoader::new();
    loader
        .load_json_with_metadata(theme_json, Some("test-ext"), None)
        .unwrap();
    assert_eq!(loader.len(), 2);

    let removed = loader.unload_extension("test-ext");
    assert_eq!(removed.len(), 2);
    assert_eq!(loader.len(), 0);
}
