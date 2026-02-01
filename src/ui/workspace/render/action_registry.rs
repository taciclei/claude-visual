//! Action handler registration for workspace

use gpui::*;
use crate::{
    AddTestsAction, BookmarkSelectedMessage, ClearConversation, CloseTab, CollapseAllMessages,
    CopyConversation, CopySelectedMessage, ExecuteCodeAction, ExplainCodeAction, ExpandAllMessages,
    ExportConversation, ImproveCodeAction, NewConversation, NewTab, NextSearchResult, NextTab,
    OpenCommandPalette, OpenSettings, PrevSearchResult, PrevTab, RefactorCodeAction, ReviewCodeAction,
    SaveCodeToFileAction, SelectFirstMessage, SelectLastMessage, SelectNextMessage, SelectPrevMessage,
    SelectTab1, SelectTab2, SelectTab3, SelectTab4, SelectTab5, SelectTab6, SelectTab7, SelectTab8,
    SelectTab9, ToggleChatSearch, ToggleSidebar, ToggleStats, ToggleTheme, ToggleVimMode,
    ToggleWordWrap, ToggleLineNumbers, ToggleFocusMode, ShowShortcuts, IncreaseFontSize,
    DecreaseFontSize, ResetFontSize, ToggleHighContrast, ToggleCompactMode, ToggleTimestamps,
    ToggleAutoScroll, ToggleBookmarkedFilter, CycleMessageFilter, ScrollToTop, ScrollToBottom,
    DismissOverlays, ToggleModelSwitcher, CopyLastResponse, RegenerateLastResponse,
    SkipToMain, SkipToInput, SkipToNavigation, SkipToSidebar,
};
use super::super::core::Workspace;

/// Registers all action handlers on the provided div
pub fn register_actions(div: Stateful<Div>, cx: &mut Context<Workspace>) -> Stateful<Div> {
    div
        // Core workspace actions
        .on_action(cx.listener(Workspace::handle_toggle_sidebar))
        .on_action(cx.listener(Workspace::handle_new_conversation))
        .on_action(cx.listener(Workspace::handle_open_command_palette))
        .on_action(cx.listener(Workspace::handle_open_settings))
        .on_action(cx.listener(Workspace::handle_execute_code))
        .on_action(cx.listener(Workspace::handle_save_code_to_file))
        .on_action(cx.listener(Workspace::handle_explain_code))
        .on_action(cx.listener(Workspace::handle_improve_code))
        .on_action(cx.listener(Workspace::handle_add_tests))
        .on_action(cx.listener(Workspace::handle_review_code))
        .on_action(cx.listener(Workspace::handle_refactor_code))
        // Tab actions
        .on_action(cx.listener(Workspace::handle_new_tab))
        .on_action(cx.listener(Workspace::handle_close_tab))
        .on_action(cx.listener(Workspace::handle_next_tab))
        .on_action(cx.listener(Workspace::handle_prev_tab))
        .on_action(cx.listener(|this, _: &SelectTab1, _window, cx| this.handle_select_tab(1, cx)))
        .on_action(cx.listener(|this, _: &SelectTab2, _window, cx| this.handle_select_tab(2, cx)))
        .on_action(cx.listener(|this, _: &SelectTab3, _window, cx| this.handle_select_tab(3, cx)))
        .on_action(cx.listener(|this, _: &SelectTab4, _window, cx| this.handle_select_tab(4, cx)))
        .on_action(cx.listener(|this, _: &SelectTab5, _window, cx| this.handle_select_tab(5, cx)))
        .on_action(cx.listener(|this, _: &SelectTab6, _window, cx| this.handle_select_tab(6, cx)))
        .on_action(cx.listener(|this, _: &SelectTab7, _window, cx| this.handle_select_tab(7, cx)))
        .on_action(cx.listener(|this, _: &SelectTab8, _window, cx| this.handle_select_tab(8, cx)))
        .on_action(cx.listener(|this, _: &SelectTab9, _window, cx| this.handle_select_tab(9, cx)))
        // Chat view actions
        .on_action(cx.listener(Workspace::handle_toggle_chat_search))
        .on_action(cx.listener(Workspace::handle_next_search_result))
        .on_action(cx.listener(Workspace::handle_prev_search_result))
        .on_action(cx.listener(Workspace::handle_toggle_stats))
        .on_action(cx.listener(Workspace::handle_export_conversation))
        .on_action(cx.listener(Workspace::handle_copy_conversation))
        .on_action(cx.listener(Workspace::handle_clear_conversation))
        .on_action(cx.listener(Workspace::handle_toggle_model_switcher))
        .on_action(cx.listener(Workspace::handle_copy_last_response))
        .on_action(cx.listener(Workspace::handle_regenerate_last_response))
        .on_action(cx.listener(Workspace::handle_collapse_all))
        .on_action(cx.listener(Workspace::handle_expand_all))
        // Message navigation actions
        .on_action(cx.listener(Workspace::handle_select_next_message))
        .on_action(cx.listener(Workspace::handle_select_prev_message))
        .on_action(cx.listener(Workspace::handle_select_first_message))
        .on_action(cx.listener(Workspace::handle_select_last_message))
        .on_action(cx.listener(Workspace::handle_copy_selected_message))
        .on_action(cx.listener(Workspace::handle_bookmark_selected_message))
        // Theme and editor actions
        .on_action(cx.listener(Workspace::handle_toggle_theme))
        .on_action(cx.listener(Workspace::handle_toggle_vim_mode))
        .on_action(cx.listener(Workspace::handle_toggle_word_wrap))
        .on_action(cx.listener(Workspace::handle_toggle_line_numbers))
        .on_action(cx.listener(Workspace::handle_toggle_focus_mode))
        .on_action(cx.listener(Workspace::handle_show_shortcuts))
        // Font size actions
        .on_action(cx.listener(Workspace::handle_increase_font_size))
        .on_action(cx.listener(Workspace::handle_decrease_font_size))
        .on_action(cx.listener(Workspace::handle_reset_font_size))
        // Accessibility actions
        .on_action(cx.listener(Workspace::handle_toggle_high_contrast))
        // View mode actions
        .on_action(cx.listener(Workspace::handle_toggle_compact_mode))
        .on_action(cx.listener(Workspace::handle_toggle_timestamps))
        .on_action(cx.listener(Workspace::handle_toggle_auto_scroll))
        .on_action(cx.listener(Workspace::handle_toggle_bookmarked_filter))
        .on_action(cx.listener(Workspace::handle_cycle_message_filter))
        // Navigation actions
        .on_action(cx.listener(Workspace::handle_scroll_to_top))
        .on_action(cx.listener(Workspace::handle_scroll_to_bottom))
        .on_action(cx.listener(Workspace::handle_dismiss_overlays))
        // Skip link actions (accessibility)
        .on_action(cx.listener(Workspace::handle_skip_to_main))
        .on_action(cx.listener(Workspace::handle_skip_to_input))
        .on_action(cx.listener(Workspace::handle_skip_to_navigation))
        .on_action(cx.listener(Workspace::handle_skip_to_sidebar))
}
