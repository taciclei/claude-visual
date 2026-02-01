//! Range slider component with two thumbs

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use crate::app::state::AppState;
use super::types::*;

/// Range slider with two thumbs
pub struct RangeSlider {
    app_state: Arc<AppState>,
    /// Start value
    start: f32,
    /// End value
    end: f32,
    /// Minimum value
    min: f32,
    /// Maximum value
    max: f32,
    /// Step size
    step: f32,
    /// Size variant
    size: SliderSize,
    /// Whether slider is disabled
    disabled: bool,
    /// Label
    label: Option<String>,
}

impl RangeSlider {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            start: 0.0,
            end: 100.0,
            min: 0.0,
            max: 100.0,
            step: 0.0,
            size: SliderSize::default(),
            disabled: false,
            label: None,
        }
    }

    pub fn set_range(&mut self, start: f32, end: f32, cx: &mut Context<Self>) {
        self.start = start.clamp(self.min, self.max);
        self.end = end.clamp(self.min, self.max);
        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);
        }
        cx.notify();
    }

    pub fn set_min(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min = min;
        self.start = self.start.max(min);
        self.end = self.end.max(min);
        cx.notify();
    }

    pub fn set_max(&mut self, max: f32, cx: &mut Context<Self>) {
        self.max = max;
        self.start = self.start.min(max);
        self.end = self.end.min(max);
        cx.notify();
    }

    pub fn set_step(&mut self, step: f32, cx: &mut Context<Self>) {
        self.step = step;
        cx.notify();
    }

    pub fn set_size(&mut self, size: SliderSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    fn start_percent(&self) -> f32 {
        if self.max <= self.min { return 0.0; }
        ((self.start - self.min) / (self.max - self.min)) * 100.0
    }

    fn end_percent(&self) -> f32 {
        if self.max <= self.min { return 100.0; }
        ((self.end - self.min) / (self.max - self.min)) * 100.0
    }
}

impl EventEmitter<RangeSliderEvent> for RangeSlider {}

impl Render for RangeSlider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let track_height = self.size.track_height();
        let thumb_size = self.size.thumb_size();
        let start_percent = self.start_percent();
        let end_percent = self.end_percent();

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("range-slider")
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .opacity(opacity)
            // Label
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text)
                                .child(label)
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child(format!("{} - {}", self.start as i32, self.end as i32))
                        )
                )
            })
            // Track and thumbs
            .child(
                div()
                    .id("range-slider-track")
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
                            .bg(theme.colors.surface_hover)
                    )
                    // Track fill (between thumbs)
                    .child(
                        div()
                            .absolute()
                            .left(pct(start_percent))
                            .w(pct(end_percent - start_percent))
                            .h(px(track_height))
                            .bg(theme.colors.accent)
                    )
                    // Start thumb
                    .child(
                        div()
                            .absolute()
                            .left(pct(start_percent))
                            .ml(px(-thumb_size / 2.0))
                            .size(px(thumb_size))
                            .rounded_full()
                            .bg(gpui::white())
                            .border_2()
                            .border_color(theme.colors.accent)
                            .shadow_sm()
                    )
                    // End thumb
                    .child(
                        div()
                            .absolute()
                            .left(pct(end_percent))
                            .ml(px(-thumb_size / 2.0))
                            .size(px(thumb_size))
                            .rounded_full()
                            .bg(gpui::white())
                            .border_2()
                            .border_color(theme.colors.accent)
                            .shadow_sm()
                    )
            )
    }
}
