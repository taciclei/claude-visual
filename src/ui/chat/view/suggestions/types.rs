//! Suggestion types

/// Contextual suggestion for the user
#[derive(Debug, Clone)]
pub struct ContextualSuggestion {
    /// Suggestion text/prompt
    pub text: String,
    /// Icon for the suggestion
    pub icon: &'static str,
    /// Category (e.g., "code", "explain", "fix")
    pub category: &'static str,
    /// Priority (higher = shown first)
    pub priority: u8,
}

impl ContextualSuggestion {
    pub fn new(
        text: impl Into<String>,
        icon: &'static str,
        category: &'static str,
        priority: u8,
    ) -> Self {
        Self {
            text: text.into(),
            icon,
            category,
            priority,
        }
    }
}

/// A quick reply suggestion based on conversation context
#[derive(Debug, Clone)]
pub struct QuickReplySuggestion {
    /// Display label for the suggestion
    pub label: String,
    /// The prompt text to insert/send
    pub prompt: String,
    /// Category of suggestion (e.g., "follow-up", "clarify", "action")
    pub category: &'static str,
    /// Icon or emoji for the suggestion
    pub icon: &'static str,
}

impl QuickReplySuggestion {
    /// Create a new quick reply suggestion
    pub fn new(
        label: impl Into<String>,
        prompt: impl Into<String>,
        category: &'static str,
        icon: &'static str,
    ) -> Self {
        Self {
            label: label.into(),
            prompt: prompt.into(),
            category,
            icon,
        }
    }

    /// Create common follow-up suggestions
    pub fn common_followups() -> Vec<Self> {
        vec![
            Self::new(
                "Explain more",
                "Can you explain that in more detail?",
                "clarify",
                "💡",
            ),
            Self::new(
                "Show example",
                "Can you show me an example?",
                "clarify",
                "📝",
            ),
            Self::new("Why?", "Why is that the case?", "clarify", "❓"),
            Self::new(
                "Alternative?",
                "What's an alternative approach?",
                "explore",
                "🔀",
            ),
            Self::new(
                "Best practices?",
                "What are the best practices for this?",
                "explore",
                "⭐",
            ),
            Self::new(
                "Continue",
                "Please continue where you left off.",
                "action",
                "▶️",
            ),
            Self::new(
                "Summarize",
                "Can you summarize what we discussed?",
                "action",
                "📋",
            ),
            Self::new(
                "Code example",
                "Can you provide a code example?",
                "action",
                "💻",
            ),
        ]
    }
}
