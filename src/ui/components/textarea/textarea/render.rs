//! Textarea rendering

use super::state::Textarea;
use crate::ui::components::textarea::types::*;
use gpui::prelude::*;
use gpui::*;

impl EventEmitter<TextareaEvent> for Textarea {}

impl Focusable for Textarea {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Textarea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let line_height = 20.0_f32;
        let rows = self.visible_rows();
        let height = (rows as f32 * line_height) + 16.0; // padding

        let has_error = self.error.is_some();
        let is_focused = self.focus_handle.is_focused(_window);

        let border_color = if has_error {
            theme.colors.error
        } else if is_focused {
            theme.colors.accent
        } else {
            theme.colors.border
        };

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        let char_count = self.text.chars().count();
        let count_text = if let Some(max) = self.max_length {
            format!("{}/{}", char_count, max)
        } else {
            format!("{}", char_count)
        };

        // Copy colors for move closures
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let surface_color = theme.colors.surface;
        let accent_color = theme.colors.accent;
        let error_color = theme.colors.error;

        // Extract listener before div chain
        let key_down_handler = cx.listener(|this, event: &KeyDownEvent, window, cx| {
            this.handle_key_down(event, window, cx);
        });

        div()
            .id("textarea")
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
            .opacity(opacity)
            // Label
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .mb_1()
                        .child(label),
                )
            })
            // Textarea container
            .child(
                div()
                    .id("textarea-input")
                    .track_focus(&self.focus_handle)
                    .w_full()
                    .h(px(height))
                    .px_3()
                    .py_2()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(border_color)
                    .bg(surface_color)
                    .when(!self.disabled && !self.readonly, |d| {
                        d.hover(|s| s.border_color(accent_color.opacity(0.5)))
                    })
                    .id("scroll-textarea")
                    .overflow_y_scroll()
                    .on_key_down(key_down_handler)
                    // Content or placeholder
                    .child(
                        div()
                            .w_full()
                            .text_sm()
                            .when(self.text.is_empty(), |d| {
                                d.text_color(text_muted).child(self.placeholder.clone())
                            })
                            .when(!self.text.is_empty(), |d| {
                                d.text_color(text_color).child(self.text.clone())
                            }),
                    ),
            )
            // Footer row (helper/error + count)
            .when(
                self.helper.is_some() || self.error.is_some() || self.show_count,
                |d| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_xs()
                            // Helper or error
                            .child(
                                div()
                                    .when_some(self.error.clone(), |d, err| {
                                        d.text_color(error_color).child(err)
                                    })
                                    .when(self.error.is_none(), |d| {
                                        d.when_some(self.helper.clone(), |d, help| {
                                            d.text_color(text_muted).child(help)
                                        })
                                    }),
                            )
                            // Character count
                            .when(self.show_count, |d| {
                                d.child(div().text_color(text_muted).child(count_text))
                            }),
                    )
                },
            )
    }
}
