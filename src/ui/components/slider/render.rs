//! Slider rendering implementation

use super::slider::Slider;
use super::types::SliderEvent;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl EventEmitter<SliderEvent> for Slider {}

impl Render for Slider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let track_height = self.size.track_height();
        let thumb_size = self.size.thumb_size();
        let percent = self.value_to_percent();

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("slider")
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .opacity(opacity)
            // Label row
            .when(self.label.is_some() || self.show_value, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .when_some(self.label.clone(), |d, label| {
                            d.child(div().text_sm().text_color(theme.colors.text).child(label))
                        })
                        .when(self.show_value, |d| {
                            d.child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child(self.format_value()),
                            )
                        }),
                )
            })
            // Track and thumb
            .child(
                div()
                    .id("slider-track-container")
                    .w_full()
                    .h(px(thumb_size))
                    .flex()
                    .items_center()
                    .relative()
                    .when(!self.disabled, |d| d.cursor_pointer())
                    // Track background
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .h(px(track_height))
                            .rounded_full()
                            .bg(theme.colors.surface_hover),
                    )
                    // Track fill
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .h(px(track_height))
                            .w(pct(percent))
                            .rounded_full()
                            .bg(theme.colors.accent),
                    )
                    // Marks
                    .children(self.marks.iter().map(|mark| {
                        let mark_percent = if self.max > self.min {
                            ((mark.value - self.min) / (self.max - self.min)) * 100.0
                        } else {
                            0.0
                        };

                        div()
                            .absolute()
                            .left(pct(mark_percent))
                            .top(px(thumb_size / 2.0 - 2.0))
                            .w(px(2.0))
                            .h(px(track_height + 4.0))
                            .bg(theme.colors.border)
                            .rounded_full()
                    }))
                    // Thumb
                    .child(
                        div()
                            .absolute()
                            .left(pct(percent))
                            .ml(px(-thumb_size / 2.0))
                            .size(px(thumb_size))
                            .rounded_full()
                            .bg(gpui::white())
                            .border_2()
                            .border_color(theme.colors.accent)
                            .shadow_sm(),
                    ),
            )
            // Mark labels
            .when(!self.marks.is_empty(), |d| {
                let has_labels = self.marks.iter().any(|m| m.label.is_some());
                if has_labels {
                    d.child(div().w_full().relative().h(px(20.0)).children(
                        self.marks.iter().filter_map(|mark| {
                            mark.label.as_ref().map(|label| {
                                let mark_percent = if self.max > self.min {
                                    ((mark.value - self.min) / (self.max - self.min)) * 100.0
                                } else {
                                    0.0
                                };

                                div()
                                    .absolute()
                                    .left(pct(mark_percent))
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(label.clone())
                            })
                        }),
                    ))
                } else {
                    d
                }
            })
    }
}
