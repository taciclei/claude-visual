//! Type definitions for code actions

/// Code action kind for categorization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeActionKind {
    /// Quick fix for an error/warning
    QuickFix,
    /// Refactoring action
    Refactor,
    /// Refactor extract (extract method, variable, etc.)
    RefactorExtract,
    /// Refactor inline
    RefactorInline,
    /// Refactor rewrite
    RefactorRewrite,
    /// Source action (organize imports, etc.)
    Source,
    /// Organize imports
    SourceOrganizeImports,
    /// Fix all
    SourceFixAll,
    /// Unknown/other kind
    Other(String),
}

impl CodeActionKind {
    /// Parse from LSP code action kind string
    pub fn from_lsp(kind: &str) -> Self {
        match kind {
            "quickfix" => Self::QuickFix,
            "refactor" => Self::Refactor,
            "refactor.extract" => Self::RefactorExtract,
            "refactor.inline" => Self::RefactorInline,
            "refactor.rewrite" => Self::RefactorRewrite,
            "source" => Self::Source,
            "source.organizeImports" => Self::SourceOrganizeImports,
            "source.fixAll" => Self::SourceFixAll,
            other => Self::Other(other.to_string()),
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &str {
        match self {
            Self::QuickFix => "Quick Fix",
            Self::Refactor => "Refactor",
            Self::RefactorExtract => "Extract",
            Self::RefactorInline => "Inline",
            Self::RefactorRewrite => "Rewrite",
            Self::Source => "Source",
            Self::SourceOrganizeImports => "Organize Imports",
            Self::SourceFixAll => "Fix All",
            Self::Other(_) => "Action",
        }
    }

    /// Get icon for the action kind
    pub fn icon(&self) -> &str {
        match self {
            Self::QuickFix => "💡",
            Self::Refactor
            | Self::RefactorExtract
            | Self::RefactorInline
            | Self::RefactorRewrite => "🔧",
            Self::Source | Self::SourceOrganizeImports | Self::SourceFixAll => "📦",
            Self::Other(_) => "⚡",
        }
    }

    /// Check if this is a preferred action
    pub fn is_preferred(&self) -> bool {
        matches!(self, Self::QuickFix | Self::SourceFixAll)
    }
}

/// Code action item
#[derive(Debug, Clone)]
pub struct CodeActionItem {
    /// Unique ID
    pub id: usize,
    /// Action title
    pub title: String,
    /// Action kind
    pub kind: CodeActionKind,
    /// Whether this is the preferred action
    pub is_preferred: bool,
    /// Associated diagnostics (if any)
    pub diagnostics: Vec<String>,
    /// Server that provided this action
    pub server: String,
    /// Original data for execution
    pub data: Option<String>,
}

impl CodeActionItem {
    /// Create a new code action item
    pub fn new(id: usize, title: String, kind: CodeActionKind) -> Self {
        Self {
            id,
            title,
            kind,
            is_preferred: false,
            diagnostics: Vec::new(),
            server: String::new(),
            data: None,
        }
    }

    /// Set as preferred
    pub fn preferred(mut self) -> Self {
        self.is_preferred = true;
        self
    }

    /// Add diagnostic
    pub fn with_diagnostic(mut self, diagnostic: String) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    /// Set server
    pub fn with_server(mut self, server: String) -> Self {
        self.server = server;
        self
    }
}

/// Events from code actions panel
#[derive(Debug, Clone)]
pub enum CodeActionsEvent {
    /// Execute a code action
    Execute(usize),
    /// Dismiss the panel
    Dismiss,
    /// Preview action (show what will change)
    Preview(usize),
}

/// Inline code action indicator (lightbulb)
pub struct CodeActionIndicator {
    pub has_actions: bool,
    pub has_quick_fix: bool,
}

impl CodeActionIndicator {
    /// Create a new indicator
    pub fn new() -> Self {
        Self {
            has_actions: false,
            has_quick_fix: false,
        }
    }

    /// Update indicator state
    pub fn update(&mut self, actions: &[CodeActionItem]) {
        self.has_actions = !actions.is_empty();
        self.has_quick_fix = actions.iter().any(|a| a.kind == CodeActionKind::QuickFix);
    }

    /// Get indicator icon
    pub fn icon(&self) -> Option<&str> {
        if self.has_quick_fix {
            Some("💡")
        } else if self.has_actions {
            Some("🔧")
        } else {
            None
        }
    }
}

impl Default for CodeActionIndicator {
    fn default() -> Self {
        Self::new()
    }
}
