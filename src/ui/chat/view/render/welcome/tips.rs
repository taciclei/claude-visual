//! Welcome tips render function for ChatView

use gpui::*;
use gpui::prelude::*;
use super::super::super::core::ChatView;
use crate::app::theme::Theme;

impl ChatView {
    pub fn render_welcome_tips(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        // Select tips based on context
        let mut tips: Vec<(&str, &str, &str, &str)> = Vec::new();

        // Always show keyboard tip first
        tips.push(("tip-keyboard", "⌨️", "Keyboard shortcuts", "Press ⌘? for all shortcuts, Ctrl+L clears, Ctrl+R searches history"));

        // If git repo, show git-specific tips
        if self.git_info.is_some() {
            tips.push(("tip-commit", "📦", "Git workflows", "Use /commit, /create-pr, /merge for smart git operations"));
            tips.push(("tip-review", "👀", "Code review", "Use /review for expert code review with security analysis"));
        }

        // Core Claude Code skills - highlight the most powerful
        tips.push(("tip-skills", "⚡", "Claude Code Skills", "Type / to see all skills: /apex, /brainstorm, /explore, /debug, /oneshot"));
        tips.push(("tip-mention", "@", "File mentions", "Type @ to reference files in your prompts for context-aware help"));
        tips.push(("tip-ultrathink", "🧠", "Deep thinking", "Use /ultrathink for craftsman-level analysis and elegant solutions"));

        // If MCP servers are connected, highlight them
        if let Some(ref info) = self.session_info {
            if !info.mcp_servers.is_empty() {
                let tool_count: usize = info.mcp_servers.iter().map(|s| s.tool_count).sum();
                tips.push(("tip-mcp-connected", "🔌", "MCP Connected",
                    if tool_count > 0 { "External tools available! Click MCP button to explore" }
                    else { "MCP servers connected. Use their capabilities in your prompts" }));
            }
        }

        // Workflow tips based on what might be most useful
        tips.push(("tip-apex", "⚡", "APEX methodology", "Use /apex for systematic Analyze-Plan-Execute-eXamine workflow"));
        tips.push(("tip-explore", "🔍", "Codebase exploration", "Use /explore to understand code, /search for quick answers"));
        tips.push(("tip-brainstorm", "💡", "Research mode", "Use /brainstorm for deep research with multiple perspectives"));
        tips.push(("tip-memory", "💾", "Persistent memory", "Use /memory to save important context across sessions"));
        tips.push(("tip-docs", "📖", "Documentation", "Use /docs for documentation research, /explain for code explanations"));

        // Limit to 6 tips for cleaner UI
        tips.truncate(6);

        div()
            .w_full()
            .max_w(px(600.0))
            .flex()
            .flex_col()
            .gap_2()
            .children(
                tips.iter()
                    .filter(|(id, _, _, _)| self.should_show_tip(id))
                    .map(|(tip_id, icon, title, description)| {
                        let tip_id_owned = tip_id.to_string();
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .p_3()
                            .rounded_lg()
                            .bg(theme.colors.info.opacity(0.05))
                            .border_1()
                            .border_color(theme.colors.info.opacity(0.2))
                            // Icon
                            .child(
                                div()
                                    .w(px(32.0))
                                    .h(px(32.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_md()
                                    .bg(theme.colors.info.opacity(0.1))
                                    .text_sm()
                                    .child(*icon)
                            )
                            // Content
                            .child(
                                div()
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.text)
                                            .child(*title)
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(*description)
                                    )
                            )
                            // Dismiss button
                            .child(
                                div()
                                    .id(ElementId::Name(format!("dismiss-welcome-tip-{}", tip_id).into()))
                                    .w(px(24.0))
                                    .h(px(24.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.dismiss_tip_by_string(&tip_id_owned, cx);
                                    }))
                                    .child("×")
                            )
                    })
            )
    }
}
