//! Dropdown render implementation

use gpui::prelude::*;
use gpui::*;

use super::state::Dropdown;

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();

        let selected_label = self
            .selected_option()
            .map(|o| o.label.clone())
            .unwrap_or_else(|| self.placeholder.clone());

        let has_error = self.error.is_some();
        let border_color = if has_error {
            theme.colors.error
        } else if self.is_open {
            theme.colors.accent
        } else {
            theme.colors.border
        };

        // Copy theme colors for move closures
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let surface_hover = theme.colors.surface_hover;
        let accent_color = theme.colors.accent;
        let surface_color = theme.colors.surface;
        let error_color = theme.colors.error;

        // Extract listener before div chain
        let toggle_listener = cx.listener(|this, _, _window, cx| {
            this.toggle(cx);
        });

        div()
            .id("dropdown")
            .w_full()
            .flex()
            .flex_col()
            .gap_1()
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
            // Dropdown trigger
            .child(
                div()
                    .relative()
                    // Trigger button
                    .child(
                        div()
                            .id("dropdown-trigger")
                            .h(px(height))
                            .w_full()
                            .px(px(padding_x))
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(border_color)
                            .bg(surface_color)
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .when(!self.disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.border_color(accent_color.opacity(0.5)))
                            })
                            .when(self.disabled, |d| d.opacity(0.5))
                            .on_click(toggle_listener)
                            // Selected value or placeholder
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(font_size))
                                    .text_color(if self.selected.is_some() {
                                        text_color
                                    } else {
                                        text_muted
                                    })
                                    .truncate()
                                    .child(selected_label),
                            )
                            // Chevron icon
                            .child(
                                div()
                                    .text_color(text_muted)
                                    .text_size(px(10.0))
                                    .child(if self.is_open { "▲" } else { "▼" }),
                            ),
                    )
                    // Dropdown menu
                    .when(self.is_open, |d| {
                        let filtered = self.filtered_options();

                        d.child(
                            div()
                                .absolute()
                                .top(px(height + 4.0))
                                .left_0()
                                .right_0()
                                .max_h(px(200.0))
                                .id("scroll-dropdown-menu")
                                .overflow_y_scroll()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(theme.colors.border)
                                .bg(surface_color)
                                .shadow_lg()
                                // Options
                                .children(filtered.iter().enumerate().map(|(i, option)| {
                                    let is_selected = self.selected.as_ref() == Some(&option.id);
                                    let option_id = option.id.clone();

                                    // Extract listener before div chain
                                    let select_listener =
                                        cx.listener(move |this, _, _window, cx| {
                                            this.select(option_id.clone(), cx);
                                        });

                                    div()
                                        .id(SharedString::from(format!("option-{}", i)))
                                        .px(px(padding_x))
                                        .py_2()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .when(is_selected, |d| d.bg(accent_color.opacity(0.15)))
                                        .when(!option.disabled, |d| {
                                            d.cursor_pointer().hover(|s| s.bg(surface_hover))
                                        })
                                        .when(option.disabled, |d| d.opacity(0.5))
                                        .when(!option.disabled, |d| d.on_click(select_listener))
                                        // Icon
                                        .when_some(option.icon.clone(), |d, icon| {
                                            d.child(div().text_sm().child(icon))
                                        })
                                        // Label and description
                                        .child(
                                            div()
                                                .flex_1()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(font_size))
                                                        .text_color(text_color)
                                                        .child(option.label.clone()),
                                                )
                                                .when_some(
                                                    option.description.clone(),
                                                    |d, desc| {
                                                        d.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(text_muted)
                                                                .child(desc),
                                                        )
                                                    },
                                                ),
                                        )
                                        // Checkmark for selected
                                        .when(is_selected, |d| {
                                            d.child(div().text_color(accent_color).child("✓"))
                                        })
                                }))
                                // Empty state
                                .when(filtered.is_empty(), |d| {
                                    d.child(
                                        div()
                                            .px(px(padding_x))
                                            .py_3()
                                            .text_sm()
                                            .text_color(text_muted)
                                            .text_center()
                                            .child("No options"),
                                    )
                                }),
                        )
                    }),
            )
            // Error message
            .when_some(self.error.clone(), |d, error| {
                d.child(div().text_xs().text_color(error_color).child(error))
            })
    }
}
