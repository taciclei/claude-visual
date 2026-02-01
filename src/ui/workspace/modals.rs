//! Modal and overlay management

use gpui::*;
use crate::ui::components::command_palette::{CommandPalette, CommandPaletteEvent};
use crate::ui::components::shortcuts_panel::{ShortcutsPanel, ShortcutsPanelEvent};
use crate::ui::settings::{SettingsModal, SettingsModalEvent};
use super::core::Workspace;

impl Workspace {
    /// Show the command palette
    pub(in crate::ui::workspace) fn show_command_palette(&mut self, cx: &mut Context<Self>) {
        if self.command_palette.is_some() {
            return; // Already open
        }

        let app_state = self.app_state.clone();
        let palette = cx.new(|cx| CommandPalette::new(app_state, cx));

        // Subscribe to palette events
        cx.subscribe(&palette, |this, _, event: &CommandPaletteEvent, cx| {
            match event {
                CommandPaletteEvent::CommandSelected(command_id) => {
                    this.execute_command(command_id, cx);
                    this.hide_command_palette(cx);
                }
                CommandPaletteEvent::Dismissed => {
                    this.hide_command_palette(cx);
                }
            }
        })
        .detach();

        self.command_palette = Some(palette);
        cx.notify();
    }

    /// Hide the command palette
    pub(in crate::ui::workspace) fn hide_command_palette(&mut self, cx: &mut Context<Self>) {
        self.command_palette = None;
        cx.notify();
    }

    /// Show the settings modal
    pub(in crate::ui::workspace) fn show_settings_modal(&mut self, cx: &mut Context<Self>) {
        if self.settings_modal.is_some() {
            return; // Already open
        }

        let app_state = self.app_state.clone();
        let modal = cx.new(|cx| SettingsModal::new(app_state, cx));

        // Subscribe to modal events
        cx.subscribe(&modal, |this, _, event: &SettingsModalEvent, cx| {
            match event {
                SettingsModalEvent::Dismissed => {
                    this.hide_settings_modal(cx);
                }
                SettingsModalEvent::Saved => {
                    tracing::info!("Settings saved");
                    this.hide_settings_modal(cx);
                }
            }
        })
        .detach();

        // Modal will focus itself when rendered
        self.settings_modal = Some(modal);
        cx.notify();
    }

    /// Hide the settings modal
    pub(in crate::ui::workspace) fn hide_settings_modal(&mut self, cx: &mut Context<Self>) {
        self.settings_modal = None;
        cx.notify();
    }

    /// Show the keyboard shortcuts panel
    pub(in crate::ui::workspace) fn show_shortcuts_panel(&mut self, cx: &mut Context<Self>) {
        if self.shortcuts_panel.is_some() {
            // Toggle off if already open
            self.hide_shortcuts_panel(cx);
            return;
        }

        let app_state = self.app_state.clone();
        let panel = cx.new(|cx| ShortcutsPanel::new(app_state, cx));

        // Subscribe to panel events
        cx.subscribe(&panel, |this, _, event: &ShortcutsPanelEvent, cx| {
            match event {
                ShortcutsPanelEvent::Dismissed => {
                    this.hide_shortcuts_panel(cx);
                }
            }
        })
        .detach();

        // Panel will focus itself when rendered
        self.shortcuts_panel = Some(panel);
        cx.notify();
    }

    /// Hide the keyboard shortcuts panel
    pub(in crate::ui::workspace) fn hide_shortcuts_panel(&mut self, cx: &mut Context<Self>) {
        self.shortcuts_panel = None;
        cx.notify();
    }
}
