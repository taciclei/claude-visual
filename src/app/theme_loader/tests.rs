//! Tests for theme loader

use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::app::theme::Theme;

use super::*;

fn create_test_theme_file(dir: &Path, name: &str, is_dark: bool) -> PathBuf {
    let path = dir.join(format!("{}.json", name));
    let theme = if is_dark {
        Theme::dark()
    } else {
        Theme::light()
    };

    let mut theme_json = theme;
    theme_json.name = name.to_string();

    let content = serde_json::to_string_pretty(&theme_json).unwrap();
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    path
}

#[test]
fn test_theme_format_detection() {
    assert_eq!(ThemeFormat::from_path(Path::new("theme.json")), Some(ThemeFormat::Json));
    assert_eq!(ThemeFormat::from_path(Path::new("theme.toml")), Some(ThemeFormat::Toml));
    assert_eq!(ThemeFormat::from_path(Path::new("theme.txt")), None);
}

#[test]
fn test_theme_builder() {
    let theme = ThemeBuilder::new("Custom")
        .dark()
        .author("Test Author")
        .description("A test theme")
        .build();

    assert_eq!(theme.name, "Custom");
    assert!(theme.is_dark);
}

#[tokio::test]
async fn test_theme_discovery() {
    let temp_dir = TempDir::new().unwrap();
    let themes_dir = temp_dir.path().to_path_buf();

    // Create test theme files
    create_test_theme_file(&themes_dir, "dark-theme", true);
    create_test_theme_file(&themes_dir, "light-theme", false);

    let loader = ThemeLoader::new(themes_dir);
    let themes = loader.discover_themes().await.unwrap();

    assert_eq!(themes.len(), 2);
    assert!(themes.iter().any(|t| t.name == "dark-theme"));
    assert!(themes.iter().any(|t| t.name == "light-theme"));
}

#[tokio::test]
async fn test_theme_loading() {
    let temp_dir = TempDir::new().unwrap();
    let themes_dir = temp_dir.path().to_path_buf();

    create_test_theme_file(&themes_dir, "test-theme", true);

    let loader = ThemeLoader::new(themes_dir);
    loader.discover_themes().await.unwrap();

    let theme = loader.load_theme("test-theme").await.unwrap();
    assert_eq!(theme.name, "test-theme");

    // Should be cached now
    assert!(loader.is_cached("test-theme"));
}

#[test]
fn test_loader_state() {
    let loader = ThemeLoader::new(PathBuf::from("/nonexistent"));
    assert_eq!(loader.state(), ThemeLoadState::NotStarted);
}
