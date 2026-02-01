//! View action handlers

use gpui::*;
use crate::app::theme::Theme;
use crate::ui::components::toast::Toast;
use crate::{
    ToggleSidebar, ToggleTheme, ToggleVimMode, ToggleWordWrap, ToggleLineNumbers,
    ToggleFocusMode, ToggleHighContrast, ToggleCompactMode, ToggleTimestamps,
    ToggleAutoScroll, ToggleBookmarkedFilter, CycleMessageFilter,
};
use super::super::core::Workspace;

impl Workspace {
    /// Handle toggle sidebar action
    pub(in crate::ui::workspace) fn handle_toggle_sidebar(&mut self, _: &ToggleSidebar, _window: &mut Window, cx: &mut Context<Self>) {
        self.toggle_sidebar(cx);
    }

    /// Handle toggle theme action (dark/light)
    pub(in crate::ui::workspace) fn handle_toggle_theme(&mut self, _: &ToggleTheme, _window: &mut Window, cx: &mut Context<Self>) {
        self.app_state.theme.update(cx, |theme, _cx| {
            // Toggle between dark and light
            let new_theme = if theme.is_dark {
                Theme::light()
            } else {
                Theme::dark()
            };
            *theme = new_theme;
        });
        let is_dark = self.app_state.theme.read(cx).is_dark;
        let message = if is_dark { "Dark theme" } else { "Light theme" };
        self.show_toast(Toast::info(message), cx);
        tracing::info!("Theme toggled to: {}", if is_dark { "dark" } else { "light" });
    }

    /// Toggle theme directly (called from chat view event)
    pub(in crate::ui::workspace) fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        self.app_state.theme.update(cx, |theme, _cx| {
            let new_theme = if theme.is_dark {
                Theme::light()
            } else {
                Theme::dark()
            };
            *theme = new_theme;
        });
        tracing::info!(
            "Theme toggled to: {}",
            if self.app_state.theme.read(cx).is_dark {
                "dark"
            } else {
                "light"
            }
        );
    }

    /// Handle toggle vim mode action
    pub(in crate::ui::workspace) fn handle_toggle_vim_mode(&mut self, _: &ToggleVimMode, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            let is_vim = chat_view.update(cx, |view, cx| {
                view.toggle_vim_mode(cx);
                view.is_vim_mode_enabled(cx)
            });
            self.update_status_bar(cx);
            tracing::info!("Vim mode toggled: {}", if is_vim { "enabled" } else { "disabled" });
        }
    }

    /// Handle toggle word wrap action
    pub(in crate::ui::workspace) fn handle_toggle_word_wrap(&mut self, _: &ToggleWordWrap, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_word_wrap(cx);
            });
            let is_wrap = chat_view.read(cx).is_word_wrap_enabled();
            self.update_status_bar(cx);
            tracing::info!("Word wrap toggled: {}", if is_wrap { "enabled" } else { "disabled" });
        }
    }

    /// Handle toggle line numbers action
    pub(in crate::ui::workspace) fn handle_toggle_line_numbers(&mut self, _: &ToggleLineNumbers, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_line_numbers(cx);
            });
            let show_lines = chat_view.read(cx).is_line_numbers_enabled();
            self.update_status_bar(cx);
            tracing::info!("Line numbers toggled: {}", if show_lines { "shown" } else { "hidden" });
        }
    }

    /// Handle toggle focus mode action
    pub(in crate::ui::workspace) fn handle_toggle_focus_mode(&mut self, _: &ToggleFocusMode, _window: &mut Window, cx: &mut Context<Self>) {
        self.toggle_focus_mode(cx);
    }

    /// Handle toggle high contrast action
    pub(in crate::ui::workspace) fn handle_toggle_high_contrast(&mut self, _: &ToggleHighContrast, _window: &mut Window, cx: &mut Context<Self>) {
        self.app_state.theme.update(cx, |theme, _cx| {
            theme.toggle_high_contrast();
        });
        let is_high_contrast = self.app_state.theme.read(cx).is_high_contrast();
        let message = if is_high_contrast {
            "High contrast mode enabled"
        } else {
            "High contrast mode disabled"
        };
        self.show_toast(Toast::info(message), cx);
        tracing::info!("{}", message);
        cx.notify();
    }

    /// Handle toggle compact mode action
    pub(in crate::ui::workspace) fn handle_toggle_compact_mode(&mut self, _: &ToggleCompactMode, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_compact_mode(cx);
            });
            let is_compact = chat_view.read(cx).is_compact_mode();
            let message = if is_compact { "Compact mode enabled" } else { "Compact mode disabled" };
            self.show_toast(Toast::info(message), cx);
            tracing::info!("{}", message);
        }
    }

    /// Handle toggle timestamps action
    pub(in crate::ui::workspace) fn handle_toggle_timestamps(&mut self, _: &ToggleTimestamps, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_timestamps(cx);
            });
            let show_timestamps = chat_view.read(cx).timestamps_visible();
            let message = if show_timestamps { "Timestamps visible" } else { "Timestamps hidden" };
            self.show_toast(Toast::info(message), cx);
            tracing::info!("{}", message);
        }
    }

    /// Handle toggle auto-scroll action
    pub(in crate::ui::workspace) fn handle_toggle_auto_scroll(&mut self, _: &ToggleAutoScroll, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_auto_scroll(cx);
            });
            // Note: ChatView.auto_scroll is private, so we just show a generic message
            self.show_toast(Toast::info("Auto-scroll toggled"), cx);
            tracing::info!("Auto-scroll toggled");
        }
    }

    /// Handle toggle bookmarked filter action
    pub(in crate::ui::workspace) fn handle_toggle_bookmarked_filter(&mut self, _: &ToggleBookmarkedFilter, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_bookmarked_filter(cx);
            });
            let show_bookmarked = chat_view.read(cx).is_bookmarked_filter_active();
            let message = if show_bookmarked {
                "Showing bookmarked messages only"
            } else {
                "Showing all messages"
            };
            self.show_toast(Toast::info(message), cx);
            tracing::info!("{}", message);
        }
    }

    /// Handle cycle message filter action
    pub(in crate::ui::workspace) fn handle_cycle_message_filter(&mut self, _: &CycleMessageFilter, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.next_filter(cx);
            });
            let filter = chat_view.read(cx).message_filter();
            let message = format!("Filter: {}", filter.label());
            self.show_toast(Toast::info(message), cx);
            self.update_status_bar(cx);
            tracing::info!("Message filter: {}", filter.label());
        }
    }
}
