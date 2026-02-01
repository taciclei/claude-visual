//! Icon theme support for file type and UI icons
//!
//! Supports loading icon themes from extensions, similar to VS Code icon themes.

mod loader;
mod theme;
mod types;
mod utils;

// Re-export public types
pub use loader::IconLoader;
pub use types::{
    FileIcons, IconDefinition, IconTheme, IconThemeManifest, IconThemeMetadata, UiIconKind,
};
pub use utils::default_icon_theme;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_icon_kinds() {
        let all = UiIconKind::all();
        assert!(all.len() > 40);

        assert_eq!(UiIconKind::ChevronRight.default_name(), "chevron-right");
        assert_eq!(UiIconKind::GitBranch.default_name(), "git-branch");
    }

    #[test]
    fn test_icon_loader() {
        let loader = IconLoader::new();
        assert!(loader.list().is_empty());
        assert!(loader.current().is_none());
    }
}
