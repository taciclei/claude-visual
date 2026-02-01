use gpui::*;
use gpui::prelude::*;
use super::super::SettingsModal;

impl SettingsModal {
    /// Render the claude tab
    pub(crate) fn render_claude_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Auto-save
            .child(self.render_section(
                "Conversations",
                "How conversations are handled",
                self.render_toggle(
                    "Auto-save conversations",
                    self.pending.auto_save_conversations,
                    |this, cx| {
                        this.pending.auto_save_conversations = !this.pending.auto_save_conversations;
                        this.mark_changed(cx);
                    },
                    cx,
                ),
                cx,
            ))
            // CLI path
            .child(self.render_section(
                "Claude CLI",
                "Path to the Claude Code CLI (leave empty to use PATH)",
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .w_full()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_sm()
                            .text_color(if self.pending.claude_cli_path.is_some() {
                                theme.colors.text
                            } else {
                                theme.colors.text_muted
                            })
                            .child(
                                self.pending
                                    .claude_cli_path
                                    .as_ref()
                                    .map(|p| p.display().to_string())
                                    .unwrap_or_else(|| "Using system PATH".to_string()),
                            ),
                    ),
                cx,
            ))
            // Claude Code Skills section
            .child(self.render_section(
                "Claude Code Skills",
                "Popular skills for quick access",
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .mb_1()
                            .child("Frequently used skills appear in the quick actions bar:")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_1()
                            .child(self.render_skill_badge("/apex", "Implementation workflow", &theme))
                            .child(self.render_skill_badge("/explore", "Codebase exploration", &theme))
                            .child(self.render_skill_badge("/debug", "Error debugging", &theme))
                            .child(self.render_skill_badge("/commit", "Smart commits", &theme))
                            .child(self.render_skill_badge("/review", "Code review", &theme))
                            .child(self.render_skill_badge("/brainstorm", "Research", &theme))
                    ),
                cx,
            ))
            // Context management
            .child(self.render_section(
                "Context Management",
                "Automatic context compaction and warnings",
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Context warning at 70% usage, urgent at 85%")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .child("Use /compact to free up context space")
                    ),
                cx,
            ))
    }

    /// Render a skill badge
    fn render_skill_badge(&self, skill: &'static str, desc: &'static str, theme: &crate::app::theme::Theme) -> impl IntoElement {
        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(theme.colors.accent.opacity(0.1))
            .border_1()
            .border_color(theme.colors.accent.opacity(0.2))
            .flex()
            .flex_col()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.accent)
                    .child(skill)
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(desc)
            )
    }
}
