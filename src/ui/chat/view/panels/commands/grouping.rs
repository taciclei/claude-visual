//! Skill categorization and grouping logic

/// Groups skills into categories based on name patterns
pub fn group_skills_by_category(skills: &[String]) -> Vec<(&'static str, Vec<&String>)> {
    vec![
        // Implementation skills
        ("⚡ Implementation", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("apex") || sl.contains("oneshot") || sl.contains("ultrathink") ||
            sl.contains("plan") || sl == "implement"
        }).collect()),
        // Exploration skills
        ("🔍 Exploration", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("explore") || sl.contains("search") || sl.contains("explain") ||
            sl.contains("docs") || sl == "find"
        }).collect()),
        // Code quality skills
        ("✨ Code Quality", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("review") || sl.contains("refactor") || sl.contains("clean") ||
            sl.contains("debug") || sl.contains("fix") || sl.contains("lint")
        }).collect()),
        // Research skills
        ("💡 Research", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("brainstorm") || sl.contains("research")
        }).collect()),
        // Git operations
        ("📦 Git & CI", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("git") || sl.contains("commit") || sl.contains("pr") ||
            sl.contains("merge") || sl.contains("ci") || sl.contains("push")
        }).collect()),
        // Documentation
        ("📚 Documentation", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("doc") || sl.contains("comment") || sl.contains("readme") ||
            sl.contains("memory") || sl.contains("claude-md")
        }).collect()),
        // Testing
        ("🧪 Testing", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("test") || sl.contains("spec")
        }).collect()),
        // Skill creation
        ("🛠️ Skill Creation", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("create-") || sl.contains("keybinding") || sl.contains("hook")
        }).collect()),
        // Utilities
        ("⚙️ Utilities", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            sl.contains("auto-") || sl.contains("watch") || sl.contains("grammar") ||
            sl.contains("utils")
        }).collect()),
        // Other (catch-all)
        ("📂 Other", skills.iter().filter(|s| {
            let sl = s.to_lowercase();
            // Exclude all categorized skills
            !sl.contains("apex") && !sl.contains("oneshot") && !sl.contains("ultrathink") && !sl.contains("plan") &&
            !sl.contains("explore") && !sl.contains("search") && !sl.contains("explain") && !sl.contains("docs") &&
            !sl.contains("review") && !sl.contains("refactor") && !sl.contains("clean") && !sl.contains("debug") && !sl.contains("fix") && !sl.contains("lint") &&
            !sl.contains("brainstorm") && !sl.contains("research") &&
            !sl.contains("git") && !sl.contains("commit") && !sl.contains("pr") && !sl.contains("merge") && !sl.contains("ci") && !sl.contains("push") &&
            !sl.contains("doc") && !sl.contains("comment") && !sl.contains("readme") && !sl.contains("memory") && !sl.contains("claude-md") &&
            !sl.contains("test") && !sl.contains("spec") &&
            !sl.contains("create-") && !sl.contains("keybinding") && !sl.contains("hook") &&
            !sl.contains("auto-") && !sl.contains("watch") && !sl.contains("grammar") && !sl.contains("utils")
        }).collect()),
    ]
}

/// Get an icon for a skill based on its name
pub fn get_skill_icon(skill: &str) -> &'static str {
    let sl = skill.to_lowercase();

    // Implementation
    if sl.contains("apex") { return "⚡"; }
    if sl.contains("oneshot") { return "🚀"; }
    if sl.contains("ultrathink") || sl.contains("think") { return "🧠"; }
    if sl.contains("plan") { return "📋"; }

    // Exploration
    if sl.contains("explore") { return "🔍"; }
    if sl.contains("search") { return "🔎"; }
    if sl.contains("explain") { return "📖"; }

    // Code Quality
    if sl.contains("review") { return "👀"; }
    if sl.contains("refactor") { return "♻️"; }
    if sl.contains("clean") { return "✨"; }
    if sl.contains("debug") { return "🐛"; }
    if sl.contains("fix") && sl.contains("ci") { return "🔧"; }
    if sl.contains("fix") { return "🔧"; }
    if sl.contains("lint") { return "🔍"; }

    // Research
    if sl.contains("brainstorm") { return "💡"; }
    if sl.contains("research") { return "🔬"; }

    // Git & CI
    if sl.contains("commit") { return "📦"; }
    if sl.contains("pr") || sl.contains("pull") { return "🔀"; }
    if sl.contains("merge") { return "🔗"; }
    if sl.contains("ci") { return "🔧"; }
    if sl.contains("push") { return "⬆️"; }

    // Documentation
    if sl.contains("docs") || sl.contains("doc") { return "📚"; }
    if sl.contains("comment") { return "💬"; }
    if sl.contains("memory") || sl.contains("claude-md") { return "📝"; }

    // Testing
    if sl.contains("test") { return "🧪"; }
    if sl.contains("spec") { return "📋"; }

    // Skill creation
    if sl.contains("create-skill") { return "🛠️"; }
    if sl.contains("create-hook") { return "🔗"; }
    if sl.contains("create-prompt") { return "✍️"; }
    if sl.contains("create-agent") { return "🤖"; }
    if sl.contains("keybinding") { return "⌨️"; }

    // Utilities
    if sl.contains("auto-") { return "🔄"; }
    if sl.contains("watch") { return "👁️"; }
    if sl.contains("grammar") { return "📝"; }

    // Default
    "⚡"
}
