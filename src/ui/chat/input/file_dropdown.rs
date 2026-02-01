//! File mention autocomplete dropdown rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

use super::ChatInput;

impl ChatInput {
    /// Render file autocomplete dropdown with fuzzy match highlighting
    pub(super) fn render_file_dropdown(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected_idx = self.selected_file_index;
        let accent = theme.colors.accent;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .id("file-autocomplete")
            .absolute()
            .bottom(px(80.0))
            .left(px(16.0))
            .right(px(16.0))
            .max_h(px(280.0))
            .overflow_y_scroll()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_lg()
            .shadow_lg()
            .p_1()
            .flex()
            .flex_col()
            .gap_px()
            // Header
            .child(
                div()
                    .px_3()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_muted)
                            .child("Files"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.6))
                            .child(format!("{} results", self.filtered_files.len())),
                    ),
            )
            // File list
            .children(
                self.filtered_files
                    .iter()
                    .enumerate()
                    .map(|(i, file_match)| {
                        let is_selected = i == selected_idx;
                        let path_clone = file_match.path.clone();
                        let icon = file_match.icon;
                        let display = file_match.display.clone();
                        let matched_indices = file_match.matched_indices.clone();
                        let full_path = file_match.path.clone();

                        div()
                            .id(SharedString::from(format!("file-{}", i)))
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
                                // Find and select this file
                                if let Some(idx) = this
                                    .filtered_files
                                    .iter()
                                    .position(|f| f.path == path_clone)
                                {
                                    this.selected_file_index = idx;
                                    this.insert_selected_file(cx);
                                }
                            }))
                            .flex()
                            .items_center()
                            .gap_3()
                            // File icon
                            .child(div().text_sm().child(icon))
                            // File info
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_px()
                                    // Filename with highlighted matches
                                    .child(div().flex().items_center().text_sm().children(
                                        render_highlighted_filename(
                                            &display,
                                            &matched_indices,
                                            accent,
                                            text_color,
                                        ),
                                    ))
                                    // Full path (muted)
                                    .when(full_path != display, |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(text_muted)
                                                .truncate()
                                                .child(full_path),
                                        )
                                    }),
                            )
                    }),
            )
            // Footer with hints
            .child(
                div()
                    .mt_1()
                    .pt_1()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .px_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("↑↓ navigate"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Tab/Enter select"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Esc close"),
                    ),
            )
    }
}

/// Render filename with matched characters highlighted
fn render_highlighted_filename(
    name: &str,
    matched_indices: &[usize],
    accent: gpui::Hsla,
    text_color: gpui::Hsla,
) -> Vec<Div> {
    let chars: Vec<char> = name.chars().collect();
    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut current_is_matched = false;

    for (i, c) in chars.iter().enumerate() {
        let is_matched = matched_indices.contains(&i);

        if i == 0 {
            current_is_matched = is_matched;
            current_text.push(*c);
        } else if is_matched == current_is_matched {
            current_text.push(*c);
        } else {
            // State changed, push current segment
            if !current_text.is_empty() {
                result.push(
                    div()
                        .text_color(if current_is_matched {
                            accent
                        } else {
                            text_color
                        })
                        .when(current_is_matched, |d| d.font_weight(FontWeight::BOLD))
                        .child(current_text.clone()),
                );
            }
            current_text = String::new();
            current_text.push(*c);
            current_is_matched = is_matched;
        }
    }

    // Push final segment
    if !current_text.is_empty() {
        result.push(
            div()
                .text_color(if current_is_matched {
                    accent
                } else {
                    text_color
                })
                .when(current_is_matched, |d| d.font_weight(FontWeight::BOLD))
                .child(current_text),
        );
    }

    result
}
