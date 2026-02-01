use crate::ui::chat::view::{ChatView, ChatViewEvent, CommandCategory, NotificationType};
use gpui::Context;

impl ChatView {
    // ==================== Commands Panel ====================

    /// Get command description for autocomplete
    pub fn get_command_description(command: &str) -> Option<&'static str> {
        match command {
            // Core commands
            "/help" => Some("Show available commands and usage"),
            "/compact" => Some("Compact conversation to free context tokens"),
            "/usage" | "/cost" => Some("Show token usage and estimated costs"),
            "/clear" => Some("Clear the conversation context"),
            "/model" => Some("Switch to a different AI model"),
            "/think" => Some("Enable extended thinking mode for deep analysis"),
            "/memory" => Some("View and edit persistent memory"),
            "/doctor" => Some("Run diagnostics check"),
            "/init" => Some("Initialize project configuration"),
            "/add-dir" => Some("Add directory to context"),
            "/permissions" => Some("Manage tool permissions"),
            "/config" => Some("View configuration settings"),
            "/resume" => Some("Resume previous session"),
            "/bug" => Some("Report a bug"),
            "/mcp" => Some("Manage MCP servers"),
            "/vim" => Some("Toggle vim keybindings"),
            "/login" => Some("Authenticate with Anthropic"),
            "/logout" => Some("Log out from Anthropic"),
            "/status" => Some("Show git repository status"),

            // Claude Code Skills
            "/apex" => Some("APEX workflow: Analyze-Plan-Execute-eXamine"),
            "/brainstorm" => Some("Deep iterative research with skeptical analysis"),
            "/explore" => Some("Deep codebase exploration and understanding"),
            "/debug" => Some("Systematic error debugging and resolution"),
            "/review" => Some("Deep code review with parallel analysis"),
            "/oneshot" => Some("Ultra-fast feature implementation"),
            "/explain" => Some("Deep code explanation with visual diagrams"),
            "/refactor" => Some("Refactor code with parallel agents"),
            "/docs" => Some("Deep documentation research"),
            "/ultrathink" => Some("Deep thinking mode for elegant solutions"),
            "/search" => Some("Lightning-fast search for specific questions"),

            // Git commands
            "/commit" => Some("Create commit with smart message"),
            "/pr" | "/create-pr" => Some("Create and push pull request"),
            "/fix-pr-comments" => Some("Fetch and implement PR review comments"),
            "/merge" => Some("Intelligent merge with conflict resolution"),

            // Other utilities
            "/test" => Some("Generate comprehensive tests"),
            "/summarize" => Some("Summarize the conversation"),

            _ => None,
        }
    }

    /// Send a slash command to Claude Code
    pub fn send_slash_command(&mut self, command: &str, cx: &mut Context<Self>) {
        let cmd = if command.starts_with('/') {
            command.to_string()
        } else {
            format!("/{}", command)
        };

        // Show feedback notification for common Claude Code commands
        let notification = match cmd.split_whitespace().next().unwrap_or("") {
            // Core Claude Code commands
            "/think" => Some(("🧠 Entering deep thinking mode...", NotificationType::Info)),
            "/review" => Some(("👀 Starting code review...", NotificationType::Info)),
            "/commit" => Some(("📦 Preparing smart commit...", NotificationType::Info)),
            "/memory" => Some(("💾 Loading memory...", NotificationType::Info)),
            "/compact" => Some(("🗜️ Compacting context...", NotificationType::Info)),
            "/debug" => Some(("🐛 Starting debug analysis...", NotificationType::Info)),
            "/test" => Some(("🧪 Generating tests...", NotificationType::Info)),
            "/resume" => Some(("▶️ Resuming session...", NotificationType::Info)),
            "/status" => Some(("📊 Checking git status...", NotificationType::Info)),
            "/add-dir" => Some(("📁 Adding directory to context...", NotificationType::Info)),

            // Claude Code Skills
            "/apex" => Some((
                "⚡ Starting APEX workflow (Analyze-Plan-Execute-eXamine)...",
                NotificationType::Info,
            )),
            "/brainstorm" => Some(("💡 Starting deep research mode...", NotificationType::Info)),
            "/explore" => Some(("🔍 Exploring codebase...", NotificationType::Info)),
            "/oneshot" => Some((
                "🚀 Ultra-fast implementation mode...",
                NotificationType::Info,
            )),
            "/explain" => Some(("📖 Generating deep explanation...", NotificationType::Info)),
            "/refactor" => Some((
                "♻️ Starting parallel refactoring...",
                NotificationType::Info,
            )),
            "/docs" => Some(("📚 Researching documentation...", NotificationType::Info)),
            "/ultrathink" => Some((
                "🧠 Entering ultra-deep thinking mode...",
                NotificationType::Info,
            )),

            // Git Skills
            "/create-pr" | "/pr" => Some(("🔀 Creating pull request...", NotificationType::Info)),
            "/fix-pr-comments" => Some((
                "💬 Fetching and fixing PR comments...",
                NotificationType::Info,
            )),
            "/merge" => Some(("🔗 Starting intelligent merge...", NotificationType::Info)),

            // Other utilities
            "/help" => Some(("❓ Loading available commands...", NotificationType::Info)),
            "/usage" | "/cost" => Some(("📊 Calculating token usage...", NotificationType::Info)),
            "/doctor" => Some(("🩺 Running diagnostics...", NotificationType::Info)),
            "/mcp" => Some(("🔌 Managing MCP servers...", NotificationType::Info)),
            "/permissions" => Some(("🔐 Loading permissions...", NotificationType::Info)),
            "/init" => Some(("🚀 Initializing project...", NotificationType::Info)),
            "/clear" => Some(("🗑️ Clearing conversation...", NotificationType::Warning)),

            _ => None,
        };

        if let Some((msg, notification_type)) = notification {
            self.show_notification(msg, notification_type, cx);
        }

        cx.emit(ChatViewEvent::Submit(cmd));
    }

    /// Toggle commands panel
    pub fn toggle_commands_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.commands_panel = !self.panels.commands_panel;
        if !self.panels.commands_panel {
            self.commands_filter.clear();
        }
        cx.notify();
    }

    /// Set commands filter
    pub fn set_commands_filter(&mut self, filter: String, cx: &mut Context<Self>) {
        self.commands_filter = filter;
        cx.notify();
    }

    /// Set commands category filter
    pub fn set_commands_category(&mut self, category: CommandCategory, cx: &mut Context<Self>) {
        self.commands_category = category;
        cx.notify();
    }

    /// Get filtered commands based on current filter and category
    pub fn filtered_commands(&self) -> (Vec<String>, Vec<String>) {
        let filter = self.commands_filter.to_lowercase();
        let session = self.session_info.as_ref();

        let slash_commands: Vec<String> = session
            .map(|info| {
                info.slash_commands
                    .iter()
                    .filter(|cmd| filter.is_empty() || cmd.to_lowercase().contains(&filter))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        let skills: Vec<String> = session
            .map(|info| {
                info.skills
                    .iter()
                    .filter(|skill| filter.is_empty() || skill.to_lowercase().contains(&filter))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        (slash_commands, skills)
    }

    /// Use a slash command
    pub fn use_slash_command(&mut self, command: &str, cx: &mut Context<Self>) {
        let cmd = if command.starts_with('/') {
            command.to_string()
        } else {
            format!("/{}", command)
        };
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.insert_text(&cmd, cx);
        });
        self.panels.commands_panel = false;
        cx.notify();
    }

    /// Use a skill
    pub fn use_skill(&mut self, skill: &str, cx: &mut Context<Self>) {
        let cmd = format!("/{}", skill);
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.insert_text(&cmd, cx);
        });
        self.panels.commands_panel = false;
        cx.notify();
    }

    // ==================== Active Tasks ====================

    /// Toggle active tasks panel
    pub fn toggle_tasks_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.tasks_panel = !self.panels.tasks_panel;
        cx.notify();
    }

    /// Get active task count
    pub fn active_task_count(&self) -> usize {
        self.active_tasks.len()
    }

    /// Update task progress
    pub fn update_task_progress(
        &mut self,
        task_id: &str,
        progress: u8,
        status: Option<String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(task) = self
            .active_tasks
            .iter_mut()
            .find(|t| t.task_id.as_deref() == Some(task_id))
        {
            task.progress = Some(progress.min(100));
            task.status = status;
            cx.notify();
        }
    }

    /// Cancel a task by ID (or first task if None)
    pub fn cancel_task(&mut self, task_id: Option<String>, cx: &mut Context<Self>) {
        if let Some(id) = task_id {
            if let Some(pos) = self
                .active_tasks
                .iter()
                .position(|t| t.task_id.as_ref() == Some(&id))
            {
                let task = self.active_tasks.remove(pos);
                self.show_notification(
                    &format!("Cancelled task: {}", task.description),
                    NotificationType::Info,
                    cx,
                );
                // Emit the cancel event
                cx.emit(ChatViewEvent::CancelTask(Some(id)));
            }
        } else if !self.active_tasks.is_empty() {
            // Cancel first task
            let task = self.active_tasks.remove(0);
            self.show_notification(
                &format!("Cancelled task: {}", task.description),
                NotificationType::Info,
                cx,
            );
            cx.emit(ChatViewEvent::CancelTask(task.task_id));
        }
    }
}
