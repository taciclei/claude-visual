//! Prompt templates and favorites

/// A favorite/saved prompt for quick access
#[derive(Debug, Clone)]
pub struct FavoritePrompt {
    /// Unique ID
    pub id: String,
    /// The prompt text
    pub text: String,
    /// Short label/name
    pub label: String,
    /// When it was saved
    pub saved_at: chrono::DateTime<chrono::Utc>,
    /// Usage count
    pub usage_count: u32,
    /// Category/tag
    pub category: Option<String>,
}

impl FavoritePrompt {
    pub fn new(text: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.into(),
            label: label.into(),
            saved_at: chrono::Utc::now(),
            usage_count: 0,
            category: None,
        }
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

/// A saved prompt template for quick reuse
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// Unique ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template content
    pub content: String,
    /// Category (e.g., "coding", "review", "explain", "custom")
    pub category: &'static str,
    /// Icon for display
    pub icon: &'static str,
    /// Whether this is a built-in template
    pub is_builtin: bool,
    /// Usage count
    pub usage_count: u32,
}

impl PromptTemplate {
    /// Create a new template
    pub fn new(name: impl Into<String>, content: impl Into<String>, category: &'static str, icon: &'static str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            content: content.into(),
            category,
            icon,
            is_builtin: false,
            usage_count: 0,
        }
    }

    /// Create a built-in template
    pub fn builtin(name: &'static str, content: &'static str, category: &'static str, icon: &'static str) -> Self {
        Self {
            id: format!("builtin-{}", name.to_lowercase().replace(' ', "-")),
            name: name.to_string(),
            content: content.to_string(),
            category,
            icon,
            is_builtin: true,
            usage_count: 0,
        }
    }
}

/// Built-in prompt templates for common tasks
pub fn default_prompt_templates() -> Vec<PromptTemplate> {
    vec![
        // Coding templates
        PromptTemplate::builtin(
            "Fix Bug",
            "Please analyze and fix this bug. The issue is:\n\n",
            "coding", "🐛"
        ),
        PromptTemplate::builtin(
            "Add Feature",
            "Please implement the following feature:\n\n",
            "coding", "✨"
        ),
        PromptTemplate::builtin(
            "Refactor Code",
            "Please refactor this code to improve:\n- Readability\n- Performance\n- Maintainability\n\n",
            "coding", "♻️"
        ),
        PromptTemplate::builtin(
            "Write Tests",
            "Please write comprehensive tests for this code, including:\n- Unit tests\n- Edge cases\n- Error handling\n\n",
            "coding", "🧪"
        ),
        PromptTemplate::builtin(
            "Add Documentation",
            "Please add documentation including:\n- Function/class docstrings\n- Inline comments for complex logic\n- Usage examples\n\n",
            "coding", "📝"
        ),

        // Review templates
        PromptTemplate::builtin(
            "Code Review",
            "Please review this code for:\n- Bugs and potential issues\n- Performance problems\n- Security vulnerabilities\n- Best practices\n\n",
            "review", "🔍"
        ),
        PromptTemplate::builtin(
            "Security Audit",
            "Please perform a security audit checking for:\n- Input validation issues\n- Authentication/authorization flaws\n- Data exposure risks\n- OWASP top 10 vulnerabilities\n\n",
            "review", "🔒"
        ),

        // Explain templates
        PromptTemplate::builtin(
            "Explain Code",
            "Please explain this code in detail:\n- What it does\n- How it works\n- Key algorithms/patterns used\n\n",
            "explain", "💡"
        ),
        PromptTemplate::builtin(
            "Explain Error",
            "Please help me understand this error and how to fix it:\n\n",
            "explain", "❓"
        ),

        // Git templates
        PromptTemplate::builtin(
            "Create Commit",
            "Please create a descriptive commit message for these changes:\n\n",
            "git", "📦"
        ),
        PromptTemplate::builtin(
            "Create PR Description",
            "Please write a pull request description covering:\n- Summary of changes\n- Testing performed\n- Breaking changes (if any)\n\n",
            "git", "🔀"
        ),

        // Architecture templates
        PromptTemplate::builtin(
            "Design System",
            "Please help me design a system with these requirements:\n- Scalability needs\n- Performance constraints\n- Technology preferences\n\n",
            "architecture", "🏗️"
        ),
        PromptTemplate::builtin(
            "API Design",
            "Please help me design a REST API for:\n\n",
            "architecture", "🔌"
        ),

        // Claude Code specific templates
        PromptTemplate::builtin(
            "Deep Analysis",
            "/think\nPlease analyze this problem deeply and consider all edge cases before implementing:\n\n",
            "claude", "🧠"
        ),
        PromptTemplate::builtin(
            "Review Changes",
            "/review\nPlease review my recent changes for issues.",
            "claude", "👀"
        ),
        PromptTemplate::builtin(
            "Smart Commit",
            "/commit\nPlease create a commit with a descriptive message.",
            "claude", "📦"
        ),
        PromptTemplate::builtin(
            "APEX Workflow",
            "/apex\nI need to implement a feature using the APEX methodology (Analyze-Plan-Execute-eXamine). Here's what I need:\n\n",
            "claude", "⚡"
        ),
        PromptTemplate::builtin(
            "Deep Research",
            "/brainstorm\nI need to deeply research and analyze this topic:\n\n",
            "claude", "💡"
        ),
        PromptTemplate::builtin(
            "Debug Issue",
            "/debug\nI'm experiencing this issue:\n- What's happening:\n- Expected behavior:\n- Error message (if any):\n\n",
            "claude", "🐛"
        ),
        PromptTemplate::builtin(
            "Explore Codebase",
            "/explore\nHelp me understand this codebase:\n- What are the main components?\n- How does the data flow?\n- What patterns are used?\n\n",
            "claude", "🔍"
        ),
        PromptTemplate::builtin(
            "Save Memory",
            "/memory\nPlease remember the following important context about this project:\n\n",
            "claude", "💾"
        ),
        PromptTemplate::builtin(
            "Create PR",
            "/pr\nPlease create a pull request with the following:\n- Title:\n- Description:\n\n",
            "claude", "🔀"
        ),

        // Advanced coding templates
        PromptTemplate::builtin(
            "Optimize Performance",
            "Please analyze this code for performance issues and suggest optimizations:\n- Time complexity improvements\n- Memory usage reduction\n- Caching opportunities\n\n",
            "coding", "⚡"
        ),
        PromptTemplate::builtin(
            "Add Error Handling",
            "Please add comprehensive error handling to this code:\n- Input validation\n- Try/catch blocks\n- User-friendly error messages\n- Logging\n\n",
            "coding", "🛡️"
        ),
        PromptTemplate::builtin(
            "Debug This",
            "Help me debug this issue. Here's what I'm experiencing:\n- Expected behavior:\n- Actual behavior:\n- Steps to reproduce:\n\n",
            "coding", "🔧"
        ),
        PromptTemplate::builtin(
            "Type Safety",
            "Please add type annotations and improve type safety for this code:\n\n",
            "coding", "📋"
        ),

        // Workflow templates - Multi-step guided workflows
        PromptTemplate::builtin(
            "Bug Fix Workflow",
            "🔧 **Bug Fix Workflow**\n\n1. First, let me understand the bug: /debug\n2. Then implement the fix\n3. Finally: /commit to save changes\n\n**Bug Description:**\n",
            "workflow", "🔧"
        ),
        PromptTemplate::builtin(
            "Feature Workflow",
            "⚡ **Feature Implementation Workflow**\n\n1. /apex - Start structured APEX workflow\n2. Analyze requirements and plan\n3. Implement step-by-step\n4. /review - Review implementation\n5. /commit - Commit changes\n\n**Feature Request:**\n",
            "workflow", "⚡"
        ),
        PromptTemplate::builtin(
            "Refactor Workflow",
            "♻️ **Refactor Workflow**\n\n1. /explore - Understand current code\n2. Plan refactoring strategy\n3. /refactor - Execute parallel refactoring\n4. /review - Verify changes\n5. /commit - Save improvements\n\n**What to refactor:**\n",
            "workflow", "♻️"
        ),
        PromptTemplate::builtin(
            "Research Workflow",
            "💡 **Deep Research Workflow**\n\n1. /brainstorm - Deep research mode\n2. Analyze multiple perspectives\n3. /docs - Consult documentation\n4. Synthesize conclusions\n\n**Research Topic:**\n",
            "workflow", "💡"
        ),
        PromptTemplate::builtin(
            "Quick Fix Workflow",
            "🚀 **Quick Fix Workflow**\n\nFor simple, focused changes:\n/oneshot - Ultra-fast: Explore → Code → Test\n\n**What needs to be fixed:**\n",
            "workflow", "🚀"
        ),
        PromptTemplate::builtin(
            "Code Review Workflow",
            "👀 **Code Review Workflow**\n\n1. /review - Expert review (security, SOLID, clean code)\n2. Analyze findings\n3. Implement fixes\n4. /commit - Save improvements\n\n**Code/files to review:**\n",
            "workflow", "👀"
        ),
        PromptTemplate::builtin(
            "PR Workflow",
            "🔀 **Pull Request Workflow**\n\n1. /review - Final code review\n2. /commit - Commit all changes\n3. /create-pr - Create PR with description\n\n**PR Purpose:**\n",
            "workflow", "🔀"
        ),
        PromptTemplate::builtin(
            "Deep Thinking Workflow",
            "🧠 **Deep Thinking Workflow**\n\nFor complex problems requiring careful analysis:\n\n1. /ultrathink - Approach like a craftsman\n2. Obsess over details\n3. Create elegant solutions\n\n**Problem to solve:**\n",
            "workflow", "🧠"
        ),
    ]
}
