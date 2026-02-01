//! Command palette rendering

use gpui::*;
use gpui::prelude::*;

use super::state::CommandPalette;
use super::types::CommandPaletteEvent;

/// Represents a segment of text, either matched or unmatched
struct LabelSegment {
    text: String,
    is_matched: bool,
}

/// Split a label into segments based on matched character indices
fn split_label_by_matches(label: &str, matched_indices: &[usize]) -> Vec<LabelSegment> {
    if matched_indices.is_empty() {
        return vec![LabelSegment {
            text: label.to_string(),
            is_matched: false,
        }];
    }

    let chars: Vec<char> = label.chars().collect();
    let mut segments = Vec::new();
    let mut current_segment = String::new();
    let mut current_is_matched = false;

    for (i, c) in chars.iter().enumerate() {
        let is_matched = matched_indices.contains(&i);

        if i == 0 {
            current_is_matched = is_matched;
            current_segment.push(*c);
        } else if is_matched == current_is_matched {
            current_segment.push(*c);
        } else {
            // State changed, push current segment and start new one
            if !current_segment.is_empty() {
                segments.push(LabelSegment {
                    text: current_segment,
                    is_matched: current_is_matched,
                });
            }
            current_segment = String::new();
            current_segment.push(*c);
            current_is_matched = is_matched;
        }
    }

    // Push final segment
    if !current_segment.is_empty() {
        segments.push(LabelSegment {
            text: current_segment,
            is_matched: current_is_matched,
        });
    }

    segments
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let matches = self.fuzzy_matches();
        let selected_index = self.selected_index;
        let query = self.query.clone();
        let recent_commands = self.recent_commands.clone();

        // Pre-compute command items for rendering with fuzzy match data
        let command_items: Vec<_> = matches
            .iter()
            .enumerate()
            .map(|(idx, m)| {
                let is_recent = recent_commands.contains(&m.command.id.to_string());
                (
                    idx,
                    m.command.id,
                    m.command.label.to_string(),
                    m.command.shortcut.map(|s| s.to_string()),
                    m.command.category.to_string(),
                    idx == selected_index,
                    m.matched_indices.clone(),
                    m.score,
                    is_recent,
                )
            })
            .collect();

        // Overlay background
        div()
            .id("command-palette-overlay")
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_start()
            .justify_center()
            .pt(px(100.0))
            .on_click(cx.listener(|_this, _, _window, cx| {
                cx.emit(CommandPaletteEvent::Dismissed);
            }))
            .child(
                // Palette modal
                div()
                    .id("command-palette")
                    .track_focus(&self.focus_handle)
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .on_click(|_, _, cx| {
                        // Stop propagation to prevent closing
                    })
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                        this.handle_key_down(event, window, cx);
                    }))
                    // Search input
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child(">"),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .text_color(if query.is_empty() {
                                                theme.colors.text_muted
                                            } else {
                                                theme.colors.text
                                            })
                                            .child(if query.is_empty() {
                                                "Type a command...".to_string()
                                            } else {
                                                query
                                            }),
                                    ),
                            ),
                    )
                    // Command list
                    .child(
                        div()
                            .max_h(px(320.0))
                            .id("scroll-commands")
                            .overflow_y_scroll()
                            .children(command_items.into_iter().map(
                                |(idx, _id, label, shortcut, category, is_selected, matched_indices, _score, is_recent)| {
                                    let bg_color = if is_selected {
                                        theme.colors.accent.opacity(0.2)
                                    } else {
                                        theme.colors.surface
                                    };

                                    // Split label into matched/unmatched segments for highlighting
                                    let segments = split_label_by_matches(&label, &matched_indices);
                                    let accent_color = theme.colors.accent;
                                    let text_color = theme.colors.text;
                                    let info_color = theme.colors.info;

                                    div()
                                        .id(ElementId::Name(format!("cmd-{}", idx).into()))
                                        .px_4()
                                        .py_2()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .bg(bg_color)
                                        .hover(|style| style.bg(theme.colors.surface_hover))
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.select_command(idx, cx);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_3()
                                                // Recent indicator
                                                .when(is_recent, |d| {
                                                    d.child(
                                                        div()
                                                            .px_1p5()
                                                            .py_0p5()
                                                            .rounded_sm()
                                                            .bg(info_color.opacity(0.15))
                                                            .text_xs()
                                                            .text_color(info_color)
                                                            .child("Recent")
                                                    )
                                                })
                                                .child(
                                                    div()
                                                        .px_2()
                                                        .py_0p5()
                                                        .rounded_sm()
                                                        .bg(theme.colors.accent.opacity(0.1))
                                                        .text_xs()
                                                        .text_color(theme.colors.accent)
                                                        .child(category),
                                                )
                                                // Label with match highlighting
                                                .child(
                                                    div()
                                                        .flex()
                                                        .text_sm()
                                                        .children(segments.into_iter().map(move |seg| {
                                                            div()
                                                                .text_color(if seg.is_matched {
                                                                    accent_color
                                                                } else {
                                                                    text_color
                                                                })
                                                                .when(seg.is_matched, |d| d.font_weight(FontWeight::BOLD))
                                                                .child(seg.text)
                                                        })),
                                                ),
                                        )
                                        .when_some(shortcut, |this, shortcut| {
                                            this.child(
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_sm()
                                                    .bg(theme.colors.background)
                                                    .text_xs()
                                                    .font_family("JetBrains Mono")
                                                    .text_color(theme.colors.text_muted)
                                                    .child(shortcut),
                                            )
                                        })
                                },
                            )),
                    )
                    // Footer hint
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
                                    .child("↑↓ Navigate"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Enter Select"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Esc Close"),
                            ),
                    ),
            )
    }
}
