//! Miscellaneous action handlers

use super::super::core::Workspace;
use crate::ui::components::toast::Toast;
use crate::{
    DecreaseFontSize, DismissOverlays, ExportConversation, IncreaseFontSize, NewConversation,
    OpenCommandPalette, OpenSettings, ResetFontSize, ShowShortcuts,
};
use gpui::*;

impl Workspace {
    /// Handle new conversation action
    pub(in crate::ui::workspace) fn handle_new_conversation(
        &mut self,
        _: &NewConversation,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.new_conversation(cx);
    }

    /// Handle open command palette action
    pub(in crate::ui::workspace) fn handle_open_command_palette(
        &mut self,
        _: &OpenCommandPalette,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_command_palette(cx);
    }

    /// Handle open settings action
    pub(in crate::ui::workspace) fn handle_open_settings(
        &mut self,
        _: &OpenSettings,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_settings_modal(cx);
    }

    /// Handle export conversation action
    pub(in crate::ui::workspace) fn handle_export_conversation(
        &mut self,
        _: &ExportConversation,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.export_conversation(cx);
    }

    /// Handle show shortcuts action
    pub(in crate::ui::workspace) fn handle_show_shortcuts(
        &mut self,
        _: &ShowShortcuts,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_shortcuts_panel(cx);
    }

    /// Handle increase font size action
    pub(in crate::ui::workspace) fn handle_increase_font_size(
        &mut self,
        _: &IncreaseFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_size = {
            self.app_state.settings.update(cx, |settings, _cx| {
                settings.increase_font_size();
                let _ = settings.save();
                settings.ui.ui_font_size
            })
        };
        self.show_toast(Toast::info(format!("Font size: {:.0}px", new_size)), cx);
        tracing::info!("Font size increased to {}", new_size);
        cx.notify();
    }

    /// Handle decrease font size action
    pub(in crate::ui::workspace) fn handle_decrease_font_size(
        &mut self,
        _: &DecreaseFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_size = {
            self.app_state.settings.update(cx, |settings, _cx| {
                settings.decrease_font_size();
                let _ = settings.save();
                settings.ui.ui_font_size
            })
        };
        self.show_toast(Toast::info(format!("Font size: {:.0}px", new_size)), cx);
        tracing::info!("Font size decreased to {}", new_size);
        cx.notify();
    }

    /// Handle reset font size action
    pub(in crate::ui::workspace) fn handle_reset_font_size(
        &mut self,
        _: &ResetFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.app_state.settings.update(cx, |settings, _cx| {
            settings.reset_font_size();
            let _ = settings.save();
        });
        self.show_toast(Toast::info("Font size reset to 14px"), cx);
        tracing::info!("Font size reset to default");
        cx.notify();
    }

    /// Handle dismiss overlays action (Escape key)
    pub(in crate::ui::workspace) fn handle_dismiss_overlays(
        &mut self,
        _: &DismissOverlays,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Dismiss in order of priority: shortcuts panel, command palette, settings, search, focus mode
        if self.shortcuts_panel.is_some() {
            self.hide_shortcuts_panel(cx);
            return;
        }
        if self.command_palette.is_some() {
            self.hide_command_palette(cx);
            return;
        }
        if self.settings_modal.is_some() {
            self.hide_settings_modal(cx);
            return;
        }
        if self.diff_preview.is_some() {
            self.diff_preview = None;
            cx.notify();
            return;
        }
        // Clear toasts
        self.toast_container.update(cx, |container, cx| {
            container.clear(cx);
        });
        // If in focus mode, exit it
        if self.focus_mode {
            self.toggle_focus_mode(cx);
        }
    }
}
