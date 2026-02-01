//! Main text input area rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

use super::ChatInput;

impl ChatInput {
    /// Render the main text input area
    pub(super) fn render_input_area(
        &self,
        theme: &Theme,
        is_focused: bool,
        is_disabled: bool,
        text_is_empty: bool,
        has_mentions: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("chat-input")
            .track_focus(&self.focus_handle)
            .relative()
            .flex_1()
            .min_h(px(40.0))
            .max_h(px(200.0))
            .px_4()
            .py_2()
            .rounded_lg()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(if is_disabled {
                theme.colors.border
            } else if is_focused {
                theme.colors.accent
            } else {
                theme.colors.border
            })
            .when(!is_disabled, |d| d.cursor_text())
            .when(is_disabled, |d| d.cursor_not_allowed())
            .when(!is_disabled, |d| {
                d.on_click(cx.listener(|this, _, window, _cx| {
                    this.focus_handle.focus(window);
                }))
            })
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_key_down(event, window, cx);
            }))
            .child(div().flex_1().child(if is_disabled {
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child("Claude is responding...")
            } else if text_is_empty {
                // Show placeholder with cursor when focused
                if is_focused {
                    self.render_placeholder_with_cursor(theme)
                } else {
                    self.render_placeholder(theme)
                }
            } else if has_mentions {
                self.render_text_with_cursor_and_mentions(theme, is_focused)
            } else {
                self.render_text_with_cursor(theme, is_focused)
            }))
            // Clear button (when text is not empty)
            .when(!text_is_empty && !is_disabled, |d| {
                d.child(self.render_clear_button(theme, cx))
            })
            // History navigation indicator
            .when_some(self.history_index, |d, idx| {
                d.child(self.render_history_indicator(idx, theme))
            })
    }

    /// Render placeholder text
    fn render_placeholder(&self, theme: &Theme) -> Div {
        let has_commands = !self.available_commands.is_empty();
        let has_history = !self.input_history.is_empty();

        div()
            .text_sm()
            .text_color(theme.colors.text_muted)
            .child(if has_commands && has_history {
                "Ask anything... (/ commands, @file, ↑ history)"
            } else if has_commands {
                "Ask anything... (/ commands, @file to attach)"
            } else if has_history {
                "Ask anything... (@file, ↑ history)"
            } else {
                "Ask anything... (@file:path to attach files)"
            })
    }

    /// Render placeholder with blinking cursor when focused
    fn render_placeholder_with_cursor(&self, theme: &Theme) -> Div {
        let has_commands = !self.available_commands.is_empty();
        let has_history = !self.input_history.is_empty();

        let placeholder = if has_commands && has_history {
            "Ask anything... (/ commands, @file, ↑ history)"
        } else if has_commands {
            "Ask anything... (/ commands, @file to attach)"
        } else if has_history {
            "Ask anything... (@file, ↑ history)"
        } else {
            "Ask anything... (@file:path to attach files)"
        };

        div()
            .flex()
            .items_center()
            // Blinking cursor
            .child(
                div()
                    .w(px(2.0))
                    .h(px(16.0))
                    .bg(theme.colors.accent)
                    .rounded_sm()
                    .with_animation(
                        "cursor-blink",
                        Animation::new(std::time::Duration::from_millis(530))
                            .repeat()
                            .with_easing(pulsating_between(0.0, 1.0)),
                        move |this, delta| this.opacity(delta),
                    ),
            )
            // Placeholder text
            .child(
                div()
                    .ml_1()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child(placeholder),
            )
    }

    /// Render text with cursor (without mentions)
    fn render_text_with_cursor(&self, theme: &Theme, is_focused: bool) -> Div {
        let text = &self.text;
        let cursor_pos = self.cursor_position.min(text.len());

        // Split text at cursor position
        let (before_cursor, after_cursor) = text.split_at(cursor_pos);

        div()
            .flex()
            .items_center()
            .text_sm()
            .text_color(theme.colors.text)
            // Text before cursor
            .child(div().child(before_cursor.to_string()))
            // Blinking cursor (only when focused)
            .when(is_focused, |d| {
                d.child(
                    div()
                        .w(px(2.0))
                        .h(px(16.0))
                        .bg(theme.colors.accent)
                        .rounded_sm()
                        .with_animation(
                            "cursor-blink",
                            Animation::new(std::time::Duration::from_millis(530))
                                .repeat()
                                .with_easing(pulsating_between(0.0, 1.0)),
                            move |this, delta| this.opacity(delta),
                        ),
                )
            })
            // Text after cursor
            .child(div().child(after_cursor.to_string()))
    }

    /// Render text with cursor and mentions (with syntax highlighting for @mentions)
    fn render_text_with_cursor_and_mentions(&self, theme: &Theme, is_focused: bool) -> Div {
        use crate::ai::mention::MentionKind;

        if self.mentions.is_empty() {
            return self.render_text_with_cursor(theme, is_focused);
        }

        let text = &self.text;
        let cursor_pos = self.cursor_position.min(text.len());

        // Sort mentions by position
        let mut mentions = self.mentions.clone();
        mentions.sort_by_key(|m| m.start);

        let mut elements: Vec<AnyElement> = Vec::new();
        let mut last_end = 0;
        let mut cursor_rendered = false;

        for mention in &mentions {
            // Add text before mention (with cursor if applicable)
            if mention.start > last_end {
                let segment = &text[last_end..mention.start];

                // Check if cursor is in this segment
                if !cursor_rendered && cursor_pos >= last_end && cursor_pos <= mention.start {
                    let offset = cursor_pos - last_end;
                    let (before, after) = segment.split_at(offset);

                    // Text before cursor
                    if !before.is_empty() {
                        elements.push(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text)
                                .child(before.to_string())
                                .into_any_element(),
                        );
                    }

                    // Blinking cursor
                    if is_focused {
                        elements.push(
                            div()
                                .w(px(2.0))
                                .h(px(16.0))
                                .bg(theme.colors.accent)
                                .rounded_sm()
                                .with_animation(
                                    "cursor-blink",
                                    Animation::new(std::time::Duration::from_millis(530))
                                        .repeat()
                                        .with_easing(pulsating_between(0.0, 1.0)),
                                    move |this, delta| this.opacity(delta),
                                )
                                .into_any_element(),
                        );
                    }

                    // Text after cursor
                    if !after.is_empty() {
                        elements.push(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text)
                                .child(after.to_string())
                                .into_any_element(),
                        );
                    }

                    cursor_rendered = true;
                } else {
                    elements.push(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(segment.to_string())
                            .into_any_element(),
                    );
                }
            }

            // Add highlighted mention (check if cursor is inside)
            let mention_color = match &mention.kind {
                MentionKind::File(_) | MentionKind::FileRange { .. } => theme.colors.accent,
                MentionKind::Snippet(_) => theme.colors.success,
                MentionKind::Url(_) => theme.colors.info,
                MentionKind::Symbol(_) => theme.colors.warning,
            };

            // Check if cursor is inside the mention
            if !cursor_rendered && cursor_pos >= mention.start && cursor_pos <= mention.end {
                let offset = cursor_pos - mention.start;
                let (before, after) = mention.raw.split_at(offset);

                elements.push(
                    div()
                        .flex()
                        .items_center()
                        .bg(mention_color.opacity(0.15))
                        .px_1()
                        .rounded_sm()
                        .child(
                            div()
                                .text_sm()
                                .text_color(mention_color)
                                .font_weight(FontWeight::MEDIUM)
                                .child(before.to_string()),
                        )
                        .when(is_focused, |d| {
                            d.child(
                                div()
                                    .w(px(2.0))
                                    .h(px(16.0))
                                    .bg(theme.colors.accent)
                                    .rounded_sm()
                                    .with_animation(
                                        "cursor-blink",
                                        Animation::new(std::time::Duration::from_millis(530))
                                            .repeat()
                                            .with_easing(pulsating_between(0.0, 1.0)),
                                        move |this, delta| this.opacity(delta),
                                    ),
                            )
                        })
                        .child(
                            div()
                                .text_sm()
                                .text_color(mention_color)
                                .font_weight(FontWeight::MEDIUM)
                                .child(after.to_string()),
                        )
                        .into_any_element(),
                );
                cursor_rendered = true;
            } else {
                elements.push(
                    div()
                        .text_sm()
                        .text_color(mention_color)
                        .font_weight(FontWeight::MEDIUM)
                        .bg(mention_color.opacity(0.15))
                        .px_1()
                        .rounded_sm()
                        .child(mention.raw.clone())
                        .into_any_element(),
                );
            }

            last_end = mention.end;
        }

        // Add text after last mention (with cursor if applicable)
        if last_end < text.len() {
            let segment = &text[last_end..];

            if !cursor_rendered && cursor_pos >= last_end {
                let offset = cursor_pos - last_end;
                let offset = offset.min(segment.len());
                let (before, after) = segment.split_at(offset);

                if !before.is_empty() {
                    elements.push(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(before.to_string())
                            .into_any_element(),
                    );
                }

                if is_focused {
                    elements.push(
                        div()
                            .w(px(2.0))
                            .h(px(16.0))
                            .bg(theme.colors.accent)
                            .rounded_sm()
                            .with_animation(
                                "cursor-blink",
                                Animation::new(std::time::Duration::from_millis(530))
                                    .repeat()
                                    .with_easing(pulsating_between(0.0, 1.0)),
                                move |this, delta| this.opacity(delta),
                            )
                            .into_any_element(),
                    );
                }

                if !after.is_empty() {
                    elements.push(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(after.to_string())
                            .into_any_element(),
                    );
                }
            } else {
                elements.push(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text)
                        .child(segment.to_string())
                        .into_any_element(),
                );
            }
        } else if !cursor_rendered && is_focused && cursor_pos == text.len() {
            // Cursor at the very end
            elements.push(
                div()
                    .w(px(2.0))
                    .h(px(16.0))
                    .bg(theme.colors.accent)
                    .rounded_sm()
                    .with_animation(
                        "cursor-blink",
                        Animation::new(std::time::Duration::from_millis(530))
                            .repeat()
                            .with_easing(pulsating_between(0.0, 1.0)),
                        move |this, delta| this.opacity(delta),
                    )
                    .into_any_element(),
            );
        }

        div().flex().items_center().flex_wrap().children(elements)
    }

    /// Render clear button
    fn render_clear_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("clear-input-button")
            .absolute()
            .right(px(8.0))
            .top(px(8.0))
            .size(px(20.0))
            .rounded_full()
            .bg(theme.colors.text_muted.opacity(0.2))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.error.opacity(0.3)))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.clear(cx);
            }))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("✕"),
            )
    }

    /// Render history navigation indicator
    fn render_history_indicator(&self, idx: usize, theme: &Theme) -> impl IntoElement {
        div()
            .absolute()
            .left(px(8.0))
            .top(px(8.0))
            .px_2()
            .py_px()
            .rounded_md()
            .bg(theme.colors.info.opacity(0.2))
            .flex()
            .items_center()
            .gap_1()
            .child(div().text_xs().text_color(theme.colors.info).child("↑"))
            .child(div().text_xs().text_color(theme.colors.info).child(format!(
                "{}/{}",
                idx + 1,
                self.input_history.len()
            )))
    }
}
