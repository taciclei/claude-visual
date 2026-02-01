//! Navigation action handlers

use gpui::*;
use crate::{
    SelectNextMessage, SelectPrevMessage, SelectFirstMessage, SelectLastMessage,
    CopySelectedMessage, BookmarkSelectedMessage, ScrollToTop, ScrollToBottom,
    SkipToMain, SkipToInput, SkipToNavigation, SkipToSidebar,
};
use super::super::core::Workspace;

impl Workspace {
    /// Handle select next message action
    pub(in crate::ui::workspace) fn handle_select_next_message(&mut self, _: &SelectNextMessage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.select_next_message(cx);
            });
        }
    }

    /// Handle select previous message action
    pub(in crate::ui::workspace) fn handle_select_prev_message(&mut self, _: &SelectPrevMessage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.select_prev_message(cx);
            });
        }
    }

    /// Handle select first message action
    pub(in crate::ui::workspace) fn handle_select_first_message(&mut self, _: &SelectFirstMessage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.select_first_message(cx);
            });
        }
    }

    /// Handle select last message action
    pub(in crate::ui::workspace) fn handle_select_last_message(&mut self, _: &SelectLastMessage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.select_last_message(cx);
            });
        }
    }

    /// Handle copy selected message action
    pub(in crate::ui::workspace) fn handle_copy_selected_message(&mut self, _: &CopySelectedMessage, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.copy_selected_message(cx);
            });
        }
    }

    /// Handle bookmark selected message action
    pub(in crate::ui::workspace) fn handle_bookmark_selected_message(
        &mut self,
        _: &BookmarkSelectedMessage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.bookmark_selected_message(cx);
            });
        }
    }

    /// Handle scroll to top action
    pub(in crate::ui::workspace) fn handle_scroll_to_top(&mut self, _: &ScrollToTop, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.select_first_message(cx);
            });
            tracing::info!("Scrolled to top");
        }
    }

    /// Handle scroll to bottom action
    pub(in crate::ui::workspace) fn handle_scroll_to_bottom(&mut self, _: &ScrollToBottom, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.scroll_to_bottom(cx);
            });
            tracing::info!("Scrolled to bottom");
        }
    }

    // ==================== Skip Link Handlers ====================

    /// Handle skip to main content (Alt+1)
    pub(in crate::ui::workspace) fn handle_skip_to_main(&mut self, _: &SkipToMain, window: &mut Window, _cx: &mut Context<Self>) {
        self.main_focus.focus(window);
        tracing::info!("Skipped to main content");
    }

    /// Handle skip to chat input (Alt+2)
    pub(in crate::ui::workspace) fn handle_skip_to_input(&mut self, _: &SkipToInput, window: &mut Window, cx: &mut Context<Self>) {
        // Focus the input field in the active chat view
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.focus_input(window, cx);
            });
        }
        tracing::info!("Skipped to chat input");
    }

    /// Handle skip to navigation (Alt+3)
    pub(in crate::ui::workspace) fn handle_skip_to_navigation(&mut self, _: &SkipToNavigation, window: &mut Window, _cx: &mut Context<Self>) {
        self.navigation_focus.focus(window);
        tracing::info!("Skipped to navigation");
    }

    /// Handle skip to sidebar (Alt+4)
    pub(in crate::ui::workspace) fn handle_skip_to_sidebar(&mut self, _: &SkipToSidebar, window: &mut Window, cx: &mut Context<Self>) {
        // Show sidebar if hidden
        if !self.show_sidebar {
            self.show_sidebar = true;
        }
        self.sidebar_focus.focus(window);
        tracing::info!("Skipped to sidebar");
        cx.notify();
    }
}
