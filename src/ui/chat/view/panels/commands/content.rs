//! Commands panel content rendering

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;
use super::super::super::types::CommandCategory;
use super::grouping::{self, get_skill_info};

pub fn render_content(
    theme: &crate::app::theme::Theme,
    category: CommandCategory,
    slash_commands: &[String],
    skills: &[String],
    skill_categories: &[(&str, Vec<&String>)],
    hovered_skill: &Option<String>,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .overflow_hidden()
        .child(
            div()
                .id("commands-list")
                .flex_1()
                .overflow_y_scroll()
                .p_2()
                .when(
                    category == CommandCategory::All || category == CommandCategory::SlashCommands,
                    |d| {
                        d.when(!slash_commands.is_empty(), |d| {
                            d.child(render_slash_commands_section(theme, slash_commands, cx))
                        })
                    },
                )
                .when(
                    category == CommandCategory::All || category == CommandCategory::Skills,
                    |d| {
                        d.when(!skills.is_empty(), |d| {
                            d.child(render_skills_section(theme, skills, skill_categories, cx))
                        })
                    },
                )
                .when(slash_commands.is_empty() && skills.is_empty(), |d| {
                    d.child(render_empty_state(theme, cx))
                }),
        )
        .when_some(hovered_skill.clone(), |d, skill| {
            d.child(render_skill_preview(theme, &skill, cx))
        })
}

fn render_slash_commands_section(
    theme: &crate::app::theme::Theme,
    slash_commands: &[String],
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    // Copy theme colors for move closures
    let background = theme.colors.background;
    let border = theme.colors.border;
    let info = theme.colors.info;

    div()
        .mb_3()
        .child(
            div()
                .px_2()
                .py_1()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.colors.text_muted)
                .child(format!("SLASH COMMANDS ({})", slash_commands.len())),
        )
        .child(div().flex().flex_wrap().gap_1().px_2().children(
            slash_commands.iter().enumerate().map(|(idx, cmd)| {
                let cmd_clone = cmd.clone();

                div()
                    .id(ElementId::Name(format!("cmd-slash-{}", idx).into()))
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .bg(background)
                    .border_1()
                    .border_color(border)
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(info)
                    .cursor_pointer()
                    .hover(move |s| s.bg(info.opacity(0.1)).border_color(info))
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.use_slash_command(&cmd_clone, cx);
                    }))
                    .child(format!("/{}", cmd))
            }),
        ))
}

fn render_skills_section(
    theme: &crate::app::theme::Theme,
    skills: &[String],
    skill_categories: &[(&str, Vec<&String>)],
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    // Copy theme colors for move closures
    let background = theme.colors.background;
    let border = theme.colors.border;
    let accent = theme.colors.accent;

    div()
        .mb_3()
        .child(
            div()
                .px_2()
                .py_1()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.colors.text_muted)
                .child(format!("SKILLS ({})", skills.len())),
        )
        .children(
            skill_categories
                .iter()
                .filter(|(_, items)| !items.is_empty())
                .map(move |(cat_name, items)| {
                    div()
                        .mb_2()
                        .child(
                            div()
                                .px_2()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("{} ({})", cat_name, items.len())),
                        )
                        .child(div().flex().flex_wrap().gap_1().px_2().children(
                            items.iter().enumerate().map(|(idx, skill)| {
                                let skill_clone = (*skill).clone();
                                let skill_hover = (*skill).clone();
                                let skill_icon = grouping::get_skill_icon(skill);
                                let has_info = get_skill_info(skill).is_some();

                                div()
                                    .id(ElementId::Name(
                                        format!("cmd-skill-{}-{}", cat_name, idx).into(),
                                    ))
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .bg(background)
                                    .border_1()
                                    .border_color(border)
                                    .text_xs()
                                    .text_color(accent)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(accent.opacity(0.1)).border_color(accent))
                                    .on_mouse_move(cx.listener(move |this, _, _window, cx| {
                                        if this.hovered_skill.as_ref() != Some(&skill_hover) {
                                            this.hovered_skill = Some(skill_hover.clone());
                                            cx.notify();
                                        }
                                    }))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.use_skill(&skill_clone, cx);
                                    }))
                                    .child(format!("{} {}", skill_icon, skill))
                                    .when(has_info, |d| {
                                        d.child(
                                            div()
                                                .ml_1()
                                                .text_color(accent.opacity(0.5))
                                                .child("ⓘ"),
                                        )
                                    })
                            }),
                        ))
                }),
        )
}

/// Render skill preview panel
fn render_skill_preview(
    theme: &crate::app::theme::Theme,
    skill: &str,
    _cx: &mut Context<ChatView>,
) -> impl IntoElement {
    let info = get_skill_info(skill);
    let skill_icon = grouping::get_skill_icon(skill);

    let accent = theme.colors.accent;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let text_muted = theme.colors.text_muted;
    let border = theme.colors.border;

    div()
        .border_t_1()
        .border_color(border)
        .bg(surface_hover)
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(28.0))
                        .rounded_md()
                        .bg(accent.opacity(0.15))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .child(skill_icon),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text)
                                .child(format!("/{}", skill)),
                        )
                        .when_some(info.as_ref(), |d, info| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .max_w(px(500.0))
                                    .child(info.description),
                            )
                        }),
                ),
        )
        .when_some(info, |d, info| {
            d.child(
                div()
                    .mt_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(border)
                    .child(
                        div()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(accent)
                            .child(info.usage_example),
                    ),
            )
        })
}

fn render_empty_state(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    use super::super::super::types::ChatViewEvent;

    let accent = theme.colors.accent;
    let info = theme.colors.info;

    div()
        .px_4()
        .py_8()
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
                .child(div().text_xl().child("⚡")),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.colors.text)
                .child("No commands available"),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .text_center()
                .child("Start a session to load available commands and skills"),
        )
        // Quick skill suggestions
        .child(
            div()
                .pt_3()
                .flex()
                .flex_wrap()
                .justify_center()
                .gap_2()
                // Start APEX
                .child(
                    div()
                        .id("commands-empty-apex")
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(accent.opacity(0.15))
                        .border_1()
                        .border_color(accent.opacity(0.3))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(accent)
                        .hover(move |s| {
                            s.bg(accent.opacity(0.25))
                                .border_color(accent.opacity(0.5))
                        })
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.panels.commands_panel = false;
                            cx.emit(ChatViewEvent::Submit("/apex".to_string()));
                            cx.notify();
                        }))
                        .child("⚡ Start with /apex"),
                )
                // Help
                .child(
                    div()
                        .id("commands-empty-help")
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(info.opacity(0.15))
                        .border_1()
                        .border_color(info.opacity(0.3))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(info)
                        .hover(move |s| {
                            s.bg(info.opacity(0.25))
                                .border_color(info.opacity(0.5))
                        })
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.panels.commands_panel = false;
                            cx.emit(ChatViewEvent::Submit("/help".to_string()));
                            cx.notify();
                        }))
                        .child("❓ Get /help"),
                ),
        )
}
