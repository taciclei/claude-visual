//! Commands panel header rendering

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;
use super::super::super::types::CommandCategory;

pub fn render_header(
    theme: &crate::app::theme::Theme,
    filter: &str,
    category: CommandCategory,
    total_count: usize,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    // Copy theme colors for move closures
    let surface_hover = theme.colors.surface_hover;
    let accent = theme.colors.accent;
    let text_muted = theme.colors.text_muted;

    // Extract listeners before div chain
    let on_close_click = cx.listener(|this, _, _window, cx| {
        this.toggle_commands_panel(cx);
    });

    div()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(theme.colors.border)
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
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().text_base().child("⚡"))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.colors.text)
                                .child("Commands & Skills"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_px()
                                .rounded_full()
                                .bg(theme.colors.accent.opacity(0.2))
                                .text_xs()
                                .text_color(theme.colors.accent)
                                .child(format!("{} total", total_count)),
                        ),
                )
                .child(
                    div()
                        .id("close-commands-panel")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .hover(move |s| s.bg(surface_hover))
                        .on_click(on_close_click)
                        .child("×"),
                ),
        )
        .child(render_category_tabs(
            category,
            accent,
            text_muted,
            surface_hover,
            cx,
        ))
        .child(render_search_hint(theme, filter))
}

fn render_category_tabs(
    current_category: CommandCategory,
    accent: Hsla,
    text_muted: Hsla,
    surface_hover: Hsla,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    div().flex().items_center().gap_1().children(
        [
            CommandCategory::All,
            CommandCategory::SlashCommands,
            CommandCategory::Skills,
        ]
        .into_iter()
        .map(|cat| {
            let is_active = current_category == cat;

            div()
                .id(ElementId::Name(format!("cmd-cat-{:?}", cat).into()))
                .px_3()
                .py_1()
                .rounded_md()
                .text_xs()
                .cursor_pointer()
                .bg(if is_active {
                    accent.opacity(0.2)
                } else {
                    gpui::transparent_black()
                })
                .text_color(if is_active { accent } else { text_muted })
                .hover(move |s| s.bg(surface_hover))
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.set_commands_category(cat, cx);
                }))
                .child(format!("{} {}", cat.icon(), cat.label()))
        }),
    )
}

fn render_search_hint(theme: &crate::app::theme::Theme, filter: &str) -> impl IntoElement {
    div()
        .px_3()
        .py_2()
        .bg(theme.colors.background)
        .rounded_md()
        .text_xs()
        .text_color(theme.colors.text_muted)
        .child(if filter.is_empty() {
            "Type / in chat to search commands...".to_string()
        } else {
            format!("Filtering: {}", filter)
        })
}
