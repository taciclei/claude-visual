//! Settings modal core logic

use std::sync::Arc;
use gpui::*;
use crate::app::settings::Settings;
use crate::app::state::AppState;
use crate::i18n::i18n;
use super::types::{SettingsTab, SettingsModalEvent};

/// Settings modal component
pub struct SettingsModal {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) active_tab: SettingsTab,
    /// Pending settings changes (not yet saved)
    pub(crate) pending: Settings,
    /// Whether there are unsaved changes
    pub(crate) has_changes: bool,
    /// Show reset confirmation dialog
    pub(crate) show_reset_confirmation: bool,
    /// Show import/export dialog
    pub(crate) show_import_export: bool,
    /// Import/export mode (true = import, false = export)
    pub(crate) import_mode: bool,
    /// Import/export text content
    pub(crate) import_export_text: String,
    /// Import/export error message
    pub(crate) import_export_error: Option<String>,
}

impl EventEmitter<SettingsModalEvent> for SettingsModal {}

impl SettingsModal {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let pending = app_state.settings.read(cx).clone();

        Self {
            app_state,
            focus_handle,
            active_tab: SettingsTab::Appearance,
            pending,
            has_changes: false,
            show_reset_confirmation: false,
            show_import_export: false,
            import_mode: false,
            import_export_text: String::new(),
            import_export_error: None,
        }
    }

    /// Switch to a different settings tab
    pub(crate) fn switch_tab(&mut self, tab: SettingsTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        cx.notify();
    }

    /// Mark settings as changed
    pub(crate) fn mark_changed(&mut self, cx: &mut Context<Self>) {
        self.has_changes = true;
        cx.notify();
    }

    /// Save pending settings
    pub(crate) fn save(&mut self, cx: &mut Context<Self>) {
        // Update the app state settings
        self.app_state.settings.update(cx, |settings, _cx| {
            *settings = self.pending.clone();
        });

        // Save to disk
        if let Err(e) = self.pending.save() {
            tracing::error!("Failed to save settings: {}", e);
        } else {
            tracing::info!("Settings saved");
        }

        // Update theme if changed
        let theme_clone = self.app_state.theme.read(cx).clone();
        let current_theme_name = theme_clone.name.clone();
        let theme_mode_name = theme_clone.mode_name();
        let theme_changed = current_theme_name != self.pending.ui.theme
            && theme_mode_name != self.pending.ui.theme;

        if theme_changed {
            // First check if it's a built-in theme
            let theme_name = &self.pending.ui.theme;
            let new_theme = match theme_name.as_str() {
                "dark" => Some(crate::app::theme::Theme::dark()),
                "light" => Some(crate::app::theme::Theme::light()),
                "high-contrast-dark" => Some(crate::app::theme::Theme::high_contrast_dark()),
                "high-contrast-light" => Some(crate::app::theme::Theme::high_contrast_light()),
                _ => {
                    // Try to load from extension themes
                    self.app_state.theme_loader.read().get(theme_name).cloned()
                }
            };

            if let Some(new_theme) = new_theme {
                self.app_state.theme.update(cx, |theme, _cx| {
                    *theme = new_theme;
                });
                tracing::info!("Theme set to: {}", theme_name);
            } else {
                tracing::warn!("Theme not found: {}", theme_name);
            }
        }

        // Update language if changed
        let new_locale = self.pending.language.effective_locale();
        i18n().set_locale(new_locale);
        tracing::info!("Language set to: {}", new_locale.native_name());

        self.has_changes = false;
        cx.emit(SettingsModalEvent::Saved);
        cx.notify();
    }

    /// Show reset confirmation dialog
    pub(crate) fn show_reset_confirmation(&mut self, cx: &mut Context<Self>) {
        self.show_reset_confirmation = true;
        cx.notify();
    }

    /// Hide reset confirmation dialog
    pub(crate) fn hide_reset_confirmation(&mut self, cx: &mut Context<Self>) {
        self.show_reset_confirmation = false;
        cx.notify();
    }

    /// Reset settings to defaults
    pub(crate) fn reset_to_defaults(&mut self, cx: &mut Context<Self>) {
        self.pending = Settings::default();
        self.has_changes = true;
        self.show_reset_confirmation = false;
        cx.notify();
    }

    /// Show export dialog
    pub(crate) fn show_export(&mut self, cx: &mut Context<Self>) {
        self.import_mode = false;
        match self.pending.export_json() {
            Ok(json) => {
                self.import_export_text = json;
                self.import_export_error = None;
            }
            Err(e) => {
                self.import_export_text = String::new();
                self.import_export_error = Some(format!("Export failed: {}", e));
            }
        }
        self.show_import_export = true;
        cx.notify();
    }

    /// Show import dialog
    pub(crate) fn show_import(&mut self, cx: &mut Context<Self>) {
        self.import_mode = true;
        self.import_export_text = String::new();
        self.import_export_error = None;
        self.show_import_export = true;
        cx.notify();
    }

    /// Hide import/export dialog
    pub(crate) fn hide_import_export(&mut self, cx: &mut Context<Self>) {
        self.show_import_export = false;
        self.import_export_text.clear();
        self.import_export_error = None;
        cx.notify();
    }

    /// Apply imported settings
    pub(crate) fn apply_import(&mut self, cx: &mut Context<Self>) {
        match Settings::import_json(&self.import_export_text) {
            Ok(settings) => {
                self.pending = settings;
                self.has_changes = true;
                self.import_export_error = None;
                self.show_import_export = false;
                tracing::info!("Settings imported successfully");
            }
            Err(e) => {
                self.import_export_error = Some(format!("Invalid JSON: {}", e));
            }
        }
        cx.notify();
    }

    /// Copy export text to clipboard
    pub(crate) fn copy_to_clipboard(&self, cx: &mut Context<Self>) {
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(self.import_export_text.clone()));
        tracing::info!("Settings exported to clipboard");
    }

    /// Paste from clipboard to import field
    pub(crate) fn paste_from_clipboard(&mut self, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            if let Some(text) = item.text() {
                self.import_export_text = text.to_string();
                self.import_export_error = None;
                cx.notify();
            }
        }
    }

    /// Dismiss the modal
    pub(crate) fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(SettingsModalEvent::Dismissed);
    }
}

impl Focusable for SettingsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
