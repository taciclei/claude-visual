//! Shortcuts help render functions for ChatView

use super::super::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_shortcuts_help(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let shortcuts = [
            (
                "General",
                vec![
                    ("⌘K", "Command palette"),
                    ("⌘F", "Search in conversation"),
                    ("⌘B", "Toggle sidebar"),
                    ("⌘?", "Show/hide shortcuts"),
                    ("⌘.", "Stop response"),
                    ("⌘T", "Templates menu"),
                    ("Escape", "Close panel / Cancel"),
                ],
            ),
            (
                "Input",
                vec![
                    ("⏎", "Send message"),
                    ("⇧⏎", "New line"),
                    ("/", "Slash commands"),
                    ("@", "Mention file"),
                    ("⌃L", "Clear conversation"),
                    ("⌃R", "Search history"),
                    ("⌃U", "Clear input line"),
                    ("⌃K", "Kill to end of line"),
                ],
            ),
            (
                "Navigation",
                vec![
                    ("⌥↑", "Select previous message"),
                    ("⌥↓", "Select next message"),
                    ("⌘↑", "Scroll to top"),
                    ("⌘↓", "Scroll to bottom"),
                    ("↑", "Browse input history"),
                ],
            ),
            (
                "⚡ Implementation",
                vec![
                    ("/apex", "Full APEX workflow"),
                    ("/oneshot", "Ultra-fast implement"),
                    ("/ultrathink", "Deep thinking mode"),
                    ("/plan", "Create impl plan"),
                ],
            ),
            (
                "🔍 Exploration",
                vec![
                    ("/explore", "Explore codebase"),
                    ("/search", "Quick answer search"),
                    ("/explain", "Deep explanation"),
                    ("/docs", "Research docs"),
                ],
            ),
            (
                "✨ Code Quality",
                vec![
                    ("/review-code", "Expert code review"),
                    ("/refactor", "Parallel refactoring"),
                    ("/clean-code", "Apply best practices"),
                    ("/debug", "Systematic debugging"),
                    ("/ci-fixer", "Fix CI failures"),
                ],
            ),
            ("💡 Research", vec![("/brainstorm", "Deep research")]),
            (
                "📦 Git & CI",
                vec![
                    ("/commit", "Smart commit"),
                    ("/create-pr", "Create PR"),
                    ("/review", "Review changes"),
                    ("/merge", "Intelligent merge"),
                    ("/fix-pr-comments", "Fix PR feedback"),
                ],
            ),
            (
                "📊 Session",
                vec![
                    ("/compact", "Compact context"),
                    ("/memory", "Save to CLAUDE.md"),
                    ("/resume", "Resume session"),
                    ("/usage", "Token usage"),
                    ("/model", "Switch model"),
                    ("/think", "Extended thinking"),
                    ("/doctor", "System health"),
                ],
            ),
            (
                "🛠️ Create",
                vec![
                    ("/create-skills", "Create skills"),
                    ("/create-hooks", "Create hooks"),
                    ("/create-prompt", "Prompt engineering"),
                ],
            ),
            (
                "View",
                vec![
                    ("⌥W", "Toggle word wrap"),
                    ("⌥L", "Toggle line numbers"),
                    ("⌃⇧V", "Toggle vim mode"),
                    ("⌘M", "Switch model"),
                ],
            ),
            (
                "Permissions",
                vec![
                    ("A", "Approve all"),
                    ("D", "Deny all"),
                    ("⏎", "Approve first"),
                ],
            ),
        ];

        div()
            .id("shortcuts-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_shortcuts_help(cx);
            }))
            .child(
                div()
                    .id("shortcuts-panel")
                    .max_w(px(600.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .rounded_xl()
                    .shadow_xl()
                    .p_6()
                    .on_click(|_, _window, cx| {
                        // Prevent closing when clicking inside the panel
                        cx.stop_propagation();
                    })
                    // Header
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .mb_4()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.text)
                                    .child("Keyboard Shortcuts"),
                            )
                            .child(
                                div()
                                    .id("close-shortcuts")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| {
                                        s.bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_shortcuts_help(cx);
                                    }))
                                    .child("✕"),
                            ),
                    )
                    // Shortcuts grid
                    .child(
                        div()
                            .id("shortcuts-content")
                            .overflow_y_scroll()
                            .max_h(px(400.0))
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            .children(shortcuts.iter().map(|(section, items)| {
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .min_w(px(220.0))
                                    // Section header
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.accent)
                                            .mb_1()
                                            .child(*section),
                                    )
                                    // Items
                                    .children(items.iter().map(|(key, desc)| {
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .gap_4()
                                            .py(px(2.0))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(*desc),
                                            )
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py(px(2.0))
                                                    .rounded_sm()
                                                    .bg(theme.colors.background)
                                                    .border_1()
                                                    .border_color(theme.colors.border)
                                                    .text_xs()
                                                    .font_family("monospace")
                                                    .text_color(theme.colors.text)
                                                    .child(*key),
                                            )
                                    }))
                            })),
                    )
                    // Footer hint
                    .child(
                        div()
                            .mt_4()
                            .pt_3()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .text_center()
                            .child("Press ⌘? or Escape to close"),
                    ),
            )
    }
}
