//! Utility functions for chat input rendering

/// Get description for common Claude CLI commands and skills
pub fn get_command_description(cmd: &str) -> &'static str {
    match cmd.to_lowercase().as_str() {
        // ===== Claude CLI Core Commands =====
        "resume" => "Resume a previous conversation",
        "usage" => "Show token usage and costs",
        "compact" => "Compact conversation context",
        "clear" => "Clear current conversation",
        "help" => "Show available commands",
        "config" => "Open configuration",
        "model" => "Switch AI model (opus, sonnet, haiku)",
        "memory" => "Manage CLAUDE.md persistent memory",
        "permissions" => "Manage tool permissions",
        "login" => "Login to Anthropic",
        "logout" => "Logout from Anthropic",
        "status" => "Show connection status",
        "context" => "Manage context files",
        "vim" => "Toggle vim mode",
        "bug" => "Report a bug",
        "pr-comments" => "View PR comments",
        "init" => "Initialize project settings",
        "add-dir" => "Add directory to context",
        "terminal-setup" => "Setup terminal integration",
        "listen" => "Enable voice input",
        "cost" => "Show session cost breakdown",
        "doctor" => "Diagnose Claude Code issues",
        "mcp" => "Manage MCP servers",
        "think" => "Enable extended thinking mode",
        "hooks" => "Manage Claude Code hooks",
        "allowed-tools" => "Configure allowed tools",
        "task" => "Run task from file or issue",

        // ===== Implementation Skills =====
        "apex" => "Full APEX workflow (Analyze-Plan-Execute-Examine)",
        "oneshot" => "Ultra-fast single-task implementation",
        "ultrathink" => "Deep thinking mode for complex problems",
        "plan" => "Create structured implementation plan",

        // ===== Exploration & Understanding =====
        "explore" => "Deep codebase exploration",
        "search" => "Lightning-fast answer search",
        "explain" => "Code explanation with visual diagrams",
        "docs" => "Research library documentation",

        // ===== Code Quality =====
        "review-code" => "Expert code review (security, SOLID, smells)",
        "review" => "Review code changes and PRs",
        "refactor" => "Parallel code refactoring",
        "clean-code" => "Apply clean code best practices",
        "debug" => "Systematic error debugging",
        "add-llm-comments" => "Add intelligent JSDoc comments",

        // ===== Research & Analysis =====
        "brainstorm" => "Deep research with skeptical analysis",

        // ===== Git & CI/CD =====
        "commit" => "Smart commit with conventional messages",
        "create-pr" => "Create pull request with description",
        "fix-pr-comments" => "Implement PR review feedback",
        "merge" => "Context-aware branch merging",
        "pr" => "Create a pull request",
        "ci-fixer" => "Auto-fix CI/CD failures",

        // ===== Session Management =====
        "summarize" => "Summarize conversation or code",
        "test" => "Run and analyze tests",

        // ===== Skill Creation =====
        "keybindings-help" => "Customize keyboard shortcuts",
        "create-hooks" => "Create Claude Code hooks",
        "create-skills" => "Create custom workflow skills",
        "create-skills-workflow" => "Create multi-step workflow skills",
        "create-meta-prompts" => "Create Claude-to-Claude pipelines",
        "create-slash-commands" => "Create custom slash commands",
        "create-subagents" => "Build specialized subagents",
        "create-agent" => "Create custom Claude agents",
        "create-prompt" => "Expert prompt engineering",
        "claude-memory" => "Create CLAUDE.md memory files",
        "create-context-docs" => "Create context documentation",

        // ===== Utility Skills =====
        "auto-fix" => "Auto-fix ESLint and TypeScript errors",
        "debug-ccli" => "Debug Claude CLI errors",
        "watch-ci" => "Monitor and fix CI failures",
        "fix-grammar" => "Fix grammar and spelling",

        // ===== Marketing & Business =====
        "copywriting" => "Marketing copywriting specialist",

        _ => "Claude CLI command",
    }
}

/// Get icon for a Claude Code skill/command
pub fn get_command_icon(cmd: &str) -> &'static str {
    match cmd.to_lowercase().as_str() {
        // Implementation
        "apex" => "⚡",
        "oneshot" => "🚀",
        "ultrathink" => "🧠",
        "plan" => "📋",

        // Exploration
        "explore" => "🔍",
        "search" => "🔎",
        "explain" => "📖",
        "docs" => "📚",

        // Code Quality
        "review-code" | "review" => "👀",
        "refactor" => "♻️",
        "clean-code" => "✨",
        "debug" => "🐛",

        // Research
        "brainstorm" => "💡",

        // Git
        "commit" => "📦",
        "create-pr" | "pr" => "🔀",
        "fix-pr-comments" => "💬",
        "merge" => "🔗",
        "ci-fixer" => "🔧",

        // Session
        "usage" | "cost" => "📊",
        "compact" => "🗜️",
        "memory" => "📝",
        "doctor" => "🩺",

        _ => "⚙️",
    }
}

/// Fuzzy match result for command autocomplete
#[derive(Clone)]
pub struct CommandMatch {
    pub command: String,
    pub score: i32,
    pub matched_indices: Vec<usize>,
}

/// File match result for file mention autocomplete
#[derive(Clone)]
pub struct FileMatch {
    /// Full path to the file
    pub path: String,
    /// Display name (filename or relative path)
    pub display: String,
    /// Fuzzy match score
    pub score: i32,
    /// Matched character indices in display
    pub matched_indices: Vec<usize>,
    /// File type icon
    pub icon: &'static str,
}

/// Get icon for file based on extension
pub fn get_file_icon(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext.to_lowercase().as_str() {
        "rs" => "🦀",
        "py" => "🐍",
        "js" | "jsx" | "mjs" => "📜",
        "ts" | "tsx" => "📘",
        "json" => "📋",
        "toml" | "yaml" | "yml" => "⚙️",
        "md" | "mdx" => "📝",
        "html" | "htm" => "🌐",
        "css" | "scss" | "sass" => "🎨",
        "sql" => "🗃️",
        "sh" | "bash" | "zsh" => "💻",
        "go" => "🐹",
        "java" => "☕",
        "c" | "h" => "⚡",
        "cpp" | "cc" | "cxx" | "hpp" => "⚡",
        "swift" => "🍎",
        "kt" | "kts" => "🟣",
        "rb" => "💎",
        "php" => "🐘",
        "vue" => "💚",
        "svelte" => "🔥",
        _ => "📄",
    }
}

/// Perform fuzzy matching on file paths
pub fn fuzzy_match_files(files: &[String], query: &str) -> Vec<FileMatch> {
    if query.is_empty() {
        return files
            .iter()
            .take(10) // Limit initial results
            .map(|path| {
                let display = path.rsplit('/').next().unwrap_or(path).to_string();
                FileMatch {
                    path: path.clone(),
                    display: display.clone(),
                    score: 0,
                    matched_indices: vec![],
                    icon: get_file_icon(path),
                }
            })
            .collect();
    }

    let query_lower = query.to_lowercase();
    let query_chars: Vec<char> = query_lower.chars().collect();

    let mut matches: Vec<FileMatch> = files
        .iter()
        .filter_map(|path| {
            let filename = path.rsplit('/').next().unwrap_or(path);
            let filename_lower = filename.to_lowercase();
            let path_lower = path.to_lowercase();

            // First try exact filename match
            if filename_lower == query_lower {
                let indices: Vec<usize> = (0..query.len()).collect();
                return Some(FileMatch {
                    path: path.clone(),
                    display: filename.to_string(),
                    score: 300,
                    matched_indices: indices,
                    icon: get_file_icon(path),
                });
            }

            // Then try filename prefix match
            if filename_lower.starts_with(&query_lower) {
                let indices: Vec<usize> = (0..query.len()).collect();
                return Some(FileMatch {
                    path: path.clone(),
                    display: filename.to_string(),
                    score: 200 + (20 - filename.len() as i32).max(0),
                    matched_indices: indices,
                    icon: get_file_icon(path),
                });
            }

            // Try fuzzy match on filename
            if let Some((score, indices)) = fuzzy_score_file(&filename_lower, &query_chars) {
                return Some(FileMatch {
                    path: path.clone(),
                    display: filename.to_string(),
                    score: score + 50, // Bonus for filename match
                    matched_indices: indices,
                    icon: get_file_icon(path),
                });
            }

            // Try fuzzy match on full path (lower priority)
            if let Some((score, indices)) = fuzzy_score_file(&path_lower, &query_chars) {
                // Adjust indices to match display (last component)
                let path_offset = path.len() - filename.len();
                let adjusted_indices: Vec<usize> = indices
                    .iter()
                    .filter_map(|&i| {
                        if i >= path_offset {
                            Some(i - path_offset)
                        } else {
                            None
                        }
                    })
                    .collect();

                return Some(FileMatch {
                    path: path.clone(),
                    display: filename.to_string(),
                    score,
                    matched_indices: adjusted_indices,
                    icon: get_file_icon(path),
                });
            }

            None
        })
        .collect();

    // Sort by score descending
    matches.sort_by(|a, b| b.score.cmp(&a.score));
    matches.truncate(15); // Limit to 15 results
    matches
}

/// Calculate fuzzy score for file matching
fn fuzzy_score_file(text: &str, query_chars: &[char]) -> Option<(i32, Vec<usize>)> {
    let text_chars: Vec<char> = text.chars().collect();
    let mut query_idx = 0;
    let mut score: i32 = 0;
    let mut matched_indices = Vec::with_capacity(query_chars.len());
    let mut prev_match_idx: Option<usize> = None;

    for (i, &c) in text_chars.iter().enumerate() {
        if query_idx < query_chars.len() && c == query_chars[query_idx] {
            matched_indices.push(i);

            // Scoring bonuses
            if i == 0 {
                score += 15; // Start of string
            } else if text_chars
                .get(i.saturating_sub(1))
                .map(|c| *c == '/' || *c == '-' || *c == '_' || *c == '.')
                .unwrap_or(false)
            {
                score += 12; // Start of path segment or word
            }

            // Consecutive match bonus
            if let Some(prev) = prev_match_idx {
                if i == prev + 1 {
                    score += 8;
                }
            }

            prev_match_idx = Some(i);
            query_idx += 1;
        }
    }

    if query_idx == query_chars.len() {
        // Length penalty
        score = score.saturating_sub((text_chars.len() - query_chars.len()) as i32 / 3);
        Some((score.max(1), matched_indices))
    } else {
        None
    }
}

/// Perform fuzzy matching on commands
pub fn fuzzy_match_commands(commands: &[String], query: &str) -> Vec<CommandMatch> {
    if query.is_empty() {
        return commands
            .iter()
            .map(|cmd| CommandMatch {
                command: cmd.clone(),
                score: 0,
                matched_indices: vec![],
            })
            .collect();
    }

    let query_lower = query.to_lowercase();
    let query_chars: Vec<char> = query_lower.chars().collect();

    let mut matches: Vec<CommandMatch> = commands
        .iter()
        .filter_map(|cmd| {
            let cmd_lower = cmd.to_lowercase();

            // First try prefix match (highest priority)
            if cmd_lower.starts_with(&query_lower) {
                let indices: Vec<usize> = (0..query.len()).collect();
                return Some(CommandMatch {
                    command: cmd.clone(),
                    score: 200 + (20 - cmd.len() as i32).max(0), // Shorter commands score higher
                    matched_indices: indices,
                });
            }

            // Then try fuzzy match
            if let Some((score, indices)) = fuzzy_score(&cmd_lower, &query_chars) {
                return Some(CommandMatch {
                    command: cmd.clone(),
                    score,
                    matched_indices: indices,
                });
            }

            None
        })
        .collect();

    // Sort by score descending
    matches.sort_by(|a, b| b.score.cmp(&a.score));
    matches
}

/// Calculate fuzzy score for a single command
fn fuzzy_score(text: &str, query_chars: &[char]) -> Option<(i32, Vec<usize>)> {
    let text_chars: Vec<char> = text.chars().collect();
    let mut query_idx = 0;
    let mut score: i32 = 0;
    let mut matched_indices = Vec::with_capacity(query_chars.len());
    let mut prev_match_idx: Option<usize> = None;

    for (i, &c) in text_chars.iter().enumerate() {
        if query_idx < query_chars.len() && c == query_chars[query_idx] {
            matched_indices.push(i);

            // Scoring bonuses
            if i == 0 {
                score += 15; // Start of string
            } else if text_chars
                .get(i.saturating_sub(1))
                .map(|c| *c == '-' || *c == '_')
                .unwrap_or(false)
            {
                score += 10; // Start of word
            }

            // Consecutive match bonus
            if let Some(prev) = prev_match_idx {
                if i == prev + 1 {
                    score += 5;
                }
            }

            prev_match_idx = Some(i);
            query_idx += 1;
        }
    }

    if query_idx == query_chars.len() {
        // Length penalty for longer commands
        score = score.saturating_sub((text_chars.len() - query_chars.len()) as i32 / 2);
        Some((score.max(1), matched_indices))
    } else {
        None
    }
}
