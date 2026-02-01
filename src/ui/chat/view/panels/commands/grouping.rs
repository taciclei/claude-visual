//! Skill categorization and grouping logic

/// Skill metadata with description and usage
pub struct SkillInfo {
    pub description: &'static str,
    pub usage_example: &'static str,
}

/// Get detailed info for a skill
pub fn get_skill_info(skill: &str) -> Option<SkillInfo> {
    let sl = skill.to_lowercase();

    // Implementation skills
    if sl.contains("apex") && !sl.contains("legacy") && !sl.contains("quick") {
        return Some(SkillInfo {
            description: "Systematic implementation using Analyze-Plan-Execute-eXamine methodology with parallel agents and self-validation",
            usage_example: "/apex Add user authentication to the login page",
        });
    }
    if sl == "oneshot" {
        return Some(SkillInfo {
            description: "Ultra-fast feature implementation - Explore, Code, then Test in one shot",
            usage_example: "/oneshot Add a dark mode toggle button",
        });
    }
    if sl == "ultrathink" {
        return Some(SkillInfo {
            description: "Deep thinking mode - approach problems like a craftsman with obsessive attention to detail",
            usage_example: "/ultrathink Design the optimal database schema for this app",
        });
    }

    // Exploration skills
    if sl == "explore" {
        return Some(SkillInfo {
            description: "Deep exploration of codebase, docs, and web for any topic or question",
            usage_example: "/explore How does the authentication system work?",
        });
    }
    if sl == "search" {
        return Some(SkillInfo {
            description: "Lightning-fast search to answer specific questions - optimized for speed",
            usage_example: "/search Where is the user model defined?",
        });
    }
    if sl == "explain" {
        return Some(SkillInfo {
            description: "Deep explanation of code features with tracing, analysis, and visual diagrams",
            usage_example: "/explain How does the payment flow work?",
        });
    }
    if sl == "docs" {
        return Some(SkillInfo {
            description: "Deep documentation research using parallel agents",
            usage_example: "/docs How do I use TanStack Query mutations?",
        });
    }

    // Code quality skills
    if sl == "review" || sl == "review-code" {
        return Some(SkillInfo {
            description: "Expert code review covering security (OWASP), clean code (SOLID), complexity metrics, and high-value feedback",
            usage_example: "/review Check the authentication module for security issues",
        });
    }
    if sl == "refactor" {
        return Some(SkillInfo {
            description: "Refactor code by finding files, grouping them, and launching parallel agents",
            usage_example: "/refactor Extract common validation logic into a shared module",
        });
    }
    if sl == "clean-code" {
        return Some(SkillInfo {
            description: "Comprehensive clean code workflow analyzing codebase and applying best practices for React, Next.js, and modern tooling",
            usage_example: "/clean-code Improve the components in src/ui/",
        });
    }
    if sl == "debug" || sl.contains("debug-skills") {
        return Some(SkillInfo {
            description: "Systematic error debugging with analysis, solution discovery, and verification",
            usage_example: "/debug Fix the TypeError in the login component",
        });
    }

    // Research skills
    if sl == "brainstorm" {
        return Some(SkillInfo {
            description: "Deep iterative research using progressive flow psychology with parallel agents and multi-perspective synthesis",
            usage_example: "/brainstorm What's the best approach for real-time notifications?",
        });
    }

    // Git operations
    if sl.contains("git:commit") || sl == "commit" {
        return Some(SkillInfo {
            description: "Quick commit and push with minimal, clean messages",
            usage_example: "/commit Fix authentication bug",
        });
    }
    if sl.contains("git:create-pr") || sl == "create-pr" {
        return Some(SkillInfo {
            description: "Create and push PR with auto-generated title and description",
            usage_example: "/create-pr",
        });
    }
    if sl.contains("git:merge") || sl == "merge" {
        return Some(SkillInfo {
            description: "Intelligently merge branches with context-aware conflict resolution",
            usage_example: "/merge main",
        });
    }
    if sl.contains("ci-fixer") {
        return Some(SkillInfo {
            description: "Automated CI/CD pipeline fixer - watches CI, fixes errors locally, commits, and loops until green",
            usage_example: "/ci-fixer",
        });
    }

    // Documentation skills
    if sl == "claude-memory" {
        return Some(SkillInfo {
            description: "Create and optimize CLAUDE.md memory files or .claude/rules/ modular rules",
            usage_example: "/claude-memory Set up memory for this project",
        });
    }
    if sl == "add-llm-comments" {
        return Some(SkillInfo {
            description: "Add minimal, intelligent JSDoc comments to help LLMs understand how to use a file",
            usage_example: "/add-llm-comments src/utils/auth.ts",
        });
    }

    // Skill creation
    if sl == "create-agent" || sl == "create-subagents" {
        return Some(SkillInfo {
            description: "Expert guidance for creating Claude Code subagents and the Task tool",
            usage_example: "/create-agent Build a specialized testing agent",
        });
    }
    if sl == "create-hooks" {
        return Some(SkillInfo {
            description: "Expert guidance for creating and configuring Claude Code hooks (PreToolUse, PostToolUse, Stop, etc.)",
            usage_example: "/create-hooks Set up auto-formatting on file save",
        });
    }
    if sl == "create-slash-commands" {
        return Some(SkillInfo {
            description: "Expert guidance for creating custom Claude Code slash commands with YAML configuration",
            usage_example: "/create-slash-commands Build a /deploy command",
        });
    }
    if sl == "create-prompt" {
        return Some(SkillInfo {
            description: "Expert prompt engineering for creating effective prompts for Claude and other LLMs",
            usage_example: "/create-prompt Write a prompt for code review",
        });
    }

    // Utilities
    if sl.contains("utils:auto-fix") {
        return Some(SkillInfo {
            description: "Automated workflow to fix all ESLint and TypeScript errors with parallel processing",
            usage_example: "/auto-fix",
        });
    }
    if sl.contains("utils:fix-grammar") {
        return Some(SkillInfo {
            description: "Fix grammar and spelling errors in files while preserving formatting",
            usage_example: "/fix-grammar README.md",
        });
    }
    if sl.contains("utils:watch-ci") {
        return Some(SkillInfo {
            description: "Monitor CI pipeline and automatically fix failures until green",
            usage_example: "/watch-ci",
        });
    }

    None
}

/// Get a short description for a skill (for tooltips)
pub fn get_skill_description(skill: &str) -> &'static str {
    get_skill_info(skill)
        .map(|info| info.description)
        .unwrap_or("Claude Code skill")
}

/// Groups skills into categories based on name patterns
pub fn group_skills_by_category(skills: &[String]) -> Vec<(&'static str, Vec<&String>)> {
    vec![
        // Implementation skills
        (
            "⚡ Implementation",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("apex")
                        || sl.contains("oneshot")
                        || sl.contains("ultrathink")
                        || sl.contains("plan")
                        || sl == "implement"
                })
                .collect(),
        ),
        // Exploration skills
        (
            "🔍 Exploration",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("explore")
                        || sl.contains("search")
                        || sl.contains("explain")
                        || sl.contains("docs")
                        || sl == "find"
                })
                .collect(),
        ),
        // Code quality skills
        (
            "✨ Code Quality",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("review")
                        || sl.contains("refactor")
                        || sl.contains("clean")
                        || sl.contains("debug")
                        || sl.contains("fix")
                        || sl.contains("lint")
                })
                .collect(),
        ),
        // Research skills
        (
            "💡 Research",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("brainstorm") || sl.contains("research")
                })
                .collect(),
        ),
        // Git operations
        (
            "📦 Git & CI",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("git")
                        || sl.contains("commit")
                        || sl.contains("pr")
                        || sl.contains("merge")
                        || sl.contains("ci")
                        || sl.contains("push")
                })
                .collect(),
        ),
        // Documentation
        (
            "📚 Documentation",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("doc")
                        || sl.contains("comment")
                        || sl.contains("readme")
                        || sl.contains("memory")
                        || sl.contains("claude-md")
                })
                .collect(),
        ),
        // Testing
        (
            "🧪 Testing",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("test") || sl.contains("spec")
                })
                .collect(),
        ),
        // Skill creation
        (
            "🛠️ Skill Creation",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("create-") || sl.contains("keybinding") || sl.contains("hook")
                })
                .collect(),
        ),
        // Utilities
        (
            "⚙️ Utilities",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    sl.contains("auto-")
                        || sl.contains("watch")
                        || sl.contains("grammar")
                        || sl.contains("utils")
                })
                .collect(),
        ),
        // Other (catch-all)
        (
            "📂 Other",
            skills
                .iter()
                .filter(|s| {
                    let sl = s.to_lowercase();
                    // Exclude all categorized skills
                    !sl.contains("apex")
                        && !sl.contains("oneshot")
                        && !sl.contains("ultrathink")
                        && !sl.contains("plan")
                        && !sl.contains("explore")
                        && !sl.contains("search")
                        && !sl.contains("explain")
                        && !sl.contains("docs")
                        && !sl.contains("review")
                        && !sl.contains("refactor")
                        && !sl.contains("clean")
                        && !sl.contains("debug")
                        && !sl.contains("fix")
                        && !sl.contains("lint")
                        && !sl.contains("brainstorm")
                        && !sl.contains("research")
                        && !sl.contains("git")
                        && !sl.contains("commit")
                        && !sl.contains("pr")
                        && !sl.contains("merge")
                        && !sl.contains("ci")
                        && !sl.contains("push")
                        && !sl.contains("doc")
                        && !sl.contains("comment")
                        && !sl.contains("readme")
                        && !sl.contains("memory")
                        && !sl.contains("claude-md")
                        && !sl.contains("test")
                        && !sl.contains("spec")
                        && !sl.contains("create-")
                        && !sl.contains("keybinding")
                        && !sl.contains("hook")
                        && !sl.contains("auto-")
                        && !sl.contains("watch")
                        && !sl.contains("grammar")
                        && !sl.contains("utils")
                })
                .collect(),
        ),
    ]
}

/// Get an icon for a skill based on its name
pub fn get_skill_icon(skill: &str) -> &'static str {
    let sl = skill.to_lowercase();

    // Implementation
    if sl.contains("apex") {
        return "⚡";
    }
    if sl.contains("oneshot") {
        return "🚀";
    }
    if sl.contains("ultrathink") || sl.contains("think") {
        return "🧠";
    }
    if sl.contains("plan") {
        return "📋";
    }

    // Exploration
    if sl.contains("explore") {
        return "🔍";
    }
    if sl.contains("search") {
        return "🔎";
    }
    if sl.contains("explain") {
        return "📖";
    }

    // Code Quality
    if sl.contains("review") {
        return "👀";
    }
    if sl.contains("refactor") {
        return "♻️";
    }
    if sl.contains("clean") {
        return "✨";
    }
    if sl.contains("debug") {
        return "🐛";
    }
    if sl.contains("fix") && sl.contains("ci") {
        return "🔧";
    }
    if sl.contains("fix") {
        return "🔧";
    }
    if sl.contains("lint") {
        return "🔍";
    }

    // Research
    if sl.contains("brainstorm") {
        return "💡";
    }
    if sl.contains("research") {
        return "🔬";
    }

    // Git & CI
    if sl.contains("commit") {
        return "📦";
    }
    if sl.contains("pr") || sl.contains("pull") {
        return "🔀";
    }
    if sl.contains("merge") {
        return "🔗";
    }
    if sl.contains("ci") {
        return "🔧";
    }
    if sl.contains("push") {
        return "⬆️";
    }

    // Documentation
    if sl.contains("docs") || sl.contains("doc") {
        return "📚";
    }
    if sl.contains("comment") {
        return "💬";
    }
    if sl.contains("memory") || sl.contains("claude-md") {
        return "📝";
    }

    // Testing
    if sl.contains("test") {
        return "🧪";
    }
    if sl.contains("spec") {
        return "📋";
    }

    // Skill creation
    if sl.contains("create-skill") {
        return "🛠️";
    }
    if sl.contains("create-hook") {
        return "🔗";
    }
    if sl.contains("create-prompt") {
        return "✍️";
    }
    if sl.contains("create-agent") {
        return "🤖";
    }
    if sl.contains("keybinding") {
        return "⌨️";
    }

    // Utilities
    if sl.contains("auto-") {
        return "🔄";
    }
    if sl.contains("watch") {
        return "👁️";
    }
    if sl.contains("grammar") {
        return "📝";
    }

    // Default
    "⚡"
}
