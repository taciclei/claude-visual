use gpui::*;
use super::*;

/// Register global action handlers
pub fn register_handlers(cx: &mut App) {
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.on_action(|_: &OpenProject, _cx| {
        tracing::info!("Open project requested - file picker would open here");
        // TODO: Implement file picker
    });
    cx.on_action(|_: &OpenCommandPalette, _cx| {
        tracing::info!("Command palette requested - would open command palette here");
        // TODO: Implement command palette
    });
    cx.on_action(|_: &OpenSettings, _cx| {
        tracing::info!("Settings requested");
        // Handled by Workspace
    });

    // Tab actions (handled by Workspace)
    cx.on_action(|_: &NewTab, _cx| {
        tracing::info!("New tab requested");
    });
    cx.on_action(|_: &CloseTab, _cx| {
        tracing::info!("Close tab requested");
    });
    cx.on_action(|_: &NextTab, _cx| {
        tracing::info!("Next tab requested");
    });
    cx.on_action(|_: &PrevTab, _cx| {
        tracing::info!("Previous tab requested");
    });
    cx.on_action(|_: &SelectTab1, _cx| {});
    cx.on_action(|_: &SelectTab2, _cx| {});
    cx.on_action(|_: &SelectTab3, _cx| {});
    cx.on_action(|_: &SelectTab4, _cx| {});
    cx.on_action(|_: &SelectTab5, _cx| {});
    cx.on_action(|_: &SelectTab6, _cx| {});
    cx.on_action(|_: &SelectTab7, _cx| {});
    cx.on_action(|_: &SelectTab8, _cx| {});
    cx.on_action(|_: &SelectTab9, _cx| {});

    // Split view actions (handled by Workspace)
    cx.on_action(|_: &SplitHorizontal, _cx| {
        tracing::info!("Split horizontal requested");
    });
    cx.on_action(|_: &SplitVertical, _cx| {
        tracing::info!("Split vertical requested");
    });
    cx.on_action(|_: &FocusNextPane, _cx| {
        tracing::info!("Focus next pane requested");
    });
    cx.on_action(|_: &FocusPrevPane, _cx| {
        tracing::info!("Focus previous pane requested");
    });
    cx.on_action(|_: &ClosePane, _cx| {
        tracing::info!("Close pane requested");
    });
    cx.on_action(|_: &ToggleVimMode, _cx| {
        tracing::info!("Toggle vim mode requested");
    });
    cx.on_action(|_: &ToggleTheme, _cx| {
        tracing::info!("Toggle theme requested");
    });
    // Chat view actions (handled by Workspace)
    cx.on_action(|_: &ToggleChatSearch, _cx| {
        tracing::info!("Toggle chat search requested");
    });
    cx.on_action(|_: &NextSearchResult, _cx| {
        tracing::info!("Next search result requested");
    });
    cx.on_action(|_: &PrevSearchResult, _cx| {
        tracing::info!("Previous search result requested");
    });
    cx.on_action(|_: &ToggleStats, _cx| {
        tracing::info!("Toggle stats requested");
    });
    cx.on_action(|_: &ExportConversation, _cx| {
        tracing::info!("Export conversation requested");
    });
    cx.on_action(|_: &CopyConversation, _cx| {
        tracing::info!("Copy conversation requested");
    });
    cx.on_action(|_: &ClearConversation, _cx| {
        tracing::info!("Clear conversation requested");
    });
    cx.on_action(|_: &CollapseAllMessages, _cx| {
        tracing::info!("Collapse all messages requested");
    });
    cx.on_action(|_: &ExpandAllMessages, _cx| {
        tracing::info!("Expand all messages requested");
    });
    // Message navigation actions (handled by Workspace)
    cx.on_action(|_: &SelectNextMessage, _cx| {
        tracing::info!("Select next message requested");
    });
    cx.on_action(|_: &SelectPrevMessage, _cx| {
        tracing::info!("Select previous message requested");
    });
    cx.on_action(|_: &SelectFirstMessage, _cx| {
        tracing::info!("Select first message requested");
    });
    cx.on_action(|_: &SelectLastMessage, _cx| {
        tracing::info!("Select last message requested");
    });
    cx.on_action(|_: &CopySelectedMessage, _cx| {
        tracing::info!("Copy selected message requested");
    });
    cx.on_action(|_: &BookmarkSelectedMessage, _cx| {
        tracing::info!("Bookmark selected message requested");
    });
    cx.on_action(|_: &ToggleWordWrap, _cx| {
        tracing::info!("Toggle word wrap requested");
    });
    cx.on_action(|_: &ToggleLineNumbers, _cx| {
        tracing::info!("Toggle line numbers requested");
    });
    cx.on_action(|_: &ToggleFocusMode, _cx| {
        tracing::info!("Toggle focus mode requested");
    });
    cx.on_action(|_: &ShowShortcuts, _cx| {
        tracing::info!("Show shortcuts requested");
    });
    cx.on_action(|_: &IncreaseFontSize, _cx| {
        tracing::info!("Increase font size requested");
    });
    cx.on_action(|_: &DecreaseFontSize, _cx| {
        tracing::info!("Decrease font size requested");
    });
    cx.on_action(|_: &ResetFontSize, _cx| {
        tracing::info!("Reset font size requested");
    });
    cx.on_action(|_: &ToggleHighContrast, _cx| {
        tracing::info!("Toggle high contrast requested");
    });
    cx.on_action(|_: &ToggleCompactMode, _cx| {
        tracing::info!("Toggle compact mode requested");
    });
    cx.on_action(|_: &ToggleTimestamps, _cx| {
        tracing::info!("Toggle timestamps requested");
    });
    cx.on_action(|_: &ToggleAutoScroll, _cx| {
        tracing::info!("Toggle auto-scroll requested");
    });
    cx.on_action(|_: &ToggleBookmarkedFilter, _cx| {
        tracing::info!("Toggle bookmarked filter requested");
    });
    cx.on_action(|_: &CycleMessageFilter, _cx| {
        tracing::info!("Cycle message filter requested");
    });
    cx.on_action(|_: &ScrollToTop, _cx| {
        tracing::info!("Scroll to top requested");
    });
    cx.on_action(|_: &ScrollToBottom, _cx| {
        tracing::info!("Scroll to bottom requested");
    });
    cx.on_action(|_: &DismissOverlays, _cx| {
        tracing::info!("Dismiss overlays requested");
    });
    // Panel toggle actions
    cx.on_action(|_: &ToggleMcpPanel, _cx| {
        tracing::info!("Toggle MCP panel requested");
    });
    cx.on_action(|_: &ToggleTasksPanel, _cx| {
        tracing::info!("Toggle tasks panel requested");
    });
    cx.on_action(|_: &ToggleGitPanel, _cx| {
        tracing::info!("Toggle git panel requested");
    });
    cx.on_action(|_: &ToggleFilePicker, _cx| {
        tracing::info!("Toggle file picker requested");
    });
    cx.on_action(|_: &ToggleBookmarksOnly, _cx| {
        tracing::info!("Toggle bookmarks only requested");
    });
    cx.on_action(|_: &ToggleSessionDetails, _cx| {
        tracing::info!("Toggle session details requested");
    });
    cx.on_action(|_: &ToggleThinking, _cx| {
        tracing::info!("Toggle thinking display requested");
    });
    cx.on_action(|_: &ToggleSessionHistory, _cx| {
        tracing::info!("Toggle session history requested");
    });
    cx.on_action(|_: &ToggleModelSwitcher, _cx| {
        tracing::info!("Toggle model switcher requested");
    });
    cx.on_action(|_: &RegenerateLastResponse, _cx| {
        tracing::info!("Regenerate last response requested");
    });
    cx.on_action(|_: &ContinueResponse, _cx| {
        tracing::info!("Continue response requested");
    });
    cx.on_action(|_: &CopyLastResponse, _cx| {
        tracing::info!("Copy last response requested");
    });
    cx.on_action(|_: &StopStreaming, _cx| {
        tracing::info!("Stop streaming requested");
    });
}
