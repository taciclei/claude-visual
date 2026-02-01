//! Core ThemeEditor struct and methods

use std::sync::Arc;
use gpui::*;

use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::ui::extensions::theme_editor::types::{EditingColor, ThemeEditorTab, ThemeEditorEvent};

/// Theme editor component
pub struct ThemeEditor {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) focus_handle: FocusHandle,
    /// Active tab
    pub(crate) active_tab: ThemeEditorTab,
    /// Theme being edited
    pub(crate) editing_theme: Theme,
    /// Original theme (for reset)
    pub(crate) original_theme: Option<Theme>,
    /// Theme name
    pub(crate) theme_name: String,
    /// Is this a new theme or editing existing
    pub(crate) is_new: bool,
    /// Currently selected color for editing
    pub(crate) selected_color: Option<EditingColor>,
    /// Color picker input (hex)
    pub(crate) color_input: String,
    /// Has unsaved changes
    pub(crate) has_changes: bool,
}

impl ThemeEditor {
    /// Create a new theme editor (new theme)
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let editing_theme = Theme::dark();

        Self {
            app_state,
            focus_handle,
            active_tab: ThemeEditorTab::Colors,
            editing_theme,
            original_theme: None,
            theme_name: "New Theme".to_string(),
            is_new: true,
            selected_color: None,
            color_input: String::new(),
            has_changes: false,
        }
    }

    /// Create editor for an existing theme
    pub fn edit(app_state: Arc<AppState>, theme: Theme, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let theme_name = theme.name.clone();
        let original = theme.clone();

        Self {
            app_state,
            focus_handle,
            active_tab: ThemeEditorTab::Colors,
            editing_theme: theme,
            original_theme: Some(original),
            theme_name,
            is_new: false,
            selected_color: None,
            color_input: String::new(),
            has_changes: false,
        }
    }

    /// Create editor from a base theme (duplicate)
    pub fn duplicate(app_state: Arc<AppState>, base: Theme, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let mut editing_theme = base;
        let theme_name = format!("{} Copy", editing_theme.name);
        editing_theme.name = theme_name.clone();

        Self {
            app_state,
            focus_handle,
            active_tab: ThemeEditorTab::Colors,
            editing_theme,
            original_theme: None,
            theme_name,
            is_new: true,
            selected_color: None,
            color_input: String::new(),
            has_changes: true,
        }
    }

    /// Switch active tab
    pub(crate) fn switch_tab(&mut self, tab: ThemeEditorTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        cx.notify();
    }

    /// Set theme variant (dark/light)
    pub(crate) fn set_variant(&mut self, variant: crate::app::theme::ThemeVariant, cx: &mut Context<Self>) {
        self.editing_theme.variant = variant;
        self.editing_theme.is_dark = variant.is_dark();
        self.has_changes = true;
        cx.notify();
    }

    /// Reset to original theme
    pub(crate) fn reset(&mut self, cx: &mut Context<Self>) {
        if let Some(original) = &self.original_theme {
            self.editing_theme = original.clone();
            self.has_changes = false;
            cx.notify();
        }
    }

    /// Close the editor
    pub(crate) fn close(&mut self, cx: &mut Context<Self>) {
        cx.emit(ThemeEditorEvent::Closed);
    }
}

impl Focusable for ThemeEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
