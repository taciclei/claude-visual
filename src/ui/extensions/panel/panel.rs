use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::plugins::ExtensionManifest;

use super::types::*;

/// Extensions panel component
pub struct ExtensionsPanel {
    pub(super) app_state: Arc<AppState>,
    pub(super) focus_handle: FocusHandle,
    /// Current tab
    pub(super) active_tab: ExtensionsTab,
    /// Search query
    pub(super) search_query: String,
    /// Installed extensions
    pub(super) installed: Vec<ExtensionItem>,
    /// Selected extension (for details)
    pub(super) selected: Option<String>,
}

impl ExtensionsPanel {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        // Load installed extensions (mock data for now)
        let installed = vec![
            ExtensionItem {
                manifest: ExtensionManifest {
                    id: "catppuccin-theme".to_string(),
                    name: "Catppuccin Theme".to_string(),
                    version: "1.0.0".to_string(),
                    authors: vec!["Catppuccin".to_string()],
                    description: Some("Soothing pastel theme for Claude Visual".to_string()),
                    repository: Some("https://github.com/catppuccin/claude-visual".to_string()),
                    lib: None,
                    themes: Some(vec![]),
                    languages: None,
                    grammars: None,
                    icon_themes: None,
                },
                enabled: true,
                has_update: false,
            },
            ExtensionItem {
                manifest: ExtensionManifest {
                    id: "git-lens".to_string(),
                    name: "Git Lens".to_string(),
                    version: "2.1.0".to_string(),
                    authors: vec!["Claude Visual Team".to_string()],
                    description: Some("Enhanced git integration with blame annotations".to_string()),
                    repository: None,
                    lib: Some(crate::plugins::LibConfig {
                        kind: Some("rust".to_string()),
                        version: Some("0.1.0".to_string()),
                    }),
                    themes: None,
                    languages: None,
                    grammars: None,
                    icon_themes: None,
                },
                enabled: true,
                has_update: true,
            },
            ExtensionItem {
                manifest: ExtensionManifest {
                    id: "python-support".to_string(),
                    name: "Python Language Support".to_string(),
                    version: "3.0.0".to_string(),
                    authors: vec!["Language Team".to_string()],
                    description: Some("Full Python language support with LSP".to_string()),
                    repository: None,
                    lib: None,
                    themes: None,
                    languages: Some(vec![]),
                    grammars: None,
                    icon_themes: None,
                },
                enabled: false,
                has_update: false,
            },
        ];

        Self {
            app_state,
            focus_handle,
            active_tab: ExtensionsTab::default(),
            search_query: String::new(),
            installed,
            selected: None,
        }
    }

    /// Switch to a different tab
    pub(super) fn switch_tab(&mut self, tab: ExtensionsTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.selected = None;
        cx.notify();
    }

    /// Update search query
    pub(super) fn set_search(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        cx.notify();
    }

    /// Select an extension
    pub(super) fn select_extension(&mut self, id: &str, cx: &mut Context<Self>) {
        self.selected = Some(id.to_string());
        cx.notify();
    }

    /// Toggle extension enabled state
    pub(super) fn toggle_extension(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(ext) = self.installed.iter_mut().find(|e| e.manifest.id == id) {
            ext.enabled = !ext.enabled;
            if ext.enabled {
                cx.emit(ExtensionsPanelEvent::ExtensionEnabled(id.to_string()));
            } else {
                cx.emit(ExtensionsPanelEvent::ExtensionDisabled(id.to_string()));
            }
            cx.notify();
        }
    }

    /// Uninstall an extension
    pub(super) fn uninstall_extension(&mut self, id: &str, cx: &mut Context<Self>) {
        self.installed.retain(|e| e.manifest.id != id);
        if self.selected.as_deref() == Some(id) {
            self.selected = None;
        }
        cx.emit(ExtensionsPanelEvent::ExtensionUninstalled(id.to_string()));
        cx.notify();
    }

    /// Dismiss the panel
    pub(super) fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(ExtensionsPanelEvent::Dismissed);
    }

    /// Get filtered extensions based on search
    pub(super) fn filtered_extensions(&self) -> Vec<&ExtensionItem> {
        let query = self.search_query.to_lowercase();
        self.installed
            .iter()
            .filter(|ext| {
                if query.is_empty() {
                    return true;
                }
                ext.manifest.name.to_lowercase().contains(&query)
                    || ext.manifest.id.to_lowercase().contains(&query)
                    || ext.manifest.description.as_ref()
                        .map(|d| d.to_lowercase().contains(&query))
                        .unwrap_or(false)
            })
            .collect()
    }
}

impl EventEmitter<ExtensionsPanelEvent> for ExtensionsPanel {}

impl Focusable for ExtensionsPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
