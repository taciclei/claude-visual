//! Empty state rendering

use super::core::McpServersPanel;
use super::types::McpServersPanelEvent;
use gpui::prelude::*;
use gpui::*;

impl McpServersPanel {
    /// Render the empty state
    pub(crate) fn render_empty(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let accent = theme.colors.accent;
        let info = theme.colors.info;

        div()
            .px_4()
            .py_6()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(
                div()
                    .size(px(48.0))
                    .rounded_full()
                    .bg(theme.colors.text_muted.opacity(0.1))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().text_xl().child("🔌")),
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .child("No MCP servers configured"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .text_center()
                    .max_w(px(280.0))
                    .child(
                        "MCP servers extend Claude's capabilities with custom tools and resources.",
                    ),
            )
            .child(
                div()
                    .mt_1()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.surface_hover)
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("Configure in ~/.claude/settings.json"),
            )
            // Quick skill suggestions
            .child(
                div()
                    .pt_3()
                    .flex()
                    .flex_wrap()
                    .justify_center()
                    .gap_2()
                    // Learn about MCP
                    .child(
                        div()
                            .id("mcp-servers-empty-docs")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(accent.opacity(0.15))
                            .border_1()
                            .border_color(accent.opacity(0.3))
                            .text_xs()
                            .text_color(accent)
                            .hover(move |s| {
                                s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5))
                            })
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(McpServersPanelEvent::SendSkillCommand(
                                    "How do I configure MCP servers for Claude Code?".to_string(),
                                ));
                            }))
                            .child("📚 Learn MCP"),
                    )
                    // Explore docs
                    .child(
                        div()
                            .id("mcp-servers-empty-explore")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(info.opacity(0.15))
                            .border_1()
                            .border_color(info.opacity(0.3))
                            .text_xs()
                            .text_color(info)
                            .hover(move |s| {
                                s.bg(info.opacity(0.25)).border_color(info.opacity(0.5))
                            })
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(McpServersPanelEvent::SendSkillCommand(
                                    "/docs mcp".to_string(),
                                ));
                            }))
                            .child("🔍 MCP Docs"),
                    ),
            )
    }
}
