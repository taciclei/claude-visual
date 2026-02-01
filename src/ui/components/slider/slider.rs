//! Single-value slider component

use std::sync::Arc;
use gpui::*;
use crate::app::state::AppState;
use super::types::*;

/// Slider component for range input
pub struct Slider {
    pub(super) app_state: Arc<AppState>,
    /// Current value
    pub(super) value: f32,
    /// Minimum value
    pub(super) min: f32,
    /// Maximum value
    pub(super) max: f32,
    /// Step size (0 for continuous)
    pub(super) step: f32,
    /// Size variant
    pub(super) size: SliderSize,
    /// Whether slider is disabled
    pub(super) disabled: bool,
    /// Whether to show current value label
    pub(super) show_value: bool,
    /// Label text
    pub(super) label: Option<String>,
    /// Marks/ticks on the track
    pub(super) marks: Vec<SliderMark>,
    /// Custom value formatter
    pub(super) value_suffix: Option<String>,
}

impl Slider {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 0.0,
            size: SliderSize::default(),
            disabled: false,
            show_value: false,
            label: None,
            marks: Vec::new(),
            value_suffix: None,
        }
    }

    /// Create a slider with range
    pub fn with_range(app_state: Arc<AppState>, min: f32, max: f32, cx: &mut Context<Self>) -> Self {
        let mut slider = Self::new(app_state, cx);
        slider.min = min;
        slider.max = max;
        slider.value = min;
        slider
    }

    /// Set value
    pub fn set_value(&mut self, value: f32, cx: &mut Context<Self>) {
        self.value = self.clamp_value(value);
        cx.notify();
    }

    /// Set minimum
    pub fn set_min(&mut self, min: f32, cx: &mut Context<Self>) {
        self.min = min;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    /// Set maximum
    pub fn set_max(&mut self, max: f32, cx: &mut Context<Self>) {
        self.max = max;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    /// Set step size
    pub fn set_step(&mut self, step: f32, cx: &mut Context<Self>) {
        self.step = step;
        self.value = self.snap_to_step(self.value);
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: SliderSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set show value
    pub fn set_show_value(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_value = show;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set marks
    pub fn set_marks(&mut self, marks: Vec<SliderMark>, cx: &mut Context<Self>) {
        self.marks = marks;
        cx.notify();
    }

    /// Set value suffix
    pub fn set_value_suffix(&mut self, suffix: Option<String>, cx: &mut Context<Self>) {
        self.value_suffix = suffix;
        cx.notify();
    }

    /// Get current value
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Clamp value to range
    fn clamp_value(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    /// Snap value to step
    fn snap_to_step(&self, value: f32) -> f32 {
        if self.step <= 0.0 {
            return value;
        }
        let steps = ((value - self.min) / self.step).round();
        self.clamp_value(self.min + steps * self.step)
    }

    /// Calculate percentage from value
    pub(super) fn value_to_percent(&self) -> f32 {
        if self.max <= self.min {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)) * 100.0
    }

    /// Format value for display
    pub(super) fn format_value(&self) -> String {
        let formatted = if self.step >= 1.0 {
            format!("{}", self.value as i32)
        } else {
            format!("{:.1}", self.value)
        };

        if let Some(suffix) = &self.value_suffix {
            format!("{}{}", formatted, suffix)
        } else {
            formatted
        }
    }
}
