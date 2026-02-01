//! Agents section rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the agents section
    pub(crate) fn render_agents_section(
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
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text_muted)
                    .child(format!("AGENTS ({})", info.agents.len())),
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
                    .children(info.agents.iter().take(10).enumerate().map(|(idx, agent)| {
                        let agent_clone = agent.clone();
                        let agent_prompt = format!("Use the {} agent to ", agent.to_lowercase());
                        let info_bg = theme.colors.info.opacity(0.1);
                        let info_hover = theme.colors.info.opacity(0.2);
                        let info_color = theme.colors.info;

                        let on_click_listener = cx.listener(move |this, _, _window, cx| {
                            this.input.update(cx, |input, cx| {
                                input.clear(cx);
                                input.insert_text(&agent_prompt, cx);
                            });
                            this.panels.session_details = false;
                            cx.notify();
                        });

                        div()
                            .id(ElementId::Name(format!("agent-{}", idx).into()))
                            .px_2()
                            .py(px(2.0))
                            .rounded_sm()
                            .bg(info_bg)
                            .text_xs()
                            .text_color(info_color)
                            .cursor_pointer()
                            .hover(move |s| s.bg(info_hover))
                            .on_click(on_click_listener)
                            .child(agent_clone)
                    }))
                    .when(info.agents.len() > 10, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", info.agents.len() - 10)),
                        )
                    }),
            )
    }
}
