//! Main chat view

use std::sync::Arc;

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;
use crate::claude::message::{ClaudeEvent, ClaudeMessage, MessageRole};
use crate::storage::models::{Conversation, Message};
use crate::ui::pct;

use crate::ui::chat::input::{ChatInput, ChatInputEvent};
use crate::ui::chat::message::{MessageView, MessageViewEvent};

use super::suggestions::{ContextualSuggestion, QuickReplySuggestion};
use super::types::*;

impl EventEmitter<ChatViewEvent> for ChatView {}

/// Chat view containing messages and input
pub struct ChatView {
    pub(crate) app_state: Arc<AppState>,
    /// Message entities (stateful, collapsible)
    pub(crate) message_views: Vec<Entity<MessageView>>,
    /// Raw messages for persistence
    pub(crate) messages: Vec<ClaudeMessage>,
    pub(crate) input: Entity<ChatInput>,
    pub(crate) is_streaming: bool,
    /// Streaming message view (temporary during streaming)
    pub(crate) streaming_message_view: Option<Entity<MessageView>>,
    /// Current conversation ID (if saved)
    pub(crate) current_conversation_id: Option<String>,
    /// Whether to show the stats bar
    pub(crate) show_stats: bool,
    /// Search query
    pub(crate) search_query: String,
    /// Whether search panel is visible
    pub(crate) show_search: bool,
    /// Current search results
    pub(crate) search_results: Vec<ConversationSearchResult>,
    /// Currently highlighted result index
    pub(crate) current_search_result: usize,
    /// Whether search is case-sensitive
    pub(crate) search_case_sensitive: bool,
    /// Whether search uses regex
    pub(crate) search_regex: bool,
    /// Role filter for search (which roles to search in)
    pub(crate) search_role_filter: MessageFilter,
    /// Whether user has scrolled away from bottom
    pub(crate) show_scroll_to_bottom: bool,
    /// Number of unread messages since scrolling away
    pub(crate) unread_count: usize,
    /// Current message filter
    pub(crate) message_filter: MessageFilter,
    /// Whether to show timestamps on messages
    pub(crate) show_timestamps: bool,
    /// Whether to show compact mode (less padding)
    pub(crate) compact_mode: bool,
    /// Whether to show time separators between message groups
    pub(crate) show_time_separators: bool,
    /// Conversation title (user-editable)
    pub(crate) conversation_title: Option<String>,
    /// Whether title is being edited
    pub(crate) editing_title: bool,
    /// Title edit buffer (temporary storage during editing)
    pub(crate) title_edit_buffer: String,
    /// Focus handle for title editing
    pub(crate) title_focus: FocusHandle,
    /// Whether to auto-scroll to bottom on new messages
    pub(crate) auto_scroll: bool,
    /// Currently selected message index for keyboard navigation
    pub(crate) selected_message_index: Option<usize>,
    /// Whether to show only bookmarked messages
    pub(crate) show_bookmarked_only: bool,
    /// Whether to wrap long lines in code blocks
    pub(crate) word_wrap: bool,
    /// Whether to show line numbers in code blocks
    pub(crate) show_line_numbers: bool,
    /// Session info from Claude CLI (commands, tools, etc.)
    pub(crate) session_info: Option<crate::claude::message::SessionInfo>,
    /// Session usage stats
    pub(crate) session_cost: f64,
    pub(crate) session_input_tokens: u64,
    pub(crate) session_output_tokens: u64,
    /// Active tasks/subagents
    pub(crate) active_tasks: Vec<ActiveTask>,
    /// Connection status
    pub(crate) connection_status: ConnectionStatus,
    /// Current notifications (toast-style)
    pub(crate) notifications: Vec<Notification>,
    /// Whether to show keyboard shortcuts help panel
    pub(crate) show_shortcuts_help: bool,
    /// Streaming progress animation frame (for smooth animation)
    pub(crate) streaming_dots: usize,
    /// Whether to show command palette
    pub(crate) show_command_palette: bool,
    /// Command palette search query
    pub(crate) palette_query: String,
    /// Selected index in command palette
    pub(crate) palette_selected_index: usize,
    /// Recently used commands (most recent first, max 10)
    pub(crate) recent_commands: Vec<String>,
    /// Last response time in milliseconds (for performance tracking)
    pub(crate) last_response_time_ms: Option<u64>,
    /// Response start time for measuring response duration
    pub(crate) response_start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether to show model switcher dropdown
    show_model_switcher: bool,
    /// Saved draft text (auto-saved input)
    pub(crate) draft_text: Option<String>,
    /// Time when draft was last saved
    pub(crate) draft_saved_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Context capacity in tokens (from Claude CLI)
    pub(crate) context_capacity: u64,
    /// Context used in tokens (estimated)
    pub(crate) context_used: u64,
    /// Available models for switching
    pub(crate) available_models: Vec<ModelInfo>,
    /// Contextual suggestions based on current state
    pub(crate) contextual_suggestions: Vec<ContextualSuggestion>,
    /// Whether to show suggestions bar
    pub(crate) show_suggestions: bool,
    /// Whether to show floating action button menu
    pub(crate) show_fab_menu: bool,
    /// Recent sessions for quick resume
    pub(crate) recent_sessions: Vec<RecentSession>,
    /// Whether to show session history panel
    pub(crate) show_session_history: bool,
    /// Last error message (for retry functionality)
    pub(crate) last_error: Option<ErrorInfo>,
    /// Pending permissions requests
    pub(crate) pending_permissions: Vec<PermissionRequest>,
    /// Whether to show permissions panel
    show_permissions_panel: bool,
    /// Whether to show MCP servers panel
    pub(crate) show_mcp_panel: bool,
    /// Whether to show active tasks panel
    pub(crate) show_tasks_panel: bool,
    /// Git information for current project
    pub(crate) git_info: Option<GitInfo>,
    /// Whether to show git panel
    pub(crate) show_git_panel: bool,
    /// File picker state
    pub(crate) file_picker_visible: bool,
    /// File picker query
    pub(crate) file_picker_query: String,
    /// File picker results
    pub(crate) file_picker_results: Vec<FilePickerItem>,
    /// Bookmarked message indices
    pub(crate) bookmarked_messages: std::collections::HashSet<usize>,
    /// Show only bookmarked messages
    pub(crate) show_bookmarks_only: bool,
    /// Multiline input mode
    pub(crate) multiline_input: bool,
    /// Input height in lines (for multiline mode)
    pub(crate) input_height_lines: u32,
    /// Show detailed session panel
    pub(crate) show_session_details: bool,
    /// Streaming tokens count (for speed calculation)
    pub(crate) streaming_token_count: usize,
    /// Last streaming speed (tokens per second)
    pub(crate) last_streaming_speed: f64,
    /// Peak streaming speed (tokens per second)
    pub(crate) peak_streaming_speed: f64,
    /// Total API requests made this session
    pub(crate) total_api_requests: u32,
    /// Show thinking/reasoning content
    pub(crate) show_thinking: bool,
    /// Current thinking content being streamed
    pub(crate) current_thinking: Option<String>,
    /// Currently executing tool name (for display)
    pub(crate) current_tool_name: Option<String>,
    /// Expanded MCP servers in panel (by name)
    pub(crate) expanded_mcp_servers: std::collections::HashSet<String>,
    /// Cached MCP server tools (server name -> tool names)
    pub(crate) mcp_server_tools: std::collections::HashMap<String, Vec<String>>,
    /// Recently used MCP tools (most recent first)
    pub(crate) recent_mcp_tools: Vec<String>,
    /// Favorite MCP tools for quick access
    pub(crate) favorite_mcp_tools: Vec<String>,
    /// Whether to show commands panel (slash commands + skills)
    pub(crate) show_commands_panel: bool,
    /// Commands panel search filter
    pub(crate) commands_filter: String,
    /// Selected command category filter
    pub(crate) commands_category: CommandCategory,
    /// Currently hovered skill for preview
    pub(crate) hovered_skill: Option<String>,
    /// Whether to show prompt templates panel
    pub(crate) show_templates_panel: bool,
    /// Saved prompt templates
    pub(crate) prompt_templates: Vec<PromptTemplate>,
    /// Templates panel search filter
    pub(crate) templates_filter: String,
    /// Whether to show context panel (files, tools, etc.)
    pub(crate) show_context_panel: bool,
    /// Files mentioned/attached in this session
    pub(crate) context_files: Vec<ContextFile>,
    /// Whether to show export panel
    pub(crate) show_export_panel: bool,
    /// Selected export format
    pub(crate) export_format: ExportFormat,
    /// Whether to include metadata in export
    pub(crate) export_include_metadata: bool,
    /// Whether to include tool calls in export
    pub(crate) export_include_tools: bool,
    /// Whether to include thinking in export
    pub(crate) export_include_thinking: bool,
    /// Pinned message indices
    pub(crate) pinned_messages: std::collections::HashSet<usize>,
    /// Edit history stack (for undo)
    edit_history: Vec<EditHistoryEntry>,
    /// Current position in edit history
    edit_history_position: usize,
    /// Message reactions (message index -> list of reactions)
    pub(crate) message_reactions: std::collections::HashMap<usize, Vec<MessageReaction>>,
    /// Session notes
    pub(crate) session_notes: String,
    /// Whether to show notes panel
    pub(crate) show_notes_panel: bool,
    /// Quick action being shown (message index, None if hidden)
    pub(crate) quick_action_message: Option<usize>,
    /// Context menu state (message index, position)
    pub(crate) context_menu: Option<ContextMenuState>,
    /// Favorite prompts (quick access)
    pub(crate) favorite_prompts: Vec<FavoritePrompt>,
    /// Whether to show favorites panel
    pub(crate) show_favorites_panel: bool,
    /// Input focus mode (distraction-free)
    pub(crate) focus_mode: bool,
    /// Conversation tags
    pub(crate) conversation_tags: Vec<String>,
    /// Whether to show tags editor
    pub(crate) show_tags_editor: bool,
    /// Available quick reactions
    quick_reactions: Vec<&'static str>,
    /// Whether to show pinned messages panel
    pub(crate) show_pinned_panel: bool,
    /// Whether to show quick settings panel
    pub(crate) show_quick_settings: bool,
    /// Whether to show statistics panel
    pub(crate) show_stats_panel: bool,
    /// Input history state (for navigating previous inputs)
    pub(crate) input_history: InputHistoryState,
    /// Show input hints/suggestions
    pub(crate) show_input_hints: bool,
    /// Recently accessed files for quick access
    pub(crate) recent_files: Vec<RecentFile>,
    /// Whether to show recent files panel
    pub(crate) show_recent_files_panel: bool,
    /// Maximum recent files to track
    pub(crate) max_recent_files: usize,
    /// Highlighted message index (for search jump animation, with timestamp)
    pub(crate) highlighted_message: Option<(usize, std::time::Instant)>,
    /// Whether to show message preview on hover
    show_message_preview: bool,
    /// Quick navigation history (for back/forward navigation)
    pub(crate) navigation_history: Vec<usize>,
    /// Current position in navigation history
    pub(crate) navigation_history_position: usize,
    /// Whether file is being dragged over the view
    pub(crate) file_drag_active: bool,
    /// Number of files being dragged
    pub(crate) drag_file_count: usize,
    /// Session health score (0.0 to 1.0)
    pub(crate) session_health: f32,
    /// Last health check time
    pub(crate) last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether to show onboarding tips
    show_onboarding_tips: bool,
    /// Dismissed tips (by ID)
    pub(crate) dismissed_tips: std::collections::HashSet<&'static str>,
    /// Quick reply suggestions based on context
    pub(crate) quick_reply_suggestions: Vec<QuickReplySuggestion>,
    /// Whether to show typing indicators
    show_typing_indicator: bool,
    /// Connection retry count
    pub(crate) connection_retry_count: u32,
    /// Last successful response time (for latency tracking)
    pub(crate) last_response_latency_ms: Option<u64>,
    /// Average response latency (rolling average)
    pub(crate) avg_response_latency_ms: f64,
    /// Whether extended thinking mode is enabled
    pub(crate) think_mode_enabled: bool,

    // ==================== Nested State Structs ====================
    // These provide organized access to related fields for module code
    /// Streaming state (grouped for module access)
    pub(crate) streaming: StreamingState,
    /// Search state (grouped for module access)
    pub(crate) search: SearchState,
    /// Panel visibility state (grouped for module access)
    pub(crate) panels: PanelVisibility,
    /// Command palette state (grouped for module access)
    pub(crate) palette: CommandPaletteState,
    /// Export settings (grouped for module access)
    pub(crate) export: ExportSettings,
    /// Session statistics (grouped for module access)
    pub(crate) stats: SessionStats,
    /// File picker state (grouped for module access)
    pub(crate) file_picker: FilePickerState,
}

impl ChatView {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Load draft from settings if available
        let saved_draft = app_state.settings.read(cx).draft_text.clone();

        let input = cx.new(|cx| {
            let mut input = ChatInput::new(app_state.clone(), cx);
            // Set default slash commands (these are common Claude CLI commands)
            input.set_available_commands(
                vec![
                    // Help & Info
                    "help".to_string(),
                    "status".to_string(),
                    "usage".to_string(),
                    "cost".to_string(),
                    // Context management
                    "clear".to_string(),
                    "compact".to_string(),
                    "context".to_string(),
                    "add-dir".to_string(),
                    // Session management
                    "resume".to_string(),
                    "memory".to_string(),
                    // Model & AI
                    "model".to_string(),
                    "think".to_string(),
                    // Development
                    "review".to_string(),
                    "commit".to_string(),
                    "pr".to_string(),
                    // Settings & Config
                    "permissions".to_string(),
                    "config".to_string(),
                    "vim".to_string(),
                    // Troubleshooting
                    "doctor".to_string(),
                    "bug".to_string(),
                    // Project setup
                    "init".to_string(),
                    "terminal-setup".to_string(),
                    // MCP
                    "mcp".to_string(),
                    // Auth
                    "login".to_string(),
                    "logout".to_string(),
                ],
                cx,
            );
            // Restore draft if available
            if !saved_draft.is_empty() {
                input.set_text(saved_draft.clone(), cx);
            }
            input
        });

        // Subscribe to input events
        cx.subscribe(&input, |this, _, event: &ChatInputEvent, cx| {
            match event {
                ChatInputEvent::Submit(text) => {
                    cx.emit(ChatViewEvent::Submit(text.clone()));
                }
                ChatInputEvent::FilesAttached(files) => {
                    // Track attached files in context and recent files
                    for file in files {
                        let path = file.to_string_lossy().to_string();
                        this.track_recent_file(path.clone(), cx);
                        this.add_context_file(path.clone(), cx);
                        cx.emit(ChatViewEvent::FileAttached(path));
                    }
                }
                ChatInputEvent::MentionPartial(_partial) => {
                    // Handle partial mention
                }
                ChatInputEvent::ToggleThinkMode => {
                    this.toggle_think_mode(cx);
                    // Sync state back to input
                    let enabled = this.think_mode_enabled;
                    this.input.update(cx, |input, cx| {
                        input.set_think_mode(enabled, cx);
                    });
                }
                ChatInputEvent::ClearConversation => {
                    // Clear the conversation (Ctrl+L)
                    this.clear_conversation(cx);
                }
                ChatInputEvent::OpenHistorySearch => {
                    // Open reverse history search (Ctrl+R)
                    this.toggle_history_search(cx);
                }
            }
        })
        .detach();

        Self {
            app_state,
            message_views: Vec::new(),
            messages: Vec::new(),
            input,
            is_streaming: false,
            streaming_message_view: None,
            current_conversation_id: None,
            show_stats: true, // Show by default
            search_query: String::new(),
            show_search: false,
            search_results: Vec::new(),
            current_search_result: 0,
            search_case_sensitive: false,
            search_regex: false,
            search_role_filter: MessageFilter::All,
            show_scroll_to_bottom: false,
            unread_count: 0,
            message_filter: MessageFilter::All,
            show_timestamps: true, // Show by default
            compact_mode: false,
            show_time_separators: true, // Show time group separators by default
            conversation_title: None,
            editing_title: false,
            title_edit_buffer: String::new(),
            title_focus: cx.focus_handle(),
            auto_scroll: true, // Auto-scroll by default
            selected_message_index: None,
            show_bookmarked_only: false,
            word_wrap: false,        // No word wrap by default (horizontal scroll)
            show_line_numbers: true, // Show line numbers by default
            session_info: None,
            session_cost: 0.0,
            session_input_tokens: 0,
            session_output_tokens: 0,
            active_tasks: Vec::new(),
            connection_status: ConnectionStatus::Disconnected,
            notifications: Vec::new(),
            show_shortcuts_help: false,
            streaming_dots: 0,
            show_command_palette: false,
            palette_query: String::new(),
            palette_selected_index: 0,
            recent_commands: Vec::new(),
            last_response_time_ms: None,
            response_start_time: None,
            show_model_switcher: false,
            draft_text: if saved_draft.is_empty() {
                None
            } else {
                Some(saved_draft)
            },
            draft_saved_at: None,
            context_capacity: 200_000, // Default Claude context window
            context_used: 0,
            available_models: ModelInfo::available_models(),
            contextual_suggestions: Vec::new(),
            show_suggestions: true, // Show suggestions by default
            show_fab_menu: false,
            recent_sessions: Vec::new(),
            show_session_history: false,
            last_error: None,
            pending_permissions: Vec::new(),
            show_permissions_panel: false,
            show_mcp_panel: false,
            show_tasks_panel: false,
            git_info: None,
            show_git_panel: false,
            file_picker_visible: false,
            file_picker_query: String::new(),
            file_picker_results: Vec::new(),
            bookmarked_messages: std::collections::HashSet::new(),
            show_bookmarks_only: false,
            multiline_input: false,
            input_height_lines: 3,
            show_session_details: false,
            streaming_token_count: 0,
            last_streaming_speed: 0.0,
            peak_streaming_speed: 0.0,
            total_api_requests: 0,
            show_thinking: true, // Show thinking by default
            current_thinking: None,
            current_tool_name: None,
            expanded_mcp_servers: std::collections::HashSet::new(),
            mcp_server_tools: std::collections::HashMap::new(),
            recent_mcp_tools: Vec::new(),
            favorite_mcp_tools: Vec::new(),
            show_commands_panel: false,
            commands_filter: String::new(),
            commands_category: CommandCategory::All,
            hovered_skill: None,
            show_templates_panel: false,
            prompt_templates: default_prompt_templates(),
            templates_filter: String::new(),
            show_context_panel: false,
            context_files: Vec::new(),
            show_export_panel: false,
            export_format: ExportFormat::default(),
            export_include_metadata: true,
            export_include_tools: true,
            export_include_thinking: false,
            pinned_messages: std::collections::HashSet::new(),
            edit_history: Vec::new(),
            edit_history_position: 0,
            message_reactions: std::collections::HashMap::new(),
            session_notes: String::new(),
            show_notes_panel: false,
            quick_action_message: None,
            context_menu: None,
            favorite_prompts: Vec::new(),
            show_favorites_panel: false,
            focus_mode: false,
            conversation_tags: Vec::new(),
            show_tags_editor: false,
            quick_reactions: QUICK_REACTIONS.to_vec(),
            show_pinned_panel: false,
            show_quick_settings: false,
            show_stats_panel: false,
            input_history: InputHistoryState {
                history: Vec::new(),
                position: -1,
                temp: String::new(),
                max_size: 50,
            },
            show_input_hints: true,
            recent_files: Vec::new(),
            show_recent_files_panel: false,
            max_recent_files: 20,
            highlighted_message: None,
            show_message_preview: true,
            navigation_history: Vec::new(),
            navigation_history_position: 0,
            file_drag_active: false,
            drag_file_count: 0,
            session_health: 1.0,
            last_health_check: None,
            show_onboarding_tips: true,
            dismissed_tips: std::collections::HashSet::new(),
            quick_reply_suggestions: Vec::new(),
            show_typing_indicator: true,
            connection_retry_count: 0,
            last_response_latency_ms: None,
            avg_response_latency_ms: 0.0,
            think_mode_enabled: false,
            // Nested state structs (default initialization)
            streaming: StreamingState::default(),
            search: SearchState::default(),
            panels: PanelVisibility::default(),
            palette: CommandPaletteState::default(),
            export: ExportSettings::default(),
            stats: SessionStats::default(),
            file_picker: FilePickerState::default(),
        }
    }

    // track_recent_command, start_response_timer, stop_response_timer moved to vim_fab.rs

    // toggle_model_switcher moved to models.rs
    // current_session_id method moved to session.rs
    // select_model moved to models.rs

    // Draft methods moved to draft.rs:
    // - save_draft, clear_draft, get_draft, has_draft
    // - restore_draft_from_settings, sync_draft_to_settings

    // update_context_usage moved to context_usage.rs
    // context_usage_percentage method moved to session.rs
    // context_status_color moved to context_usage.rs
    // context_warning_message moved to context_usage.rs
    // format_context_usage moved to context_usage.rs

    // ==================== Contextual Suggestions ====================
    // Moved to suggestions.rs: update_suggestions, toggle_suggestions, use_suggestion
    // toggle_fab_menu moved to vim_fab.rs
    // Moved to suggestions.rs: get_smart_commands
    // get_command_description moved to commands_logic.rs
    // Moved to suggestions.rs: get_contextual_placeholder, get_quick_actions

    // ==================== Session History ====================
    // Session history methods moved to session.rs:
    // - toggle_session_history
    // - add_recent_session
    // - resume_session
    // - resume_last_session

    // send_slash_command moved to commands_logic.rs

    // ==================== Error Handling ====================
    // Error handling methods moved to errors.rs:
    // - record_error, can_retry_last_error, retry_last_error
    // - get_last_error, clear_last_error

    // Moved to errors.rs

    // Moved to errors.rs

    // Moved to errors.rs

    // Moved to errors.rs

    // Moved to errors.rs

    // Git commands moved to git/commands.rs:
    // - send_tool_command, request_code_review, create_pr
    // - show_pr_comments, show_status
    // show_cost method moved to session.rs

    // enable_think_mode moved to thinking.rs
    // disable_think_mode moved to thinking.rs
    // toggle_think_mode moved to thinking.rs
    // is_think_mode_enabled moved to thinking.rs

    // ==================== Permissions ====================
    // toggle_permissions_panel moved to permissions.rs
    // add_permission_request moved to permissions.rs
    // approve_permission moved to permissions.rs
    // deny_permission moved to permissions.rs
    // approve_all_permissions moved to permissions.rs
    // deny_all_permissions moved to permissions.rs
    // has_pending_permissions moved to permissions.rs

    // ==================== MCP Servers ====================
    // toggle_mcp_panel moved to mcp.rs
    // mcp_server_count moved to mcp.rs
    // toggle_mcp_server_expanded moved to mcp.rs
    // is_mcp_server_expanded moved to mcp.rs
    // get_mcp_server_tools moved to mcp.rs
    // use_mcp_tool moved to mcp.rs

    // ==================== Commands Panel ====================
    // toggle_commands_panel moved to commands_logic.rs
    // set_commands_filter moved to commands_logic.rs
    // set_commands_category moved to commands_logic.rs
    // filtered_commands moved to commands_logic.rs
    // use_slash_command moved to commands_logic.rs
    // use_skill moved to commands_logic.rs

    // ==================== Prompt Templates ====================
    // toggle_templates_panel moved to templates.rs
    // set_templates_filter moved to templates.rs
    // filtered_templates moved to templates.rs
    // use_template moved to templates.rs
    // save_as_template moved to templates.rs
    // delete_template moved to templates.rs
    // templates_by_category moved to templates.rs

    // ==================== Context Panel ====================
    // Context panel methods moved to context.rs:
    // - toggle_context_panel
    // - add_context_file, remove_context_file, clear_context_files
    // - context_files_tokens, context_file_count

    // ==================== Recent Files ====================
    // Recent files methods moved to context.rs:
    // - toggle_recent_files_panel
    // - track_recent_file
    // - recent_files_count, clear_recent_files
    // - add_recent_file_to_context
    // - insert_file_mention

    // ==================== Active Tasks ====================
    // toggle_tasks_panel moved to commands_logic.rs
    // active_task_count moved to commands_logic.rs
    // update_task_progress moved to commands_logic.rs
    // cancel_task moved to commands_logic.rs

    // ==================== Git Integration ====================
    // Git integration methods moved to git/integration.rs:
    // - toggle_git_panel, update_git_info, clear_git_info, refresh_git_status

    // ==================== File Picker ====================
    // File picker methods moved to context.rs:
    // - toggle_file_picker
    // - update_file_picker_query
    // - select_file

    // ==================== Bookmarks ====================
    // Bookmark methods moved to messages.rs:
    // - toggle_bookmark, is_bookmarked
    // - jump_to_next_bookmark, jump_to_prev_bookmark
    // - get_bookmarked_indices, get_bookmarked_messages
    // - bookmarked_count, toggle_bookmarked_only
    // - toggle_bookmarked_filter, is_bookmarked_filter_active
    // - bookmarked_message_count

    // Input handlers moved to handlers.rs:
    // - toggle_multiline_input, increase_input_height, decrease_input_height
    // - toggle_input_hints

    // toggle_session_details method moved to session.rs

    // toggle_thinking moved to thinking.rs

    // Streaming metrics methods moved to claude_events.rs:
    // - update_streaming_speed
    // - reset_streaming_metrics
    // format_streaming_speed moved to streaming.rs

    // get_palette_commands moved to commands/definitions.rs
    // filter_palette_commands moved to commands/filtering.rs
    // toggle_command_palette moved to commands/palette.rs
    // show_notification moved to notifications.rs

    // Palette event handlers moved to handlers.rs:
    // - handle_palette_key, palette_type_char, palette_backspace
    // - palette_select_prev, palette_select_next, palette_execute_selected
    // execute_palette_command moved to commands/executor.rs

    // Keyboard shortcuts help handlers moved to handlers.rs:
    // - toggle_shortcuts_help, is_shortcuts_help_visible

    // get_streaming_dots moved to streaming.rs
    // get_tool_icon moved to streaming.rs

    // set_file_drag_active moved to handlers.rs (see mouse/drag handlers)

    // Session health methods moved to session.rs:
    // - calculate_session_health
    // - get_session_health
    // - session_health_label
    // - session_health_color

    // get_response_latency_ms moved to context_usage.rs
    // Moved to suggestions.rs: generate_quick_reply_suggestions

    // Tips methods moved to tips.rs:
    // - dismiss_tip, dismiss_tip_by_string
    // - should_show_tip, get_recommended_workflow_action, get_contextual_pro_tip
    // update_response_latency moved to context_usage.rs

    // Streaming animation method moved to claude_events.rs:
    // - start_streaming_animation

    // insert_mention moved to selection.rs

    // Slash command methods moved to context.rs:
    // - insert_slash_command

    // Scroll event handlers moved to handlers.rs:
    // - on_scroll_away, scroll_to_bottom, scroll_to_search_result
    // - toggle_auto_scroll, set_auto_scroll, is_auto_scroll

    // increment_unread moved to selection.rs
    // unread_count moved to selection.rs

    // Toggle methods moved to toggles.rs:
    // - toggle_stats, set_show_stats
    // - collapse_all, expand_all, are_all_collapsed, are_all_expanded
    // - collapse_tool_messages, expand_tool_messages, toggle_collapse_tool_messages
    // - collapse_assistant_messages, expand_assistant_messages, tool_message_count
    // - toggle_timestamps, set_show_timestamps, timestamps_visible
    // - toggle_compact_mode, set_compact_mode, is_compact_mode
    // - toggle_time_separators, is_time_separators_enabled
    // - toggle_word_wrap, is_word_wrap_enabled
    // - toggle_line_numbers, is_line_numbers_enabled

    // toggle_search moved to search.rs

    // Message filtering methods moved to messages.rs:
    // - message_filter, set_message_filter, next_filter
    // - filtered_message_views, filtered_message_views_with_indices
    // - visible_message_count, message_count_for_filter

    // Message group methods moved to messages.rs:
    // - time_group_label (private helper)
    // - messages_with_time_groups

    // Auto-scroll handler moved to handlers.rs (see scroll handlers)

    // toggle_vim_mode moved to vim_fab.rs
    // is_vim_mode_enabled moved to vim_fab.rs
    // messages_len moved to message_ops.rs

    // Auto-scroll methods moved to handlers.rs (see scroll handlers)

    // Message selection methods moved to messages.rs:
    // - selected_message_index, selected_message_position
    // - select_message, select_next_message, select_prev_message
    // - update_message_selection_states
    // select_first_message moved to selection.rs
    // select_last_message moved to selection.rs
    // clear_message_selection moved to selection.rs
    // copy_selected_message moved to selection.rs
    // bookmark_selected_message moved to selection.rs
    // is_selected_bookmarked moved to selection.rs
    // has_selected_message moved to selection.rs

    // Conversation title methods moved to session.rs:
    // - conversation_title
    // - set_conversation_title
    // - display_title
    // start_editing_title moved to selection.rs
    // cancel_editing_title moved to selection.rs
    // save_edited_title moved to selection.rs
    // title_edit_buffer moved to selection.rs
    // set_title_edit_buffer moved to selection.rs
    // Title edit handlers moved to handlers.rs:
    // - handle_title_key_down, handle_title_input
    // is_editing_title moved to selection.rs
    // set_search_query moved to search.rs
    // perform_search moved to search_logic.rs
    // next_search_result moved to search.rs
    // prev_search_result moved to search.rs
    // current_result moved to search.rs
    // search_result_count moved to search.rs
    // current_result_index moved to search.rs
    // scroll_to_search_result moved to handlers.rs (see scroll handlers)
    // jump_to_search_result moved to search.rs
    // Message navigation methods moved to messages.rs:
    // - navigate_back, navigate_forward
    // - can_navigate_back, can_navigate_forward
    // - message_highlight_opacity
    // clear_highlight moved to vim_fab.rs
    // toggle_search_case_sensitive moved to search.rs
    // toggle_search_regex moved to search.rs
    // set_search_role_filter moved to search.rs
    // cycle_search_role_filter moved to search.rs

    // Conversation save/load methods moved to session.rs:
    // - ensure_conversation
    // - save_message

    // create_message_view moved to streaming.rs

    // Conversation load/add methods moved to session.rs:
    // - load_conversation
    // - add_message

    // request_stop moved to context_usage.rs

    // request_export moved to context_usage.rs
    // request_theme_toggle moved to context_usage.rs

    // Conversation clearing methods moved to session.rs:
    // - clear_conversation
    // - clear_messages
    // - clear

    // Claude event handling method moved to claude_events.rs:
    // - handle_claude_event - Main event dispatcher for Claude CLI streaming events

    // conversation_id method moved to session.rs

    // Export methods moved to session.rs:
    // - export_to_markdown, export_to_json, export_to_html
    // - export_to_plain_text, export_with_format, _export_metadata
    // (placeholder methods also in message_ops.rs)

    // toggle_export_panel moved to context_usage.rs
    // set_export_format moved to context_usage.rs
    // toggle_export_metadata moved to context_usage.rs
    // toggle_export_tools moved to context_usage.rs
    // toggle_export_thinking moved to thinking.rs

    // Pinned message methods moved to messages.rs:
    // - toggle_pin, is_pinned, pinned_count
    // - get_pinned_indices, get_pinned_messages
    // - toggle_pinned_panel

    // toggle_quick_settings moved to panel_toggles.rs
    // toggle_stats_panel moved to panel_toggles.rs

    // Statistics methods moved to messages.rs:
    // - get_conversation_stats, calculate_stats
    // - count_messages_by_role
    // - get_conversation_topics, get_tools_used
    // - get_brief_summary, format_token_count

    // ==================== Input History ====================

    // Input history handlers moved to handlers.rs:
    // - add_to_input_history, input_history_previous, input_history_next
    // - input_history_count, clear_input_history

    // ==================== Message Reactions ====================
    // add_reaction moved to reactions.rs
    // Reactions and context menu methods moved to reactions.rs:
    // - get_reactions, has_reactions, show_quick_reactions, hide_quick_reactions
    // - show_context_menu, hide_context_menu, execute_context_menu_action
    // - toggle_notes_panel, set_session_notes

    // get_session_notes, has_notes moved to reactions.rs

    // ==================== Auto-Title & Summary ====================
    // Moved to summary.rs:
    // - auto_generate_title
    // - request_ai_title
    // - request_summary
    // - get_quick_summary
    // - export_shareable_summary
    // - extract_mentioned_files
    // - quick_mention_readme
    // - quick_mention_package
    // - quick_mention_cargo

    // ==================== Favorite Prompts ====================
    // toggle_favorites_panel moved to favorites.rs
    // save_input_as_favorite moved to handlers.rs (see input handlers)
    // use_favorite moved to favorites.rs
    // remove_favorite moved to favorites.rs
    // favorites_by_usage moved to favorites.rs

    // ==================== Focus Mode ====================
    // toggle_focus_mode, is_focus_mode moved to toggles.rs

    // ==================== Conversation Tags ====================

    // Tag-related methods moved to tags.rs:
    // - toggle_tags_editor
    // - add_tag
    // - remove_tag
    // - get_tags
    // - has_tags
    // - suggest_tags

    // has_messages moved to message_ops.rs
    // get_last_assistant_message moved to context_usage.rs
    // get_last_user_message moved to context_usage.rs
    // edit_last_message moved to message_ops.rs
    // copy_last_response moved to message_ops.rs
    // branch_from_message moved to message_ops.rs

    // Message editing methods moved to messages.rs:
    // - edit_message_at, retry_from_message
    // - delete_message_at, quote_message

    // copy_conversation_to_clipboard moved to message_ops.rs
    // copy_conversation_as_markdown moved to message_ops.rs

    // Session capabilities formatting moved to claude_events.rs:
    // - format_session_capabilities

    // ==================== Render: Context Menu ====================

    // render_search_bar moved to render.rs
    // render_notifications moved to render.rs
    // render_shortcuts_help moved to render.rs
    // render_command_palette moved to render.rs
    // render_messages_toolbar moved to render.rs
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = {
            let theme = self.app_state.theme.read(cx);
            theme.clone()
        };
        let theme = &theme_clone;
        let is_streaming = self.is_streaming;
        // Dynamic streaming text based on current activity
        let streaming_text = if let Some(ref tool_name) = self.current_tool_name {
            format!("Using {}{}", tool_name, self.get_streaming_dots())
        } else if self.current_thinking.is_some() {
            format!("Thinking{}", self.get_streaming_dots())
        } else {
            format!("Claude is thinking{}", self.get_streaming_dots())
        };
        let show_stats = self.show_stats && !self.messages.is_empty();
        let show_search = self.show_search;
        let show_scroll_btn = self.show_scroll_to_bottom && self.messages.len() > 3;

        div()
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            // Status header (always visible)
            .child(self.render_status_header(&theme, cx))
            // Model switcher dropdown (when visible)
            .when(self.show_model_switcher, |d| {
                d.child(self.render_model_switcher(&theme, cx))
            })
            // Context progress bar (when we have usage data)
            .when(self.context_used > 0, |d| {
                d.child(self.render_context_progress_bar(&theme, cx))
            })
            // Messages toolbar (when messages exist)
            .when(!self.messages.is_empty(), |d| {
                d.child(self.render_messages_toolbar(&theme, cx))
            })
            // Search bar (when visible)
            .when(show_search, |d| {
                d.child(self.render_search_bar(&theme, cx))
            })
            // Messages area
            .child(
                div()
                    .flex_1()
                    .id("messages-scroll-container")
                    .overflow_y_scroll()
                    .px_4()
                    .py_4()
                    .gap_3()
                    .flex()
                    .flex_col()
                    // Existing messages (Entity-based, collapsible, filtered) with highlight and time separators
                    .children({
                        let show_separators = self.show_time_separators;
                        let highlighted_message = self.highlighted_message;
                        self.messages_with_time_groups(cx).into_iter().map(move |(time_group, idx, view)| {
                            // Calculate highlight opacity inline
                            let highlight_opacity = if let Some((highlighted_idx, timestamp)) = highlighted_message {
                                if highlighted_idx == idx {
                                    let elapsed = timestamp.elapsed().as_millis() as f32;
                                    let fade_duration = 2000.0;
                                    if elapsed < fade_duration {
                                        1.0 - (elapsed / fade_duration)
                                    } else {
                                        0.0
                                    }
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            };
                            let is_highlighted = highlight_opacity > 0.0;

                            // Container for optional time separator + message
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                // Time separator (if new group and enabled)
                                .when_some(time_group.filter(|_| show_separators), |d, group_label| {
                                    d.child(
                                    div()
                                        .w_full()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .py_2()
                                        .my_2()
                                        // Left line
                                        .child(
                                            div()
                                                .flex_1()
                                                .h(px(1.0))
                                                .bg(theme.colors.border.opacity(0.3))
                                        )
                                        // Group label
                                        .child(
                                            div()
                                                .px_3()
                                                .py_1()
                                                .rounded_full()
                                                .bg(theme.colors.surface)
                                                .border_1()
                                                .border_color(theme.colors.border.opacity(0.3))
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(group_label)
                                        )
                                        // Right line
                                        .child(
                                            div()
                                                .flex_1()
                                                .h(px(1.0))
                                                .bg(theme.colors.border.opacity(0.3))
                                        )
                                )
                            })
                            // Message wrapper with highlight
                            .child(
                                div()
                                    .id(ElementId::Name(format!("message-wrapper-{}", idx).into()))
                                    .w_full()
                                    .when(is_highlighted, |d| {
                                        d.bg(theme.colors.accent.opacity(highlight_opacity * 0.15))
                                            .rounded_lg()
                                            .border_l_4()
                                            .border_color(theme.colors.accent.opacity(highlight_opacity))
                                    })
                                    .child(view)
                            )
                        })
                    })
                    // Streaming message
                    .when_some(self.streaming_message_view.clone(), |this, view| {
                        this.child(view)
                    })
                    // Thinking indicator (when Claude is reasoning)
                    .when_some(self.current_thinking.clone(), |this, thinking| {
                        if !self.show_thinking {
                            return this;
                        }
                        // Show truncated thinking content
                        let preview = if thinking.len() > 200 {
                            format!("{}...", &thinking[..200])
                        } else {
                            thinking
                        };
                        this.child(
                            div()
                                .id("thinking-indicator")
                                .px_3()
                                .py_2()
                                .mx_2()
                                .mb_1()
                                .rounded_md()
                                .bg(theme.colors.warning.opacity(0.05))
                                .border_1()
                                .border_color(theme.colors.warning.opacity(0.2))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .mb_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.colors.warning)
                                                .child("💭 Claude's Thinking")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("(⌥T to hide)")
                                        )
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted.opacity(0.8))
                                        .font_family("monospace")
                                        .child(preview)
                                )
                        )
                    })
                    // Streaming indicator with Stop button and progress info
                    .when(is_streaming, |this| {
                        // Calculate current response length from streaming content
                        let response_chars = self.streaming.current_message
                            .as_ref()
                            .map(|m| m.len())
                            .unwrap_or(0);
                        let response_words = self.streaming.current_message
                            .as_ref()
                            .map(|m| m.split_whitespace().count())
                            .unwrap_or(0);
                        // Estimate tokens (~1.3 per word)
                        let est_tokens = ((response_words as f64) * 1.3).ceil() as usize;
                        // Calculate elapsed time
                        let elapsed_str = self.response_start_time
                            .map(|start| {
                                let elapsed = chrono::Utc::now().signed_duration_since(start);
                                if elapsed.num_seconds() < 60 {
                                    format!("{}s", elapsed.num_seconds())
                                } else {
                                    format!("{}m {}s", elapsed.num_minutes(), elapsed.num_seconds() % 60)
                                }
                            })
                            .unwrap_or_else(|| "0s".to_string());

                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .px_3()
                                .py_2()
                                .rounded_lg()
                                .bg(theme.colors.surface)
                                .border_1()
                                .border_color(theme.colors.accent.opacity(0.3))
                                .mx_2()
                                .mb_2()
                                // Left: Status and progress
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        // Animated pulse indicator or tool icon
                                        .when_some(self.current_tool_name.as_ref(), |d, tool_name| {
                                            d.child(
                                                div()
                                                    .text_base()
                                                    .child(Self::get_tool_icon(tool_name))
                                            )
                                        })
                                        .when(self.current_tool_name.is_none(), |d| {
                                            d.child(
                                                div()
                                                    .size(px(10.0))
                                                    .rounded_full()
                                                    .bg(theme.colors.accent)
                                            )
                                        })
                                        // Status text
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_family("monospace")
                                                .text_color(theme.colors.text_muted)
                                                .child(streaming_text)
                                        )
                                        // Elapsed time
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_2()
                                                .py_px()
                                                .rounded_sm()
                                                .bg(theme.colors.warning.opacity(0.1))
                                                .text_xs()
                                                .text_color(theme.colors.warning)
                                                .child("⏱")
                                                .child(elapsed_str)
                                        )
                                        // Progress stats
                                        .when(response_words > 0, |d| {
                                            d.child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .px_2()
                                                    .py_px()
                                                    .rounded_sm()
                                                    .bg(theme.colors.info.opacity(0.1))
                                                    .text_xs()
                                                    .text_color(theme.colors.info)
                                                    .child(format!("{} words", response_words))
                                                    .child("·")
                                                    .child(format!("~{} tokens", est_tokens))
                                            )
                                        })
                                        // Streaming speed indicator with peak
                                        .when(self.last_streaming_speed > 0.0, |d| {
                                            let speed = self.last_streaming_speed;
                                            let peak = self.peak_streaming_speed;
                                            let speed_color = if speed > 50.0 {
                                                theme.colors.success
                                            } else if speed > 20.0 {
                                                theme.colors.info
                                            } else {
                                                theme.colors.warning
                                            };
                                            d.child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .px_2()
                                                    .py_px()
                                                    .rounded_sm()
                                                    .bg(speed_color.opacity(0.1))
                                                    .text_xs()
                                                    .text_color(speed_color)
                                                    .child("⚡")
                                                    .child(format!("{:.0}", speed))
                                                    .when(peak > speed * 1.2, |d| {
                                                        d.child(
                                                            div()
                                                                .text_color(theme.colors.text_muted)
                                                                .child(format!("(↑{:.0})", peak))
                                                        )
                                                    })
                                                    .child(
                                                        div()
                                                            .text_color(speed_color.opacity(0.7))
                                                            .child("tok/s")
                                                    )
                                            )
                                        })
                                )
                                // Right: Stop button with keyboard hint
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .id("stop-button")
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_3()
                                                .py_1()
                                                .rounded_md()
                                                .bg(theme.colors.error.opacity(0.1))
                                                .border_1()
                                                .border_color(theme.colors.error.opacity(0.3))
                                                .text_xs()
                                                .text_color(theme.colors.error)
                                                .cursor_pointer()
                                                .hover(|style| {
                                                    style
                                                        .bg(theme.colors.error.opacity(0.2))
                                                        .border_color(theme.colors.error.opacity(0.5))
                                                })
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.request_stop(cx);
                                                }))
                                                .child("Stop")
                                                .child(
                                                    div()
                                                        .text_color(theme.colors.error.opacity(0.6))
                                                        .font_family("monospace")
                                                        .child("⎋")
                                                )
                                        )
                                )
                        )
                    })
                    // Empty state (show only when no messages and not streaming)
                    .when(self.messages.is_empty() && self.streaming.current_message.is_none(), |this| {
                this.child(
                            div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_4()
                                        .child(
                                            div()
                                                .text_xl()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.colors.text_muted)
                                                .child("Welcome to Claude Visual"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child("Start a conversation by typing a message below"),
                                        )
                                        // Session info if available
                                        .when_some(self.session_info.as_ref(), |d, info| {
                                            d.child(
                                                div()
                                                    .mt_2()
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .bg(theme.colors.success.opacity(0.1))
                                                    .text_xs()
                                                    .text_color(theme.colors.success)
                                                    .child(format!("Connected to {} · {} tools available", info.model, info.tools.len()))
                                            )
                                        })
                                        // Welcome tips for new users (dismissable)
                                        .when(self.show_onboarding_tips, |d| {
                                            d.child(
                                                div()
                                                    .mt_4()
                                                    .child(self.render_welcome_tips(&theme, cx))
                                            )
                                        })
                                        // Recent Sessions section (for quick resume)
                                        .when(!self.recent_sessions.is_empty(), |d| {
                                            d.child(
                                                div()
                                                    .mt_6()
                                                    .w_full()
                                                    .max_w(px(600.0))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_between()
                                                            .mb_3()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(theme.colors.text_muted)
                                                                    .child("Recent Sessions")
                                                            )
                                                            .child(
                                                                div()
                                                                    .id("view-all-sessions-btn")
                                                                    .px_2()
                                                                    .py_1()
                                                                    .rounded_md()
                                                                    .cursor_pointer()
                                                                    .text_xs()
                                                                    .text_color(theme.colors.accent)
                                                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                                                    .on_click(cx.listener(|this, _, _window, cx| {
                                                                        this.show_session_history = true;
                                                                        cx.notify();
                                                                    }))
                                                                    .child("View All")
                                                            )
                                                    )
                                                    .child(self.render_recent_sessions_cards(&theme, cx))
                                            )
                                        })
                                        // Starter prompts - quick conversation starters
                                        .child(
                                            div()
                                                .mt_6()
                                                .w_full()
                                                .max_w(px(600.0))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .mb_3()
                                                        .child("Try asking...")
                                                )
                                                .child(self.render_starter_prompts(&theme, cx))
                                        )
                                        // Quick actions
                                        .child(
                                            div()
                                                .mt_6()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .mb_3()
                                                        .child("Commands")
                                                )
                                                .child(self.render_quick_actions(&theme, cx))
                                        )
                                        // Tips section
                                        .child(
                                            div()
                                                .mt_6()
                                                .flex()
                                                .flex_col()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .mb_1()
                                                        .child("Tips")
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_wrap()
                                                        .gap_3()
                                                        .justify_center()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("/")
                                                                )
                                                                .child("Slash commands")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘?")
                                                                )
                                                                .child("Keyboard shortcuts")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⇧↩")
                                                                )
                                                                .child("New line")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘K")
                                                                )
                                                                .child("Command palette")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("@")
                                                                )
                                                                .child("Mention files")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘B")
                                                                )
                                                                .child("Toggle sidebar")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘E")
                                                                )
                                                                .child("Export chat")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘M")
                                                                )
                                                                .child("Switch model")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .text_xs()
                                                                        .child("📁→")
                                                                )
                                                                .child("Drag & drop files")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌥S")
                                                                )
                                                                .child("Session details")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌥T")
                                                                )
                                                                .child("Show thinking")
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap_1()
                                                                .child(
                                                                    div()
                                                                        .px_1()
                                                                        .rounded_sm()
                                                                        .bg(theme.colors.surface)
                                                                        .border_1()
                                                                        .border_color(theme.colors.border)
                                                                        .font_family("monospace")
                                                                        .child("⌘T")
                                                                )
                                                                .child("Prompt templates")
                                                        )
                                                )
                                        ),
                                ),
                        )
                    }),
            )
            // Scroll to bottom button (floating)
            .when(show_scroll_btn, |d| {
                d.child(self.render_scroll_to_bottom(&theme, cx))
            })
            // Navigation history bar (when there's history to navigate)
            .when(self.can_navigate_back() || self.can_navigate_forward(), |d| {
                d.child(self.render_navigation_bar(&theme, cx))
            })
            // Floating action button (when not streaming and has messages)
            .when(!is_streaming && !self.messages.is_empty(), |d| {
                d.child(self.render_fab(&theme, cx))
            })
            // Floating navigation indicator (when message is selected)
            .when_some(self.selected_message_index, |d, idx| {
                let total = self.messages.len();
                d.child(
                    div()
                        .absolute()
                        .bottom(px(140.0))
                        .left_1_2()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_2()
                        .rounded_lg()
                        .bg(theme.colors.surface.opacity(0.95))
                        .border_1()
                        .border_color(theme.colors.accent.opacity(0.3))
                        .shadow_md()
                        // Position indicator
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.accent)
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(format!("{}/{}", idx + 1, total))
                                )
                        )
                        // Navigation hints
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .pl_2()
                                .border_l_1()
                                .border_color(theme.colors.border)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .px_1()
                                                .py_px()
                                                .rounded_sm()
                                                .bg(theme.colors.background)
                                                .border_1()
                                                .border_color(theme.colors.border)
                                                .text_xs()
                                                .font_family("monospace")
                                                .text_color(theme.colors.text_muted)
                                                .child("⌥↑↓")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("navigate")
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .px_1()
                                                .py_px()
                                                .rounded_sm()
                                                .bg(theme.colors.background)
                                                .border_1()
                                                .border_color(theme.colors.border)
                                                .text_xs()
                                                .font_family("monospace")
                                                .text_color(theme.colors.text_muted)
                                                .child("⎋")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("clear")
                                        )
                                )
                        )
                )
            })
            // Stats bar (above input)
            .when(show_stats, |d| {
                d.child(self.render_stats_bar(&theme))
            })
            // Quick actions bar (between stats and input when not streaming)
            .when(!is_streaming && !self.messages.is_empty(), |d| {
                d.child(self.render_quick_actions_bar(&theme, cx))
            })
            // Error retry bar (when there's an error that can be retried)
            .when(self.last_error.is_some(), |d| {
                d.child(self.render_error_retry_bar(&theme, cx))
            })
            // Suggestions bar (above input when enabled and has suggestions)
            .when(self.show_suggestions && !self.contextual_suggestions.is_empty(), |d| {
                d.child(self.render_suggestions_bar(&theme, cx))
            })
            // Quick templates bar (when messages exist and not streaming)
            .when(!is_streaming && self.messages.is_empty(), |d| {
                d.child(self.render_quick_templates_bar(&theme, cx))
            })
            // Input toolbar (above input area)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_1()
                    .border_t_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .bg(theme.colors.surface.opacity(0.3))
                    // Left side - input mode and file picker
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Multiline toggle
                            .child(
                                div()
                                    .id("multiline-toggle")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(if self.multiline_input { theme.colors.accent } else { theme.colors.text_muted })
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_multiline_input(cx);
                                    }))
                                    .when(self.multiline_input, |d| d.child("▼").child("Multi-line"))
                                    .when(!self.multiline_input, |d| d.child("▶").child("Single-line"))
                            )
                            // Height controls (when multiline)
                            .when(self.multiline_input, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .id("decrease-height")
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded_sm()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.decrease_input_height(cx);
                                                }))
                                                .child("−")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(format!("{} lines", self.input_height_lines))
                                        )
                                        .child(
                                            div()
                                                .id("increase-height")
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .rounded_sm()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.increase_input_height(cx);
                                                }))
                                                .child("+")
                                        )
                                )
                            })
                            // File mention button
                            .child(
                                div()
                                    .id("file-mention-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_file_picker(cx);
                                    }))
                                    .child("@")
                                    .child("File")
                            )
                            // Think mode toggle button
                            .child(
                                div()
                                    .id("think-mode-toggle")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(if self.think_mode_enabled { theme.colors.warning } else { theme.colors.text_muted })
                                    .when(self.think_mode_enabled, |d| d.bg(theme.colors.warning.opacity(0.1)))
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_think_mode(cx);
                                    }))
                                    .child("🧠")
                                    .child(if self.think_mode_enabled { "Deep Think" } else { "Think" })
                            )
                            // MCP servers button (if connected)
                            .when_some(self.session_info.as_ref().filter(|i| !i.mcp_servers.is_empty()), |d, info| {
                                let server_count = info.mcp_servers.len();
                                d.child(
                                    div()
                                        .id("mcp-servers-btn")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.colors.info)
                                        .bg(theme.colors.info.opacity(0.1))
                                        .hover(|s| s.bg(theme.colors.info.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_mcp_panel(cx);
                                        }))
                                        .child("🔌")
                                        .child(format!("{} MCP", server_count))
                                )
                            })
                            // Memory quick access button
                            .child(
                                div()
                                    .id("memory-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.send_slash_command("/memory", cx);
                                    }))
                                    .child("🧠")
                                    .child("Memory")
                            )
                            // Add context button
                            .child(
                                div()
                                    .id("add-context-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.send_slash_command("/add-dir", cx);
                                    }))
                                    .child("📁")
                                    .child("Context")
                            )
                            // Review button
                            .child(
                                div()
                                    .id("review-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.send_slash_command("/review", cx);
                                    }))
                                    .child("👀")
                                    .child("Review")
                            )
                            // Commit button (when git changes exist)
                            .when(self.git_info.as_ref().map(|g| g.is_dirty).unwrap_or(false), |d| {
                                d.child(
                                    div()
                                        .id("commit-btn")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.colors.success)
                                        .bg(theme.colors.success.opacity(0.1))
                                        .hover(|s| s.bg(theme.colors.success.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.send_slash_command("/commit", cx);
                                        }))
                                        .child("📦")
                                        .child("Commit")
                                )
                            })
                            // Commands palette button
                            .child(
                                div()
                                    .id("commands-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.accent)
                                    .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_commands_panel(cx);
                                    }))
                                    .child("⚡")
                                    .child("More")
                            )
                    )
                    // Center - Recommended action (when available)
                    .when_some(self.get_recommended_workflow_action(), |d, (icon, name, cmd, desc)| {
                        let command = cmd.to_string();
                        let is_urgent = cmd == "/compact" && self.context_usage_percentage() > 0.85;
                        d.child(
                            div()
                                .id("recommended-action")
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py(px(3.0))
                                .rounded_lg()
                                .cursor_pointer()
                                .border_1()
                                .border_color(if is_urgent { theme.colors.error.opacity(0.5) } else { theme.colors.warning.opacity(0.3) })
                                .bg(if is_urgent { theme.colors.error.opacity(0.1) } else { theme.colors.warning.opacity(0.1) })
                                .hover(|s| s.bg(if is_urgent { theme.colors.error.opacity(0.2) } else { theme.colors.warning.opacity(0.2) }))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    if command.starts_with('/') {
                                        this.send_slash_command(&command, cx);
                                    } else if command == "retry" {
                                        this.retry_last_request(cx);
                                    } else if command == "continue" {
                                        this.continue_conversation(cx);
                                    }
                                }))
                                .child(
                                    div()
                                        .text_sm()
                                        .child(icon)
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(if is_urgent { theme.colors.error } else { theme.colors.warning })
                                                .child(format!("Suggested: {}", name))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(desc)
                                        )
                                )
                        )
                    })
                    // Right side - model and session info
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Quick model switcher button
                            .when_some(self.session_info.as_ref(), |d, info| {
                                let model_short = if info.model.len() > 15 {
                                    format!("{}...", &info.model[..15])
                                } else {
                                    info.model.clone()
                                };
                                d.child(
                                    div()
                                        .id("quick-model-btn")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(theme.colors.accent.opacity(0.1))
                                        .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_model_switcher(cx);
                                        }))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.accent)
                                                .font_weight(FontWeight::MEDIUM)
                                                .child(model_short)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.accent.opacity(0.7))
                                                .child("▾")
                                        )
                                )
                            })
                            // Session ID (if available)
                            .when_some(self.session_info.as_ref(), |d, info| {
                                let session_short = if info.session_id.len() > 8 {
                                    format!("{}...", &info.session_id[..8])
                                } else {
                                    info.session_id.clone()
                                };
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted.opacity(0.5))
                                                .child("Session:")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_family("monospace")
                                                .text_color(theme.colors.text_muted)
                                                .child(session_short)
                                        )
                                )
                            })
                            // Keyboard hint
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_1()
                                            .py_px()
                                            .rounded_sm()
                                            .bg(theme.colors.background)
                                            .border_1()
                                            .border_color(theme.colors.border.opacity(0.5))
                                            .text_xs()
                                            .font_family("monospace")
                                            .text_color(theme.colors.text_muted.opacity(0.6))
                                            .child("⏎")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted.opacity(0.5))
                                            .child("send")
                                    )
                            )
                    )
            )
            // Context usage progress bar (compact, above input)
            .when(self.context_used > 0 || self.session_input_tokens > 0, |d| {
                let usage_pct = self.context_usage_percentage();
                let usage_color = if usage_pct < 0.5 {
                    theme.colors.success
                } else if usage_pct < 0.8 {
                    theme.colors.warning
                } else {
                    theme.colors.error
                };
                let tokens_used = self.context_used.max(self.session_input_tokens + self.session_output_tokens);
                let tokens_display = Self::format_token_count(tokens_used);
                d.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_px()
                        .px_3()
                        .py_1()
                        .bg(theme.colors.surface.opacity(0.5))
                        .border_t_1()
                        .border_color(theme.colors.border.opacity(0.3))
                        // Label row
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted.opacity(0.7))
                                                .child("Context")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(usage_color)
                                                .child(format!("{}%", (usage_pct * 100.0) as u32))
                                        )
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted.opacity(0.6))
                                        .child(format!("{} tokens", tokens_display))
                                )
                        )
                        // Progress bar
                        .child(
                            div()
                                .h(px(3.0))
                                .w_full()
                                .rounded_full()
                                .bg(theme.colors.border.opacity(0.3))
                                .child(
                                    div()
                                        .h_full()
                                        .rounded_full()
                                        .bg(usage_color)
                                        .w(pct((usage_pct * 100.0) as f32))
                                )
                        )
                )
            })
            // Quick actions bar (continue, regenerate when applicable)
            .when(self.is_last_response_truncated() && !self.is_streaming, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .px_4()
                        .py_2()
                        .bg(theme.colors.surface.opacity(0.8))
                        .border_t_1()
                        .border_color(theme.colors.border)
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child("Response may be incomplete")
                        )
                        .child(
                            div()
                                .id("continue-button")
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .bg(theme.colors.accent.opacity(0.1))
                                .text_color(theme.colors.accent)
                                .border_1()
                                .border_color(theme.colors.accent.opacity(0.3))
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.continue_conversation(cx);
                                }))
                                .child("Continue ▶")
                        )
                        .child(
                            div()
                                .id("regenerate-button")
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.regenerate_last_response(cx);
                                }))
                                .child("Regenerate 🔄")
                        )
                )
            })
            // Pro tip (shown occasionally, every 5 messages when not streaming and show tips enabled)
            .when(self.show_onboarding_tips && !is_streaming && self.messages.len() > 0 && self.messages.len() % 5 == 0, |d| {
                if let Some((icon, title, desc)) = self.get_contextual_pro_tip() {
                    d.child(
                        div()
                            .mx_4()
                            .mb_2()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .bg(theme.colors.accent.opacity(0.05))
                            .border_1()
                            .border_color(theme.colors.accent.opacity(0.2))
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .child(icon)
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_px()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.accent)
                                            .child(format!("Pro tip: {}", title))
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(desc)
                                    )
                            )
                    )
                } else {
                    d
                }
            })
            // Streaming suggestions (contextual skill suggestions during streaming)
            .when(is_streaming, |d| {
                d.child(self.render_streaming_suggestions(&theme, cx))
            })
            // Post-response suggestions (after response completes)
            .when(!is_streaming && !self.messages.is_empty(), |d| {
                d.child(self.render_post_response_suggestions(&theme, cx))
            })
            // Quick reply suggestions (when available and not streaming)
            .when(!self.quick_reply_suggestions.is_empty() && !is_streaming, |d| {
                d.child(self.render_quick_reply_suggestions(&theme, cx))
            })
            // Git quick actions bar (when there are uncommitted changes)
            .child(self.render_git_quick_actions(&theme, cx))
            // Contextual keyboard hints bar
            .child(self.render_contextual_hints(&theme))
            // Context files preview bar (when files are attached)
            .when(!self.context_files.is_empty(), |d| {
                d.child(self.render_context_files_bar(&theme, cx))
            })
            // Input area
            .child(
                div()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .child(self.input.clone()),
            )
            // Notifications (toast-style, floating top-right with quick actions)
            .when(!self.notifications.is_empty(), |d| {
                d.child(self.render_notifications(&theme, cx))
            })
            // Context menu (floating, higher z-index)
            .when(self.context_menu.is_some(), |d| {
                d.child(self.render_context_menu(&theme, cx))
            })
            // Keyboard shortcuts help panel (modal overlay)
            .when(self.show_shortcuts_help, |d| {
                d.child(self.render_shortcuts_help(&theme, cx))
            })
            // Command palette (modal overlay)
            .when(self.show_command_palette, |d| {
                d.child(self.render_command_palette(&theme, cx))
            })
            // Session history panel (modal overlay)
            .when(self.show_session_history, |d| {
                d.child(self.render_session_history_panel(&theme, cx))
            })
            // Permissions panel (modal overlay)
            .when(self.show_permissions_panel, |d| {
                d.child(self.render_permissions_panel(&theme, cx))
            })
            // MCP servers panel (modal overlay)
            .when(self.panels.mcp_panel, |d| {
                d.child(self.render_mcp_panel(&theme, cx))
            })
            // MCP quick tools dropdown
            .when(self.panels.mcp_quick_tools, |d| {
                d.child(self.render_mcp_quick_tools(&theme, cx))
            })
            // Commands panel (modal overlay)
            .when(self.show_commands_panel, |d| {
                d.child(self.render_commands_panel(&theme, cx))
            })
            // Templates panel (modal overlay)
            .when(self.show_templates_panel, |d| {
                d.child(self.render_templates_panel(&theme, cx))
            })
            // Context panel (modal overlay)
            .when(self.show_context_panel, |d| {
                d.child(self.render_context_panel(&theme, cx))
            })
            // Export panel (modal overlay)
            .when(self.show_export_panel, |d| {
                d.child(self.render_export_panel(&theme, cx))
            })
            // Notes panel (modal overlay)
            .when(self.show_notes_panel, |d| {
                d.child(self.render_notes_panel(&theme, cx))
            })
            // Favorites panel (modal overlay)
            .when(self.show_favorites_panel, |d| {
                d.child(self.render_favorites_panel(&theme, cx))
            })
            // Tags editor panel (modal overlay)
            .when(self.show_tags_editor, |d| {
                d.child(self.render_tags_panel(&theme, cx))
            })
            // Pinned messages panel (modal overlay)
            .when(self.show_pinned_panel, |d| {
                d.child(self.render_pinned_panel(&theme, cx))
            })
            // Recent files panel (modal overlay)
            .when(self.show_recent_files_panel, |d| {
                d.child(self.render_recent_files_panel(&theme, cx))
            })
            // Statistics panel (modal overlay)
            .when(self.show_stats_panel, |d| {
                d.child(self.render_stats_panel(&theme, cx))
            })
            // Quick settings panel (modal overlay)
            .when(self.show_quick_settings, |d| {
                d.child(self.render_quick_settings_panel(&theme, cx))
            })
            // Active tasks panel (modal overlay)
            .when(self.show_tasks_panel, |d| {
                d.child(self.render_tasks_panel(&theme, cx))
            })
            // Git status panel (modal overlay)
            .when(self.show_git_panel, |d| {
                d.child(self.render_git_panel(&theme, cx))
            })
            // File picker (modal overlay)
            .when(self.file_picker_visible, |d| {
                d.child(self.render_file_picker(&theme, cx))
            })
            // Session details panel (modal overlay)
            .when(self.show_session_details, |d| {
                d.child(self.render_session_details(&theme, cx))
            })
            // File drop zone overlay (highest z-index, shown when dragging files)
            .when(self.file_drag_active, |d| {
                d.child(self.render_file_drop_zone(&theme))
            })
    }
}

// HTML helper functions moved to session.rs:
// - html_escape
// - convert_code_blocks_to_html

#[cfg(test)]
mod tests {
    use super::*;

    fn test_conversation_search_result() {
        let result = ConversationSearchResult {
            message_index: 0,
            line_number: 5,
            start: 10,
            end: 15,
            snippet: "...test query here...".to_string(),
            role: MessageRole::User,
        };

        assert_eq!(result.message_index, 0);
        assert_eq!(result.line_number, 5);
        assert_eq!(result.start, 10);
        assert_eq!(result.end, 15);
        assert!(result.snippet.contains("query"));
    }

    #[test]
    fn test_message_filter_labels() {
        assert_eq!(MessageFilter::All.label(), "All");
        assert_eq!(MessageFilter::UserOnly.label(), "You");
        assert_eq!(MessageFilter::AssistantOnly.label(), "Claude");
        assert_eq!(MessageFilter::ToolsOnly.label(), "Tools");
    }

    #[test]
    fn test_message_filter_includes_role() {
        // All filter includes everything
        assert!(MessageFilter::All.includes_role(MessageRole::User));
        assert!(MessageFilter::All.includes_role(MessageRole::Assistant));
        assert!(MessageFilter::All.includes_role(MessageRole::ToolUse));
        assert!(MessageFilter::All.includes_role(MessageRole::ToolResult));

        // UserOnly filter
        assert!(MessageFilter::UserOnly.includes_role(MessageRole::User));
        assert!(!MessageFilter::UserOnly.includes_role(MessageRole::Assistant));
        assert!(!MessageFilter::UserOnly.includes_role(MessageRole::ToolUse));

        // AssistantOnly filter
        assert!(!MessageFilter::AssistantOnly.includes_role(MessageRole::User));
        assert!(MessageFilter::AssistantOnly.includes_role(MessageRole::Assistant));
        assert!(!MessageFilter::AssistantOnly.includes_role(MessageRole::ToolUse));

        // ToolsOnly filter
        assert!(!MessageFilter::ToolsOnly.includes_role(MessageRole::User));
        assert!(!MessageFilter::ToolsOnly.includes_role(MessageRole::Assistant));
        assert!(MessageFilter::ToolsOnly.includes_role(MessageRole::ToolUse));
        assert!(MessageFilter::ToolsOnly.includes_role(MessageRole::ToolResult));
    }

    #[test]
    fn test_message_filter_all_options() {
        let options = MessageFilter::all_options();
        assert_eq!(options.len(), 4);
        assert_eq!(options[0], MessageFilter::All);
        assert_eq!(options[1], MessageFilter::UserOnly);
        assert_eq!(options[2], MessageFilter::AssistantOnly);
        assert_eq!(options[3], MessageFilter::ToolsOnly);
    }

    #[test]
    fn test_message_filter_default() {
        let filter = MessageFilter::default();
        assert_eq!(filter, MessageFilter::All);
    }
}
