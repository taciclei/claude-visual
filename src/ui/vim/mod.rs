//! Vim mode for efficient keyboard navigation
//!
//! Provides Vim-like modal editing with Normal, Insert, Visual, and Command modes.

mod mode;
mod keymaps;

pub use mode::{VimMode, VimState};
pub use keymaps::{VimAction, VimKeyHandler};

use gpui::*;
use gpui::prelude::*;
use gpui::prelude::*;

use std::sync::Arc;
use crate::app::state::AppState;

/// Events emitted by VimState
pub enum VimEvent {
    /// Mode changed
    ModeChanged(VimMode),
    /// Action executed
    ActionExecuted(VimAction),
    /// Command entered (from command mode)
    CommandEntered(String),
}

impl EventEmitter<VimEvent> for VimState {}

/// Vim status line component for displaying current mode
pub struct VimStatusLine {
    app_state: Arc<AppState>,
    mode: VimMode,
    command_buffer: String,
    search_pattern: String,
}

impl VimStatusLine {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            mode: VimMode::Normal,
            command_buffer: String::new(),
            search_pattern: String::new(),
        }
    }

    /// Update the current mode
    pub fn set_mode(&mut self, mode: VimMode, cx: &mut Context<Self>) {
        self.mode = mode;
        // Clear buffers when changing modes
        if mode != VimMode::Command {
            self.command_buffer.clear();
        }
        if mode != VimMode::Normal {
            self.search_pattern.clear();
        }
        cx.notify();
    }

    /// Update command buffer
    pub fn set_command_buffer(&mut self, buffer: String, cx: &mut Context<Self>) {
        self.command_buffer = buffer;
        cx.notify();
    }

    /// Update search pattern
    pub fn set_search_pattern(&mut self, pattern: String, cx: &mut Context<Self>) {
        self.search_pattern = pattern;
        cx.notify();
    }
}

impl Render for VimStatusLine {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Mode colors
        let (mode_bg, mode_text) = match self.mode {
            VimMode::Normal => (theme.colors.accent, theme.colors.text),
            VimMode::Insert => (theme.colors.success, theme.colors.background),
            VimMode::Visual => (theme.colors.warning, theme.colors.background),
            VimMode::VisualLine => (theme.colors.warning, theme.colors.background),
            VimMode::VisualBlock => (theme.colors.warning, theme.colors.background),
            VimMode::Command => (theme.colors.surface, theme.colors.text),
            VimMode::Search => (theme.colors.surface, theme.colors.text),
        };

        let mode_label = match self.mode {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::VisualLine => "V-LINE",
            VimMode::VisualBlock => "V-BLOCK",
            VimMode::Command => "COMMAND",
            VimMode::Search => "SEARCH",
        };

        div()
            .id("vim-status-line")
            .h(px(24.0))
            .w_full()
            .bg(theme.colors.surface)
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            // Mode indicator
            .child(
                div()
                    .px_2()
                    .py_px()
                    .bg(mode_bg)
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(mode_text)
                    .child(mode_label),
            )
            // Command/search buffer
            .when(self.mode == VimMode::Command, |this| {
                this.child(
                    div()
                        .flex_1()
                        .text_xs()
                        .text_color(theme.colors.text)
                        .child(format!(":{}", self.command_buffer)),
                )
            })
            .when(self.mode == VimMode::Search, |this| {
                this.child(
                    div()
                        .flex_1()
                        .text_xs()
                        .text_color(theme.colors.text)
                        .child(format!("/{}", self.search_pattern)),
                )
            })
            // Spacer
            .when(self.mode != VimMode::Command && self.mode != VimMode::Search, |this| {
                this.child(div().flex_1())
            })
            // Help hint
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .px_2()
                    .child(match self.mode {
                        VimMode::Normal => "Press 'i' to insert, ':' for command",
                        VimMode::Insert => "Press Esc to return to Normal",
                        VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => {
                            "Press Esc to cancel, y to yank, d to delete"
                        }
                        VimMode::Command => "Enter to execute, Esc to cancel",
                        VimMode::Search => "Enter to search, Esc to cancel",
                    }),
            )
    }
}
