//! Skills section rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::panels::commands::grouping::get_skill_icon;

impl ChatView {
    /// Renders the skills section
    pub(crate) fn render_skills_section(
        &self,
        info: &crate::claude::message::SessionInfo,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child(format!("SKILLS ({})", info.skills.len())),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.6))
                            .child("Click to insert"),
                    ),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(info.skills.iter().take(20).enumerate().map(|(idx, skill)| {
                        let skill_clone = skill.clone();
                        let skill_display = skill.clone();
                        let skill_icon = get_skill_icon(skill);
                        let accent_bg = theme.colors.accent.opacity(0.1);
                        let accent_hover = theme.colors.accent.opacity(0.2);
                        let accent_color = theme.colors.accent;

                        let on_click_listener = cx.listener(move |this, _, _window, cx| {
                            this.insert_slash_command(&skill_clone, cx);
                        });

                        div()
                            .id(ElementId::Name(format!("skill-{}", idx).into()))
                            .px_2()
                            .py(px(2.0))
                            .rounded_sm()
                            .bg(accent_bg)
                            .text_xs()
                            .text_color(accent_color)
                            .cursor_pointer()
                            .hover(move |s| s.bg(accent_hover))
                            .on_click(on_click_listener)
                            .child(format!("{} {}", skill_icon, skill_display))
                    }))
                    .when(info.skills.len() > 20, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", info.skills.len() - 20)),
                        )
                    }),
            )
    }
}
