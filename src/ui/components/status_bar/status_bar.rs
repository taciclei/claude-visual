//! Main status bar component implementation

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::StatusBarEvent;
use super::{left_section, right_section};
use crate::app::state::AppState;

/// Status bar component
pub struct StatusBar {
    pub(crate) app_state: Arc<AppState>,
    /// Current project name
    pub(crate) project_name: Option<String>,
    /// Current project path
    pub(crate) project_path: Option<String>,
    /// Git branch name (None = no git)
    pub(crate) git_branch: Option<String>,
    /// AI model name
    pub(crate) model_name: String,
    /// Session cost in dollars
    pub(crate) session_cost: f64,
    /// Session token count (in thousands)
    pub(crate) session_tokens: u64,
    /// Usage percentage (0-100)
    pub(crate) usage_percent: u8,
    /// Session duration in seconds
    pub(crate) session_duration: u64,
    /// Message count in current conversation
    pub(crate) message_count: usize,
    /// Whether Claude is currently streaming
    pub(crate) is_streaming: bool,
    /// Current vim mode state
    pub(crate) vim_mode: bool,
    /// Current filter name
    pub(crate) filter_name: String,
    /// Word wrap enabled
    pub(crate) word_wrap: bool,
    /// Line numbers enabled
    pub(crate) line_numbers: bool,
    /// Session health score (0.0 to 1.0)
    pub(crate) session_health: f32,
    /// Response latency in ms
    pub(crate) response_latency_ms: Option<u64>,
    /// Number of connected MCP servers
    pub(crate) mcp_server_count: usize,
    /// Number of available MCP tools
    pub(crate) mcp_tool_count: usize,
    /// Context files count
    pub(crate) context_files_count: usize,
    /// Memory items count
    pub(crate) memory_items_count: usize,
}

impl EventEmitter<StatusBarEvent> for StatusBar {}

impl StatusBar {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            project_name: None,
            project_path: None,
            git_branch: None,
            model_name: "Opus 4.5".to_string(),
            session_cost: 0.0,
            session_tokens: 0,
            usage_percent: 0,
            session_duration: 0,
            message_count: 0,
            is_streaming: false,
            vim_mode: false,
            filter_name: "All".to_string(),
            word_wrap: false,
            line_numbers: true,
            session_health: 1.0,
            response_latency_ms: None,
            mcp_server_count: 0,
            mcp_tool_count: 0,
            context_files_count: 0,
            memory_items_count: 0,
        }
    }

    /// Update git branch
    pub fn set_git_branch(&mut self, branch: Option<String>, cx: &mut Context<Self>) {
        self.git_branch = branch;
        cx.notify();
    }

    /// Update model name
    pub fn set_model(&mut self, model: String, cx: &mut Context<Self>) {
        self.model_name = model;
        cx.notify();
    }

    /// Update session stats
    pub fn set_session_stats(&mut self, cost: f64, tokens: u64, cx: &mut Context<Self>) {
        self.session_cost = cost;
        self.session_tokens = tokens;
        cx.notify();
    }

    /// Update usage percentage and duration
    pub fn set_usage(&mut self, percent: u8, duration_secs: u64, cx: &mut Context<Self>) {
        self.usage_percent = percent.min(100);
        self.session_duration = duration_secs;
        cx.notify();
    }

    /// Update project info
    pub fn set_project(
        &mut self,
        name: Option<String>,
        path: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.project_name = name;
        self.project_path = path;
        cx.notify();
    }

    /// Update message count
    pub fn set_message_count(&mut self, count: usize, cx: &mut Context<Self>) {
        self.message_count = count;
        cx.notify();
    }

    /// Update streaming state
    pub fn set_streaming(&mut self, streaming: bool, cx: &mut Context<Self>) {
        self.is_streaming = streaming;
        cx.notify();
    }

    /// Update vim mode state
    pub fn set_vim_mode(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.vim_mode = enabled;
        cx.notify();
    }

    /// Update filter name
    pub fn set_filter(&mut self, filter: String, cx: &mut Context<Self>) {
        self.filter_name = filter;
        cx.notify();
    }

    /// Update word wrap state
    pub fn set_word_wrap(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.word_wrap = enabled;
        cx.notify();
    }

    /// Update line numbers state
    pub fn set_line_numbers(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.line_numbers = enabled;
        cx.notify();
    }

    /// Update session health score
    pub fn set_session_health(&mut self, health: f32, cx: &mut Context<Self>) {
        self.session_health = health.clamp(0.0, 1.0);
        cx.notify();
    }

    /// Update response latency
    pub fn set_response_latency(&mut self, latency_ms: Option<u64>, cx: &mut Context<Self>) {
        self.response_latency_ms = latency_ms;
        cx.notify();
    }

    /// Update MCP server info
    pub fn set_mcp_info(&mut self, server_count: usize, tool_count: usize, cx: &mut Context<Self>) {
        self.mcp_server_count = server_count;
        self.mcp_tool_count = tool_count;
        cx.notify();
    }

    /// Update context files count
    pub fn set_context_files(&mut self, count: usize, cx: &mut Context<Self>) {
        self.context_files_count = count;
        cx.notify();
    }

    /// Update memory items count
    pub fn set_memory_items(&mut self, count: usize, cx: &mut Context<Self>) {
        self.memory_items_count = count;
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .id("status-bar")
            .w_full()
            .h(px(28.0))
            .flex_shrink_0()
            .bg(theme.colors.surface)
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .text_xs()
            // Left side - Project info
            .child(left_section::render_left_section(
                &self.git_branch,
                &self.project_path,
                &self.model_name,
                self.is_streaming,
                self.mcp_server_count,
                self.mcp_tool_count,
                self.context_files_count,
                self.memory_items_count,
                &theme,
                cx,
            ))
            // Right side - Usage stats
            .child(right_section::render_right_section(
                self.session_cost,
                self.session_tokens,
                self.usage_percent,
                self.session_duration,
                self.session_health,
                self.response_latency_ms,
                self.vim_mode,
                &theme,
                cx,
            ))
    }
}
