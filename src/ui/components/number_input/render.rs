//! Render implementation for NumberInput

use gpui::prelude::*;
use gpui::*;

use super::component::NumberInput;

impl Render for NumberInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let button_size = self.size.button_size();
        let font_size = self.size.font_size();

        let is_focused = self.focus_handle.is_focused(_window);
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        let border_color = if is_focused {
            theme.colors.accent
        } else {
            theme.colors.border
        };

        let formatted = self.format_value();

        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let surface_color = theme.colors.surface;
        let border_theme_color = theme.colors.border;
        let surface_hover_color = theme.colors.surface_hover;

        let on_decrement = cx.listener(|this, _, _window, cx| {
            this.decrement(cx);
        });

        let on_increment = cx.listener(|this, _, _window, cx| {
            this.increment(cx);
        });

        div()
            .id("number-input")
            .flex()
            .flex_col()
            .gap_1()
            .opacity(opacity)
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(label)
                )
            })
            .child(
                div()
                    .id("number-input-container")
                    .track_focus(&self.focus_handle)
                    .h(px(height))
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(border_color)
                    .bg(surface_color)
                    .flex()
                    .items_center()
                    .when(!self.disabled, |d| {
                        d.hover(|s| s.border_color(accent_color.opacity(0.5)))
                    })
                    .when(self.show_stepper, |d| {
                        let can_dec = self.can_decrement();
                        let dec_text_color = if can_dec { text_color } else { text_muted_color };
                        d.child(
                            div()
                                .id("number-dec")
                                .w(px(button_size))
                                .h_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .border_r_1()
                                .border_color(border_theme_color)
                                .text_size(px(font_size))
                                .text_color(dec_text_color)
                                .when(can_dec, |d| {
                                    d.cursor_pointer()
                                        .hover(|s| s.bg(surface_hover_color))
                                        .on_click(on_decrement)
                                })
                                .child("−")
                        )
                    })
                    .when_some(self.prefix.clone(), |d, prefix| {
                        d.child(
                            div()
                                .pl_2()
                                .text_size(px(font_size))
                                .text_color(text_muted_color)
                                .child(prefix)
                        )
                    })
                    .child(
                        div()
                            .flex_1()
                            .px_2()
                            .text_size(px(font_size))
                            .text_color(text_color)
                            .text_center()
                            .child(formatted)
                    )
                    .when_some(self.suffix.clone(), |d, suffix| {
                        d.child(
                            div()
                                .pr_2()
                                .text_size(px(font_size))
                                .text_color(text_muted_color)
                                .child(suffix)
                        )
                    })
                    .when(self.show_stepper, |d| {
                        let can_inc = self.can_increment();
                        let inc_text_color = if can_inc { text_color } else { text_muted_color };
                        d.child(
                            div()
                                .id("number-inc")
                                .w(px(button_size))
                                .h_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .border_l_1()
                                .border_color(border_theme_color)
                                .text_size(px(font_size))
                                .text_color(inc_text_color)
                                .when(can_inc, |d| {
                                    d.cursor_pointer()
                                        .hover(|s| s.bg(surface_hover_color))
                                        .on_click(on_increment)
                                })
                                .child("+")
                        )
                    })
            )
    }
}
