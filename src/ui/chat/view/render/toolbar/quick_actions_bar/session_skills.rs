//! Session skills rendering

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

/// Default Claude Code skills to show when no session skills are available
/// Organized by workflow priority
const DEFAULT_SKILLS: &[(&str, &str, &str)] = &[
    ("apex", "⚡", "Full implementation"),
    ("explore", "🔍", "Explore code"),
    ("oneshot", "🚀", "Quick impl"),
    ("debug", "🐛", "Debug"),
    ("commit", "📦", "Commit"),
    ("review", "👀", "Review"),
];

/// Additional skills to show based on context
const CONTEXT_SKILLS: &[(&str, &str, &str, &str)] = &[
    // (skill, icon, label, context_trigger)
    ("ultrathink", "🧠", "Deep Think", "complex"),
    ("brainstorm", "💡", "Research", "research"),
    ("refactor", "♻️", "Refactor", "code"),
    ("clean-code", "✨", "Clean", "messy"),
    ("ci-fixer", "🔧", "CI Fix", "ci"),
    ("create-pr", "🔀", "PR", "staged"),
    ("docs", "📚", "Docs", "doc"),
];

impl ChatView {
    pub(super) fn render_session_skills(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Get popular skills from session (if any)
        let session_skills: Vec<String> = self.session_info.as_ref()
            .map(|info| {
                info.skills.iter()
                    .filter(|s| {
                        // Filter for commonly-used workflow skills
                        let s_lower = s.to_lowercase();
                        s_lower.contains("commit") ||
                        s_lower.contains("review") ||
                        s_lower.contains("test") ||
                        s_lower.contains("fix") ||
                        s_lower.contains("debug") ||
                        s_lower.contains("refactor") ||
                        s_lower.contains("apex") ||
                        s_lower.contains("explore") ||
                        s_lower.contains("brainstorm") ||
                        s_lower.contains("explain") ||
                        s_lower.contains("think") ||
                        s_lower.contains("pr") ||
                        s_lower.contains("merge")
                    })
                    .take(4)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        // If no session skills, show default skills
        let show_defaults = session_skills.is_empty();

        div()
            // Show default skills when no session skills
            .when(show_defaults, |d| {
                d.children(DEFAULT_SKILLS.iter().enumerate().map(|(idx, (skill, icon, _desc))| {
                    let skill_cmd = format!("/{}", skill);
                    let icon = *icon;
                    let skill = *skill;
                    div()
                        .id(ElementId::Name(format!("default-skill-{}", idx).into()))
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .cursor_pointer()
                        .text_xs()
                        .bg(theme.colors.accent.opacity(0.05))
                        .border_1()
                        .border_color(theme.colors.accent.opacity(0.2))
                        .text_color(theme.colors.accent)
                        .hover(|s| s.bg(theme.colors.accent.opacity(0.15)).text_color(theme.colors.accent))
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            cx.emit(ChatViewEvent::Submit(skill_cmd.clone()));
                        }))
                        .child(icon)
                        .child(skill.to_string())
                }))
                .child(
                    div()
                        .w(px(1.0))
                        .h(px(12.0))
                        .bg(theme.colors.border)
                        .mx_1()
                )
            })
            // Show session skills when available
            .when(!show_defaults, |d| {
                d.children(session_skills.iter().enumerate().map(|(idx, skill)| {
                    let skill_cmd = format!("/{}", skill);
                    let skill_icon = if skill.contains("commit") { "📦" }
                        else if skill.contains("review") { "👀" }
                        else if skill.contains("test") { "🧪" }
                        else if skill.contains("fix") || skill.contains("debug") { "🐛" }
                        else if skill.contains("refactor") { "♻️" }
                        else if skill.contains("apex") { "⚡" }
                        else if skill.contains("explore") { "🔍" }
                        else if skill.contains("brainstorm") { "💡" }
                        else if skill.contains("explain") { "📖" }
                        else if skill.contains("think") { "🧠" }
                        else if skill.contains("pr") { "🔀" }
                        else if skill.contains("merge") { "🔗" }
                        else { "⚡" };
                    let skill_name = skill.split(':').last().unwrap_or(skill);
                    div()
                        .id(ElementId::Name(format!("skill-quick-{}", idx).into()))
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .cursor_pointer()
                        .text_xs()
                        .bg(theme.colors.accent.opacity(0.05))
                        .border_1()
                        .border_color(theme.colors.accent.opacity(0.2))
                        .text_color(theme.colors.accent)
                        .hover(|s| s.bg(theme.colors.accent.opacity(0.15)).text_color(theme.colors.accent))
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            cx.emit(ChatViewEvent::Submit(skill_cmd.clone()));
                        }))
                        .child(skill_icon)
                        .child(skill_name.to_string())
                }))
                .child(
                    div()
                        .w(px(1.0))
                        .h(px(12.0))
                        .bg(theme.colors.border)
                        .mx_1()
                )
            })
    }
}
