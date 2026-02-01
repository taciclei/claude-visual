//! Chat action handlers

use super::super::core::Workspace;
use crate::{
    ClearConversation, CollapseAllMessages, CopyConversation, CopyLastResponse, ExpandAllMessages,
    NextSearchResult, PrevSearchResult, RegenerateLastResponse, ToggleChatSearch,
    ToggleModelSwitcher, ToggleStats,
};
use gpui::*;

impl Workspace {
    /// Handle toggle chat search action
    pub(in crate::ui::workspace) fn handle_toggle_chat_search(
        &mut self,
        _: &ToggleChatSearch,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_search(cx);
            });
        }
    }

    /// Handle next search result action
    pub(in crate::ui::workspace) fn handle_next_search_result(
        &mut self,
        _: &NextSearchResult,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.next_search_result(cx);
            });
        }
    }

    /// Handle previous search result action
    pub(in crate::ui::workspace) fn handle_prev_search_result(
        &mut self,
        _: &PrevSearchResult,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.prev_search_result(cx);
            });
        }
    }

    /// Handle toggle stats action
    pub(in crate::ui::workspace) fn handle_toggle_stats(
        &mut self,
        _: &ToggleStats,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_stats(cx);
            });
        }
    }

    /// Handle copy conversation action
    pub(in crate::ui::workspace) fn handle_copy_conversation(
        &mut self,
        _: &CopyConversation,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.copy_conversation_to_clipboard(cx);
            });
        }
    }

    /// Handle clear conversation action
    pub(in crate::ui::workspace) fn handle_clear_conversation(
        &mut self,
        _: &ClearConversation,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.clear_conversation(cx);
            });
        }
    }

    /// Handle toggle model switcher action (⌘M)
    pub(in crate::ui::workspace) fn handle_toggle_model_switcher(
        &mut self,
        _: &ToggleModelSwitcher,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.toggle_model_switcher(cx);
            });
        }
    }

    /// Handle copy last response action (⌥⇧C)
    pub(in crate::ui::workspace) fn handle_copy_last_response(
        &mut self,
        _: &CopyLastResponse,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.copy_last_response(cx);
            });
        }
    }

    /// Handle regenerate last response action (⌘⇧R)
    pub(in crate::ui::workspace) fn handle_regenerate_last_response(
        &mut self,
        _: &RegenerateLastResponse,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.regenerate_last_response(cx);
            });
        }
    }

    /// Handle collapse all messages action
    pub(in crate::ui::workspace) fn handle_collapse_all(
        &mut self,
        _: &CollapseAllMessages,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.collapse_all(cx);
            });
        }
    }

    /// Handle expand all messages action
    pub(in crate::ui::workspace) fn handle_expand_all(
        &mut self,
        _: &ExpandAllMessages,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.expand_all(cx);
            });
        }
    }
}
