//! Streaming suggestions - contextual skill suggestions during streaming

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

/// Get contextual skill suggestions based on current tool
pub fn get_tool_suggestions(tool_name: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match tool_name.to_lowercase().as_str() {
        "read" => vec![
            ("📖", "Explain", "/explain"),
            ("♻️", "Refactor", "/refactor"),
            ("🧪", "Tests", "Write tests for this file"),
        ],
        "write" | "edit" => vec![
            ("👀", "Review", "/review-code"),
            ("✨", "Clean", "/clean-code"),
            ("🐛", "Debug", "/debug"),
        ],
        "bash" => vec![
            ("🐛", "Debug", "/debug"),
            ("🔧", "CI Fix", "/ci-fixer"),
            ("📖", "Explain", "Explain this command"),
        ],
        "grep" | "glob" => vec![
            ("🔍", "Explore", "/explore"),
            ("🔎", "Search", "/search"),
            ("📖", "Explain", "/explain"),
        ],
        "task" => vec![
            ("⚡", "APEX", "/apex"),
            ("🧠", "Think", "/ultrathink"),
            ("🚀", "Oneshot", "/oneshot"),
        ],
        "websearch" | "webfetch" => vec![
            ("💡", "Brainstorm", "/brainstorm"),
            ("📚", "Docs", "/docs"),
            ("🔍", "Explore", "/explore"),
        ],
        _ => vec![
            ("⚡", "APEX", "/apex"),
            ("🔍", "Explore", "/explore"),
            ("🐛", "Debug", "/debug"),
        ],
    }
}

/// Get suggestions based on content being streamed
pub fn get_content_suggestions(content: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    let content_lower = content.to_lowercase();

    // Code-related content
    if content.contains("```")
        || content.contains("fn ")
        || content.contains("function ")
        || content.contains("class ")
        || content.contains("def ")
    {
        return vec![
            ("👀", "Review", "/review-code"),
            ("🧪", "Tests", "Write tests for this"),
            ("📖", "Explain", "/explain"),
        ];
    }

    // Error-related content
    if content_lower.contains("error")
        || content_lower.contains("failed")
        || content_lower.contains("exception")
    {
        return vec![
            ("🐛", "Debug", "/debug"),
            ("🔧", "Fix", "Fix this error"),
            ("📖", "Explain", "Explain this error"),
        ];
    }

    // Git-related content
    if content_lower.contains("commit")
        || content_lower.contains("branch")
        || content_lower.contains("merge")
        || content_lower.contains("pull request")
    {
        return vec![
            ("📦", "Commit", "/commit"),
            ("🔀", "PR", "/create-pr"),
            ("👀", "Review", "/review"),
        ];
    }

    // Test-related content
    if content_lower.contains("test") || content_lower.contains("spec") {
        return vec![
            ("🧪", "Run Tests", "Run these tests"),
            ("🐛", "Debug", "/debug"),
            ("✨", "Clean", "/clean-code"),
        ];
    }

    // Documentation
    if content_lower.contains("readme") || content_lower.contains("documentation") {
        return vec![
            ("📚", "Docs", "/docs"),
            ("📖", "Explain", "/explain"),
            ("💡", "Brainstorm", "/brainstorm"),
        ];
    }

    // Default suggestions
    vec![
        ("💬", "Continue", "Continue"),
        ("📖", "Explain", "Explain more"),
        ("🔍", "Explore", "/explore"),
    ]
}

impl ChatView {
    /// Render streaming suggestions bar
    pub fn render_streaming_suggestions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        if !self.streaming.is_streaming {
            return div();
        }

        // Get suggestions based on current tool or content
        let suggestions = if let Some(ref tool_name) = self.current_tool_name {
            get_tool_suggestions(tool_name)
        } else if let Some(ref content) = self.streaming.current_message {
            get_content_suggestions(content)
        } else {
            vec![
                ("⚡", "APEX", "/apex"),
                ("🔍", "Explore", "/explore"),
                ("📦", "Commit", "/commit"),
            ]
        };

        // Don't show while actively streaming text (only show when tool is running)
        if self.current_tool_name.is_none() && self.streaming.token_count > 10 {
            return div();
        }

        div()
            .w_full()
            .px_4()
            .py_1()
            .bg(theme.colors.surface.opacity(0.3))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.2))
            .flex()
            .items_center()
            .gap_2()
            // Label
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.5))
                    .child("Next:"),
            )
            // Suggestions
            .children(suggestions.into_iter().map(|(icon, label, action)| {
                let action_str = action.to_string();
                div()
                    .id(ElementId::Name(format!("stream-sug-{}", label).into()))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_px()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.6))
                    .bg(theme.colors.background.opacity(0.5))
                    .border_1()
                    .border_color(theme.colors.border.opacity(0.3))
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .text_color(theme.colors.text)
                            .border_color(theme.colors.accent.opacity(0.4))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        // Queue the suggestion for after streaming stops
                        this.input.update(cx, |input, cx| {
                            input.set_text(action_str.clone(), cx);
                        });
                    }))
                    .child(icon)
                    .child(label)
            }))
            // Stop hint
            .child(
                div()
                    .ml_auto()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.4))
                    .child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border.opacity(0.3))
                            .font_family("monospace")
                            .child("⌘."),
                    )
                    .child("stop"),
            )
    }

    /// Render post-response suggestions based on what Claude did
    pub fn render_post_response_suggestions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        // Only show if not streaming and we have a recent response
        if self.streaming.is_streaming || self.messages.is_empty() {
            return div();
        }

        // Check last message for context-aware suggestions
        let last_content = self
            .messages
            .last()
            .map(|m| m.content.clone())
            .unwrap_or_default();
        let content_lower = last_content.to_lowercase();
        let is_truncated = self.is_last_response_truncated();

        let suggestions = if last_content.contains("```") {
            vec![
                ("👀", "Review Code", "/review-code"),
                ("🧪", "Generate Tests", "Write tests for this code"),
                ("♻️", "Refactor", "/refactor"),
                ("📖", "Explain", "/explain"),
            ]
        } else if content_lower.contains("error") || content_lower.contains("failed") {
            vec![
                ("🐛", "Debug", "/debug"),
                ("🔄", "Retry", "retry"),
                ("📖", "Explain Error", "Explain this error in detail"),
            ]
        } else if content_lower.contains("commit") || content_lower.contains("changes") {
            vec![
                ("📦", "Commit", "/commit"),
                ("🔀", "Create PR", "/create-pr"),
                ("👀", "Review", "/review"),
            ]
        } else if is_truncated {
            vec![
                ("➡️", "Continue", "Continue"),
                ("📖", "Summarize", "Summarize what you were saying"),
                ("🔍", "Expand", "Expand on this"),
            ]
        } else {
            // Default follow-up suggestions
            vec![
                ("💬", "Continue", "Continue"),
                ("📖", "Explain More", "Explain more"),
                ("⚡", "APEX", "/apex"),
            ]
        };

        // Check if context is getting full
        let ctx_pct = self.context_usage_percentage();
        let context_warning = ctx_pct > 0.7;
        let ctx_display = format!("{}% ctx", (ctx_pct * 100.0) as i32);

        // Copy colors for closures
        let warning_bg = theme.colors.warning.opacity(0.1);
        let warning_border = theme.colors.warning.opacity(0.2);
        let warning_color = theme.colors.warning;

        div()
            .w_full()
            .px_4()
            .py_2()
            .bg(theme.colors.surface.opacity(0.3))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.2))
            .flex()
            .items_center()
            .gap_2()
            // Label
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.6))
                    .child("Follow up:"),
            )
            // Suggestions
            .children(suggestions.into_iter().map(|(icon, label, action)| {
                let action_str = action.to_string();
                div()
                    .id(ElementId::Name(format!("post-sug-{}", label).into()))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .bg(theme.colors.accent.opacity(0.1))
                    .border_1()
                    .border_color(theme.colors.accent.opacity(0.2))
                    .text_color(theme.colors.accent)
                    .hover(|s| {
                        s.bg(theme.colors.accent.opacity(0.2))
                            .border_color(theme.colors.accent.opacity(0.4))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        if action_str == "retry" {
                            this.retry_last_request(cx);
                        } else {
                            cx.emit(ChatViewEvent::Submit(action_str.clone()));
                        }
                    }))
                    .child(icon)
                    .child(label)
            }))
            // Context warning (pre-rendered to avoid closure issues)
            .when(context_warning, |d| {
                d.child(
                    div()
                        .ml_auto()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(warning_bg)
                        .border_1()
                        .border_color(warning_border)
                        .text_xs()
                        .text_color(warning_color)
                        .child("⚠️")
                        .child(ctx_display.clone())
                        .child("·")
                        .child("/compact"),
                )
            })
    }
}
