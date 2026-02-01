mod handlers;
mod keybindings;

use gpui::*;

pub use handlers::register_handlers;
pub use keybindings::bind_keys;

/// Register global application actions
pub fn register_actions(cx: &mut App) {
    bind_keys(cx);
    register_handlers(cx);
}

// Global actions (unit actions without data)
actions!(
    claude_visual,
    [
        Quit,
        NewConversation,
        OpenProject,
        ToggleSidebar,
        OpenCommandPalette,
        OpenSettings,
        // Tab management actions
        NewTab,
        CloseTab,
        NextTab,
        PrevTab,
        SelectTab1,
        SelectTab2,
        SelectTab3,
        SelectTab4,
        SelectTab5,
        SelectTab6,
        SelectTab7,
        SelectTab8,
        SelectTab9,
        // Split view actions
        SplitHorizontal,
        SplitVertical,
        FocusNextPane,
        FocusPrevPane,
        ClosePane,
        // Vim mode
        ToggleVimMode,
        // Theme
        ToggleTheme,
        // Update
        CheckForUpdates,
        // Chat view actions
        ToggleChatSearch,
        NextSearchResult,
        PrevSearchResult,
        ToggleStats,
        ExportConversation,
        CopyConversation,
        ClearConversation,
        CollapseAllMessages,
        ExpandAllMessages,
        // Message navigation
        SelectNextMessage,
        SelectPrevMessage,
        SelectFirstMessage,
        SelectLastMessage,
        CopySelectedMessage,
        BookmarkSelectedMessage,
        // View toggles
        ToggleWordWrap,
        ToggleLineNumbers,
        ToggleFocusMode,
        // Help
        ShowShortcuts,
        // Font size
        IncreaseFontSize,
        DecreaseFontSize,
        ResetFontSize,
        // Accessibility
        ToggleHighContrast,
        // View modes
        ToggleCompactMode,
        ToggleTimestamps,
        ToggleAutoScroll,
        ToggleBookmarkedFilter,
        CycleMessageFilter,
        // Quick navigation
        ScrollToTop,
        ScrollToBottom,
        DismissOverlays,
        // Panel toggles
        ToggleMcpPanel,
        ToggleTasksPanel,
        ToggleGitPanel,
        ToggleFilePicker,
        ToggleBookmarksOnly,
        ToggleSessionDetails,
        ToggleThinking,
        // Session management
        ToggleSessionHistory,
        ToggleModelSwitcher,
        // Response actions
        RegenerateLastResponse,
        ContinueResponse,
        CopyLastResponse,
        StopStreaming,
        // Accessibility skip links (Alt+1-4)
        SkipToMain,
        SkipToInput,
        SkipToNavigation,
        SkipToSidebar
    ]
);

/// Action to execute code (carries the code string)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct ExecuteCodeAction {
    pub code: String,
}

/// Action to save code to a file (carries the code string)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct SaveCodeToFileAction {
    pub code: String,
}

/// Action to explain code (sends to Claude for explanation)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct ExplainCodeAction {
    pub code: String,
    pub language: Option<String>,
}

/// Action to improve/refactor code (sends to Claude for improvement)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct ImproveCodeAction {
    pub code: String,
    pub language: Option<String>,
}

/// Action to add tests for code (sends to Claude for test generation)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct AddTestsAction {
    pub code: String,
    pub language: Option<String>,
}

/// Action to review code (sends to Claude for code review using /review skill)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct ReviewCodeAction {
    pub code: String,
    pub language: Option<String>,
}

/// Action to refactor code (sends to Claude for refactoring using /refactor skill)
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    gpui::private::schemars::JsonSchema,
    gpui::Action,
)]
pub struct RefactorCodeAction {
    pub code: String,
    pub language: Option<String>,
}
