//! Command palette render functions for ChatView

use super::super::core::ChatView;
use super::super::types::{ChatViewEvent, PaletteCommand};
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Get quick skill chips for the palette header
    fn get_quick_skills() -> Vec<(&'static str, &'static str, &'static str)> {
        // (icon, label, command) - Most used Claude Code skills
        vec![
            ("⚡", "APEX", "/apex"),
            ("🔍", "Explore", "/explore"),
            ("🐛", "Debug", "/debug"),
            ("📦", "Commit", "/commit"),
            ("👀", "Review", "/review"),
            ("🚀", "Oneshot", "/oneshot"),
            ("🧠", "Ultrathink", "/ultrathink"),
            ("💡", "Brainstorm", "/brainstorm"),
        ]
    }

    /// Get priority order for categories (skills first)
    fn category_priority(category: &str) -> usize {
        match category {
            "Skills" => 0,
            "Git" => 1,
            "Claude Code" => 2,
            "Commands" => 3,
            "Actions" => 4,
            "Navigation" => 5,
            "View" => 6,
            _ => 10,
        }
    }

    /// Get category icon and style hint
    fn category_style(category: &str) -> (&'static str, &'static str) {
        // (icon, style_hint for color selection)
        match category {
            "Skills" => ("⚡", "accent"),
            "Git" => ("🔀", "success"),
            "Claude Code" => ("🤖", "info"),
            "Commands" => ("/", "warning"),
            "Actions" => ("▶", "text"),
            "Navigation" => ("🧭", "text"),
            "View" => ("👁", "text"),
            "Messages" => ("💬", "text"),
            "Session" => ("📊", "info"),
            "Input" => ("⌨", "text"),
            "Files" => ("📁", "warning"),
            "Bookmarks" => ("⭐", "warning"),
            "Panels" => ("📋", "text"),
            "Workflow" => ("🎯", "accent"),
            "Help" => ("❓", "info"),
            _ => ("•", "text"),
        }
    }

    pub fn render_command_palette(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let commands = Self::get_palette_commands();
        let query = self.palette_query.to_lowercase();

        // Filter commands based on query
        let filtered_commands: Vec<&PaletteCommand> = if query.is_empty() {
            commands.iter().collect()
        } else {
            commands
                .iter()
                .filter(|cmd| {
                    cmd.label.to_lowercase().contains(&query)
                        || cmd.description.to_lowercase().contains(&query)
                        || cmd.category.to_lowercase().contains(&query)
                })
                .collect()
        };

        // Group by category
        let mut categories: Vec<(&str, Vec<&PaletteCommand>)> = vec![];
        for cmd in &filtered_commands {
            if let Some(cat) = categories.iter_mut().find(|(c, _)| *c == cmd.category) {
                cat.1.push(cmd);
            } else {
                categories.push((cmd.category, vec![cmd]));
            }
        }

        // Sort categories by priority
        categories.sort_by_key(|(cat, _)| Self::category_priority(cat));

        let selected_idx = self.palette_selected_index;
        let palette_query = self.palette_query.clone();
        let quick_skills = Self::get_quick_skills();

        div()
            .id("command-palette-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_start()
            .justify_center()
            .pt(px(80.0))
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_command_palette(cx);
            }))
            .child(
                div()
                    .id("command-palette-panel")
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .rounded_xl()
                    .shadow_xl()
                    .overflow_hidden()
                    .on_click(|_, _window, cx| {
                        cx.stop_propagation();
                    })
                    // Search input
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_lg()
                                    .text_color(theme.colors.text_muted)
                                    .child("⌘"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_base()
                                    .text_color(theme.colors.text)
                                    .child(if palette_query.is_empty() {
                                        div()
                                            .text_color(theme.colors.text_muted)
                                            .child("Type a command...")
                                    } else {
                                        div().child(palette_query)
                                    }),
                            ),
                    )
                    // Quick skills chips (when no query)
                    .when(query.is_empty(), |d| {
                        d.child(
                            div()
                                .px_4()
                                .py_2()
                                .border_b_1()
                                .border_color(theme.colors.border)
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Quick:"),
                                )
                                .children(quick_skills.iter().map(|(icon, label, cmd)| {
                                    let cmd_str = cmd.to_string();
                                    div()
                                        .id(SharedString::from(format!("quick-skill-{}", label)))
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .bg(theme.colors.accent.opacity(0.1))
                                        .border_1()
                                        .border_color(theme.colors.accent.opacity(0.2))
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(theme.colors.accent.opacity(0.2))
                                                .border_color(theme.colors.accent.opacity(0.4))
                                        })
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.toggle_command_palette(cx);
                                            cx.emit(ChatViewEvent::Submit(cmd_str.clone()));
                                        }))
                                        .child(div().text_xs().child(*icon))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.colors.accent)
                                                .child(*label),
                                        )
                                })),
                        )
                    })
                    // Commands list
                    .child(
                        div()
                            .id("command-palette-list")
                            .overflow_y_scroll()
                            .max_h(px(320.0))
                            .p_2()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .children(categories.iter().flat_map(|(category, cmds)| {
                                let mut elements: Vec<gpui::AnyElement> = vec![];
                                let (cat_icon, cat_style) = Self::category_style(category);
                                let cat_color = match cat_style {
                                    "accent" => theme.colors.accent,
                                    "success" => theme.colors.success,
                                    "info" => theme.colors.info,
                                    "warning" => theme.colors.warning,
                                    _ => theme.colors.text_muted,
                                };

                                // Category header with icon and color
                                elements.push(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .mt_1()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().text_xs().child(cat_icon))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(cat_color)
                                                .child(*category),
                                        )
                                        .into_any_element(),
                                );

                                // Commands in category
                                for (i, cmd) in cmds.iter().enumerate() {
                                    let cmd_id = cmd.id;
                                    let global_idx = filtered_commands
                                        .iter()
                                        .position(|c| c.id == cmd.id)
                                        .unwrap_or(0);
                                    let is_selected = global_idx == selected_idx;

                                    elements.push(
                                        div()
                                            .id(SharedString::from(format!("cmd-{}", i)))
                                            .px_3()
                                            .py_2()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(if is_selected {
                                                theme.colors.accent.opacity(0.15)
                                            } else {
                                                gpui::transparent_black()
                                            })
                                            .hover(|s| s.bg(theme.colors.surface_hover))
                                            .on_click(cx.listener(move |this, _, _window, cx| {
                                                this.execute_palette_command(cmd_id, cx);
                                            }))
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            // Icon
                                            .child(div().w(px(20.0)).text_center().child(cmd.icon))
                                            // Label and description
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.colors.text)
                                                            .child(cmd.label),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.colors.text_muted)
                                                            .child(cmd.description),
                                                    ),
                                            )
                                            // Shortcut
                                            .when_some(cmd.shortcut, |d, shortcut| {
                                                d.child(
                                                    div()
                                                        .px_2()
                                                        .py(px(2.0))
                                                        .rounded_sm()
                                                        .bg(theme.colors.background)
                                                        .border_1()
                                                        .border_color(theme.colors.border)
                                                        .text_xs()
                                                        .font_family("monospace")
                                                        .text_color(theme.colors.text_muted)
                                                        .child(shortcut),
                                                )
                                            })
                                            .into_any_element(),
                                    );
                                }

                                elements
                            })),
                    )
                    // Footer
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("{} commands", filtered_commands.len())),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("↑↓ navigate")
                                    .child("·")
                                    .child("Enter select")
                                    .child("·")
                                    .child("Esc close"),
                            ),
                    ),
            )
    }
}
