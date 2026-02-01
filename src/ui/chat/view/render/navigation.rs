//! Navigation render functions for ChatView

use super::super::core::ChatView;
use super::super::types::ChatViewEvent;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_scroll_to_bottom(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let unread = self.unread_count;

        div()
            .absolute()
            .bottom(px(80.0)) // Above input area
            .right(px(20.0))
            .child(
                div()
                    .id("scroll-to-bottom")
                    .relative()
                    .px_3()
                    .py_2()
                    .rounded_full()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_md()
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent)
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.scroll_to_bottom(cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().child("↓"))
                            .when(unread > 0, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text)
                                        .child(format!("{} new", unread)),
                                )
                            }),
                    )
                    // Unread badge
                    .when(unread > 0, |d| {
                        d.child(
                            div()
                                .absolute()
                                .top(px(-4.0))
                                .right(px(-4.0))
                                .size(px(16.0))
                                .rounded_full()
                                .bg(theme.colors.accent)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                        .child(if unread > 9 {
                                            "9+".to_string()
                                        } else {
                                            unread.to_string()
                                        }),
                                ),
                        )
                    }),
            )
    }
    pub fn render_navigation_bar(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let can_back = self.can_navigate_back();
        let can_forward = self.can_navigate_forward();
        let history_len = self.navigation_history.len();
        let pos = self.navigation_history_position;

        div().absolute().bottom(px(80.0)).left(px(20.0)).child(
            div()
                .id("navigation-bar")
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py_1()
                .rounded_lg()
                .bg(theme.colors.surface)
                .border_1()
                .border_color(theme.colors.border)
                .shadow_md()
                // Back button
                .child(
                    div()
                        .id("nav-back")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor(if can_back {
                            CursorStyle::PointingHand
                        } else {
                            CursorStyle::default()
                        })
                        .text_sm()
                        .text_color(if can_back {
                            theme.colors.text
                        } else {
                            theme.colors.text_muted.opacity(0.5)
                        })
                        .when(can_back, |d| d.hover(|s| s.bg(theme.colors.surface_hover)))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.navigate_back(cx);
                        }))
                        .child("◀"),
                )
                // Position indicator
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .px_1()
                        .child(format!("{}/{}", pos, history_len)),
                )
                // Forward button
                .child(
                    div()
                        .id("nav-forward")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor(if can_forward {
                            CursorStyle::PointingHand
                        } else {
                            CursorStyle::default()
                        })
                        .text_sm()
                        .text_color(if can_forward {
                            theme.colors.text
                        } else {
                            theme.colors.text_muted.opacity(0.5)
                        })
                        .when(can_forward, |d| {
                            d.hover(|s| s.bg(theme.colors.surface_hover))
                        })
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.navigate_forward(cx);
                        }))
                        .child("▶"),
                ),
        )
    }
    pub fn render_fab(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        // Build context-aware actions organized by priority
        let mut fab_actions: Vec<(&str, &str, &str, &str)> = Vec::new();

        // ===== URGENT ACTIONS (always first when applicable) =====

        // Context is critical - must compact
        if self.context_usage_percentage() > 0.85 {
            fab_actions.push((
                "🗜️",
                "COMPACT!",
                "/compact",
                "Context critical - compact now",
            ));
        }

        // Error recovery
        if self.last_error.is_some() {
            fab_actions.push(("🐛", "Debug", "/debug", "Debug the error"));
            fab_actions.push(("🔄", "Retry", "retry", "Retry last request"));
        }

        // Response was truncated
        if self.is_last_response_truncated() {
            fab_actions.push(("▶️", "Continue", "continue", "Continue response"));
        }

        // ===== GIT ACTIONS (when relevant) =====
        if let Some(ref git) = self.git_info {
            if git.is_dirty {
                if git.staged_count > 0 {
                    fab_actions.push(("📦", "Commit", "/commit", "Commit staged changes"));
                    fab_actions.push(("🔀", "Create PR", "/create-pr", "Create pull request"));
                }
                fab_actions.push(("👀", "Review", "/review", "Review changes"));
            }
            if git.ahead > 0 {
                fab_actions.push(("⬆️", "Push", "/create-pr", "Create PR / Push"));
            }
        }

        // ===== IMPLEMENTATION SKILLS =====
        fab_actions.push(("⚡", "APEX", "/apex", "Full implementation workflow"));
        fab_actions.push(("🚀", "Oneshot", "/oneshot", "Quick implementation"));
        fab_actions.push(("🧠", "Ultrathink", "/ultrathink", "Deep analysis mode"));

        // ===== EXPLORATION & UNDERSTANDING =====
        fab_actions.push(("🔍", "Explore", "/explore", "Explore codebase"));
        fab_actions.push(("📖", "Explain", "/explain", "Explain code"));
        fab_actions.push(("🔎", "Search", "/search", "Quick search"));

        // ===== CODE QUALITY =====
        fab_actions.push(("👀", "Review Code", "/review-code", "Review code quality"));
        fab_actions.push(("♻️", "Refactor", "/refactor", "Refactor code"));
        fab_actions.push(("✨", "Clean Code", "/clean-code", "Apply best practices"));
        fab_actions.push(("🐛", "Debug", "/debug", "Debug issues"));

        // ===== RESEARCH & DOCUMENTATION =====
        fab_actions.push(("💡", "Brainstorm", "/brainstorm", "Deep research mode"));
        fab_actions.push(("📚", "Docs", "/docs", "Research documentation"));

        // ===== CI/CD =====
        fab_actions.push(("🔧", "CI Fixer", "/ci-fixer", "Fix CI failures"));

        // ===== SESSION MANAGEMENT =====
        fab_actions.push(("📊", "Usage", "/usage", "Token usage"));
        fab_actions.push(("📝", "Memory", "/memory", "Save to CLAUDE.md"));

        // Context management (if not already urgent)
        if self.context_usage_percentage() > 0.5 && self.context_usage_percentage() <= 0.85 {
            fab_actions.push(("🗜️", "Compact", "/compact", "Free up context"));
        }

        if !self.messages.is_empty() {
            fab_actions.push(("📋", "Summary", "/summarize", "Summarize conversation"));
        }

        // ===== SESSION PANELS =====
        fab_actions.push(("📝", "Notes", "notes", "Session notes"));
        fab_actions.push(("📌", "Pinned", "pinned", "View pinned messages"));
        fab_actions.push(("📊", "Stats", "stats", "View statistics"));
        fab_actions.push(("📜", "History", "history", "Session history"));
        fab_actions.push(("⚙️", "Settings", "settings", "Quick settings"));
        fab_actions.push(("❓", "Shortcuts", "shortcuts", "Keyboard shortcuts"));

        // Limit to top 14 most relevant actions (urgent ones are already first)
        fab_actions.truncate(14);

        div()
            .absolute()
            .bottom(px(140.0)) // Above input and scroll btn
            .right(px(20.0))
            .flex()
            .flex_col()
            .items_end()
            .gap_2()
            // Menu items (when expanded)
            .when(self.show_fab_menu, |d| {
                d.children(fab_actions.iter().enumerate().map(
                    |(idx, (icon, label, cmd, _desc))| {
                        let command = cmd.to_string();
                        div()
                            .id(ElementId::Name(format!("fab-action-{}", idx).into()))
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border)
                            .shadow_md()
                            .cursor_pointer()
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .border_color(theme.colors.accent)
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.show_fab_menu = false;
                                match command.as_str() {
                                    "export" => cx.emit(ChatViewEvent::ExportRequested),
                                    "notes" => this.toggle_notes_panel(cx),
                                    "pinned" => this.toggle_pinned_panel(cx),
                                    "stats" => this.toggle_stats_panel(cx),
                                    "settings" => this.toggle_quick_settings(cx),
                                    "shortcuts" => this.toggle_shortcuts_help(cx),
                                    "history" => this.toggle_session_history(cx),
                                    "retry" => this.retry_last_request(cx),
                                    "continue" => this.continue_conversation(cx),
                                    cmd if cmd.starts_with('/') => {
                                        cx.emit(ChatViewEvent::Submit(cmd.to_string()))
                                    }
                                    _ => {}
                                }
                            }))
                            .child(div().text_sm().child(*icon))
                            .child(div().text_xs().text_color(theme.colors.text).child(*label))
                    },
                ))
            })
            // Main FAB button
            .child(
                div()
                    .id("fab-main")
                    .size(px(48.0))
                    .rounded_full()
                    .bg(theme.colors.accent)
                    .shadow_lg()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.8)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_fab_menu(cx);
                    }))
                    .child(
                        div()
                            .text_lg()
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(if self.show_fab_menu { "×" } else { "⚡" }),
                    ),
            )
    }
}
