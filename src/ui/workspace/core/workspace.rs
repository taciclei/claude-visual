//! Workspace struct definition and constructor

use std::sync::Arc;
use gpui::*;
use tokio::sync::mpsc;
use crate::app::state::AppState;
use crate::claude::client::ClaudeClient;
use crate::ui::chat::view::ChatView;
use crate::ui::cloud::TeamPanel;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::components::toast::ToastContainer;
use crate::ui::explorer::FileTree;
use crate::ui::sidebar::history::HistorySidebar;
use crate::ui::sidebar::projects::ProjectsSidebar;
use crate::ui::sidebar::worktrees::WorktreePanel;
use crate::ui::split::SplitContainer;
use crate::ui::tabs::TabBar;
use crate::ui::components::command_palette::CommandPalette;
use crate::ui::settings::SettingsModal;
use crate::ui::components::shortcuts_panel::ShortcutsPanel;
use crate::ui::accessibility::skip_links::SkipLinkManager;
use super::super::types::SidebarTab;

/// Main workspace view containing sidebar and chat area
pub struct Workspace {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) projects_sidebar: Entity<ProjectsSidebar>,
    pub(crate) file_tree: Entity<FileTree>,
    pub(crate) history_sidebar: Entity<HistorySidebar>,
    pub(crate) worktree_panel: Entity<WorktreePanel>,
    pub(crate) team_panel: Entity<TeamPanel>,
    /// Tab bar for multi-conversation support
    pub(crate) tab_bar: Entity<TabBar>,
    /// Multiple chat views (one per tab)
    pub(crate) chat_views: Vec<Entity<ChatView>>,
    /// Active chat view index
    pub(crate) active_chat_index: usize,
    pub(crate) claude_client: ClaudeClient,
    pub(crate) show_sidebar: bool,
    pub(crate) sidebar_tab: SidebarTab,
    /// Channel to cancel the current streaming request
    pub(crate) cancel_sender: Option<mpsc::Sender<()>>,
    /// Command palette (shown when Cmd+K is pressed)
    pub(crate) command_palette: Option<Entity<CommandPalette>>,
    /// Settings modal (shown when Cmd+, is pressed)
    pub(crate) settings_modal: Option<Entity<SettingsModal>>,
    /// Shortcuts panel (shown when ? is pressed)
    pub(crate) shortcuts_panel: Option<Entity<ShortcutsPanel>>,
    /// Toast notification container
    pub(crate) toast_container: Entity<ToastContainer>,
    /// Diff preview popup (file path, diff content)
    pub(crate) diff_preview: Option<(String, String)>,
    /// Diff preview display mode (true = side-by-side, false = unified)
    pub(crate) diff_side_by_side: bool,
    /// Syntax highlighter for code
    pub(crate) syntax_highlighter: std::sync::Arc<std::sync::RwLock<crate::syntax::Highlighter>>,
    /// Split view container (optional)
    pub(crate) split_container: Option<Entity<SplitContainer>>,
    /// Whether split view mode is active
    pub(crate) split_mode: bool,
    /// Chat views for split panes (maps pane id to chat view index)
    pub(crate) split_pane_views: std::collections::HashMap<String, usize>,
    /// Focus mode (hides sidebar and tab bar for distraction-free editing)
    pub(crate) focus_mode: bool,
    /// Status bar component
    pub(crate) status_bar: Entity<StatusBar>,
    /// Skip link manager for accessibility
    pub(crate) skip_link_manager: SkipLinkManager,
    /// Focus handle for main content area
    pub(crate) main_focus: FocusHandle,
    /// Focus handle for navigation (tab bar)
    pub(crate) navigation_focus: FocusHandle,
    /// Focus handle for sidebar
    pub(crate) sidebar_focus: FocusHandle,
}

impl Workspace {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let projects_sidebar = cx.new(|cx| ProjectsSidebar::new(app_state.clone(), cx));
        let file_tree = cx.new(|cx| FileTree::new(cx));
        let history_sidebar = cx.new(|cx| HistorySidebar::new(app_state.clone(), cx));
        let worktree_panel = cx.new(|cx| WorktreePanel::new(app_state.clone(), cx));
        let team_panel = cx.new(|cx| TeamPanel::new(app_state.clone(), cx));
        let tab_bar = cx.new(|cx| TabBar::new(app_state.clone(), cx));

        // Create initial chat view
        let chat_view = cx.new(|cx| ChatView::new(app_state.clone(), cx));
        let chat_views = vec![chat_view.clone()];

        let claude_client = ClaudeClient::new();

        let toast_container = cx.new(|cx| ToastContainer::new(app_state.clone(), cx));
        let status_bar = cx.new(|cx| StatusBar::new(app_state.clone(), cx));

        // Create focus handles for skip links
        let main_focus = cx.focus_handle();
        let navigation_focus = cx.focus_handle();
        let sidebar_focus = cx.focus_handle();

        let mut workspace = Self {
            app_state,
            projects_sidebar,
            file_tree,
            history_sidebar,
            worktree_panel,
            team_panel,
            tab_bar,
            chat_views,
            active_chat_index: 0,
            claude_client,
            show_sidebar: true,
            sidebar_tab: SidebarTab::Projects,
            cancel_sender: None,
            command_palette: None,
            settings_modal: None,
            shortcuts_panel: None,
            toast_container,
            diff_preview: None,
            diff_side_by_side: false,
            syntax_highlighter: std::sync::Arc::new(std::sync::RwLock::new(crate::syntax::Highlighter::new())),
            split_container: None,
            split_mode: false,
            split_pane_views: std::collections::HashMap::new(),
            focus_mode: false,
            status_bar,
            skip_link_manager: SkipLinkManager::new(),
            main_focus,
            navigation_focus,
            sidebar_focus,
        };

        // Setup subscriptions
        workspace.subscribe_to_tab_bar(cx);
        workspace.subscribe_to_chat_view(&chat_view, cx);
        workspace.subscribe_to_projects_sidebar(cx);
        workspace.subscribe_to_file_tree(cx);
        workspace.subscribe_to_history_sidebar(cx);
        workspace.subscribe_to_worktree_panel(cx);
        workspace.subscribe_to_team_panel(cx);
        workspace.subscribe_to_status_bar(cx);

        workspace
    }
}
