use super::*;
use gpui::*;

/// Bind keyboard shortcuts
pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-n", NewConversation, None),
        KeyBinding::new("cmd-o", OpenProject, None),
        KeyBinding::new("cmd-b", ToggleSidebar, None),
        KeyBinding::new("cmd-shift-p", OpenCommandPalette, None),
        KeyBinding::new("cmd-k", OpenCommandPalette, None),
        KeyBinding::new("cmd-,", OpenSettings, None),
        // Tab management shortcuts
        KeyBinding::new("cmd-t", NewTab, None),
        KeyBinding::new("cmd-w", CloseTab, None),
        KeyBinding::new("ctrl-tab", NextTab, None),
        KeyBinding::new("ctrl-shift-tab", PrevTab, None),
        KeyBinding::new("cmd-shift-]", NextTab, None),
        KeyBinding::new("cmd-shift-[", PrevTab, None),
        // Number shortcuts for tabs
        KeyBinding::new("cmd-1", SelectTab1, None),
        KeyBinding::new("cmd-2", SelectTab2, None),
        KeyBinding::new("cmd-3", SelectTab3, None),
        KeyBinding::new("cmd-4", SelectTab4, None),
        KeyBinding::new("cmd-5", SelectTab5, None),
        KeyBinding::new("cmd-6", SelectTab6, None),
        KeyBinding::new("cmd-7", SelectTab7, None),
        KeyBinding::new("cmd-8", SelectTab8, None),
        KeyBinding::new("cmd-9", SelectTab9, None),
        // Split view shortcuts
        KeyBinding::new("cmd-\\", SplitHorizontal, None),
        KeyBinding::new("cmd-shift-\\", SplitVertical, None),
        KeyBinding::new("cmd-alt-right", FocusNextPane, None),
        KeyBinding::new("cmd-alt-left", FocusPrevPane, None),
        KeyBinding::new("cmd-shift-w", ClosePane, None),
        // Vim mode toggle
        KeyBinding::new("ctrl-shift-v", ToggleVimMode, None),
        // Theme toggle
        KeyBinding::new("cmd-shift-t", ToggleTheme, None),
        // Chat view shortcuts
        KeyBinding::new("cmd-f", ToggleChatSearch, None),
        KeyBinding::new("cmd-g", NextSearchResult, None),
        KeyBinding::new("cmd-shift-g", PrevSearchResult, None),
        KeyBinding::new("cmd-i", ToggleStats, None),
        KeyBinding::new("cmd-e", ExportConversation, None),
        KeyBinding::new("cmd-shift-c", CopyConversation, None),
        KeyBinding::new("cmd-shift-backspace", ClearConversation, None),
        KeyBinding::new("cmd-[", CollapseAllMessages, None),
        KeyBinding::new("cmd-]", ExpandAllMessages, None),
        // Message navigation shortcuts
        KeyBinding::new("alt-down", SelectNextMessage, None),
        KeyBinding::new("alt-up", SelectPrevMessage, None),
        KeyBinding::new("alt-home", SelectFirstMessage, None),
        KeyBinding::new("alt-end", SelectLastMessage, None),
        KeyBinding::new("alt-c", CopySelectedMessage, None),
        KeyBinding::new("alt-b", BookmarkSelectedMessage, None),
        // View toggle shortcuts
        KeyBinding::new("alt-w", ToggleWordWrap, None),
        KeyBinding::new("alt-l", ToggleLineNumbers, None),
        KeyBinding::new("cmd-shift-f", ToggleFocusMode, None),
        // Keyboard shortcuts help
        KeyBinding::new("shift-/", ShowShortcuts, None), // ? key
        // Font size controls
        KeyBinding::new("cmd-=", IncreaseFontSize, None),
        KeyBinding::new("cmd-+", IncreaseFontSize, None),
        KeyBinding::new("cmd--", DecreaseFontSize, None),
        KeyBinding::new("cmd-0", ResetFontSize, None),
        // Accessibility
        KeyBinding::new("cmd-shift-h", ToggleHighContrast, None),
        // Skip links for keyboard navigation
        KeyBinding::new("alt-1", SkipToMain, None),
        KeyBinding::new("alt-2", SkipToInput, None),
        KeyBinding::new("alt-3", SkipToNavigation, None),
        KeyBinding::new("alt-4", SkipToSidebar, None),
        // View modes
        KeyBinding::new("alt-c", ToggleCompactMode, None),
        KeyBinding::new("alt-t", ToggleTimestamps, None),
        KeyBinding::new("alt-s", ToggleAutoScroll, None),
        KeyBinding::new("alt-shift-b", ToggleBookmarkedFilter, None),
        KeyBinding::new("alt-f", CycleMessageFilter, None),
        // Quick navigation
        KeyBinding::new("cmd-home", ScrollToTop, None),
        KeyBinding::new("cmd-end", ScrollToBottom, None),
        // Panel shortcuts
        KeyBinding::new("cmd-shift-m", ToggleMcpPanel, None),
        KeyBinding::new("cmd-shift-a", ToggleTasksPanel, None),
        KeyBinding::new("cmd-g", ToggleGitPanel, None),
        KeyBinding::new("cmd-p", ToggleFilePicker, None),
        KeyBinding::new("cmd-d", ToggleBookmarksOnly, None),
        // Session shortcuts
        KeyBinding::new("alt-s", ToggleSessionDetails, None),
        KeyBinding::new("alt-t", ToggleThinking, None),
        // Session management
        KeyBinding::new("cmd-shift-h", ToggleSessionHistory, None),
        KeyBinding::new("cmd-m", ToggleModelSwitcher, None),
        // Response actions
        KeyBinding::new("cmd-shift-r", RegenerateLastResponse, None),
        KeyBinding::new("alt-shift-c", CopyLastResponse, None),
        KeyBinding::new("cmd-.", StopStreaming, None),
        // Clear toasts
        KeyBinding::new("escape", DismissOverlays, None),
    ]);
}
