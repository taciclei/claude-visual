//! View state types

use super::{ConversationSearchResult, MessageFilter, ExportFormat, FilePickerItem};
use crate::app::theme::Theme;

/// Connection status to Claude CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Search state for the chat view
#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub show: bool,
    pub results: Vec<ConversationSearchResult>,
    pub current_result: usize,
    pub case_sensitive: bool,
    pub regex: bool,
    pub role_filter: MessageFilter,
}

/// Streaming state for real-time responses
#[derive(Default)]
pub struct StreamingState {
    pub is_streaming: bool,
    pub current_message: Option<String>,
    pub streaming_dots: usize,
    pub token_count: usize,
    pub last_speed: f64,
    pub peak_speed: f64,
    pub response_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_response_time_ms: Option<u64>,
}

/// Panel visibility state
#[derive(Default)]
pub struct PanelVisibility {
    pub shortcuts_help: bool,
    pub command_palette: bool,
    pub model_switcher: bool,
    pub suggestions: bool,
    pub fab_menu: bool,
    pub session_history: bool,
    pub permissions_panel: bool,
    pub mcp_panel: bool,
    pub mcp_quick_tools: bool,
    pub tasks_panel: bool,
    pub git_panel: bool,
    pub session_details: bool,
    pub commands_panel: bool,
    pub templates_panel: bool,
    pub context_panel: bool,
    pub export_panel: bool,
    pub notes_panel: bool,
    pub favorites_panel: bool,
    pub tags_editor: bool,
    pub pinned_panel: bool,
    pub quick_settings: bool,
    pub stats_panel: bool,
    pub recent_files_panel: bool,
    pub onboarding_tips: bool,
}

/// Command palette state
#[derive(Default)]
pub struct CommandPaletteState {
    pub query: String,
    pub selected_index: usize,
    pub recent_commands: Vec<String>,
}

/// Export settings
#[derive(Default)]
pub struct ExportSettings {
    pub format: ExportFormat,
    pub include_metadata: bool,
    pub include_tools: bool,
    pub include_thinking: bool,
}

/// Session statistics
#[derive(Default)]
pub struct SessionStats {
    pub cost: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_api_requests: u32,
    pub health: f32,
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    pub last_response_latency_ms: Option<u64>,
    pub avg_response_latency_ms: f64,
    pub connection_retry_count: u32,
}

impl SessionStats {
    /// Get color for health indicator
    pub fn health_color(&self, theme: &Theme) -> gpui::Hsla {
        if self.health >= 0.8 {
            theme.colors.success
        } else if self.health >= 0.5 {
            theme.colors.warning
        } else {
            theme.colors.error
        }
    }

    /// Get label for health status
    pub fn health_label(&self) -> &'static str {
        if self.health >= 0.8 {
            "Healthy"
        } else if self.health >= 0.5 {
            "Fair"
        } else {
            "Poor"
        }
    }
}

/// File picker state
#[derive(Default)]
pub struct FilePickerState {
    pub visible: bool,
    pub query: String,
    pub results: Vec<FilePickerItem>,
}

/// Input history state
#[derive(Default)]
pub struct InputHistoryState {
    pub history: Vec<String>,
    pub position: i32,
    pub temp: String,
    pub max_size: usize,
}
