//! Number input component struct and methods

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;
use super::types::*;

/// Number input with increment/decrement buttons
pub struct NumberInput {
    pub(super) app_state: Arc<AppState>,
    pub(super) value: f64,
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) step: f64,
    pub(super) precision: usize,
    pub(super) size: NumberInputSize,
    pub(super) disabled: bool,
    pub(super) show_stepper: bool,
    pub(super) label: Option<String>,
    pub(super) suffix: Option<String>,
    pub(super) prefix: Option<String>,
    pub(super) focus_handle: FocusHandle,
}

impl NumberInput {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            precision: 0,
            size: NumberInputSize::default(),
            disabled: false,
            show_stepper: true,
            label: None,
            suffix: None,
            prefix: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: f64, cx: &mut Context<Self>) {
        let clamped = self.clamp_value(value);
        if (clamped - self.value).abs() > f64::EPSILON {
            self.value = clamped;
            cx.emit(NumberInputEvent::Changed(clamped));
            cx.notify();
        }
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set minimum
    pub fn set_min(&mut self, min: Option<f64>, cx: &mut Context<Self>) {
        self.min = min;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    /// Set maximum
    pub fn set_max(&mut self, max: Option<f64>, cx: &mut Context<Self>) {
        self.max = max;
        self.value = self.clamp_value(self.value);
        cx.notify();
    }

    /// Set step
    pub fn set_step(&mut self, step: f64, cx: &mut Context<Self>) {
        self.step = step.max(0.001);
        cx.notify();
    }

    /// Set precision
    pub fn set_precision(&mut self, precision: usize, cx: &mut Context<Self>) {
        self.precision = precision;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: NumberInputSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set show stepper
    pub fn set_show_stepper(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_stepper = show;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set suffix
    pub fn set_suffix(&mut self, suffix: Option<String>, cx: &mut Context<Self>) {
        self.suffix = suffix;
        cx.notify();
    }

    /// Set prefix
    pub fn set_prefix(&mut self, prefix: Option<String>, cx: &mut Context<Self>) {
        self.prefix = prefix;
        cx.notify();
    }

    /// Increment value
    pub fn increment(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_value(self.value + self.step, cx);
        }
    }

    /// Decrement value
    pub fn decrement(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_value(self.value - self.step, cx);
        }
    }

    /// Clamp value to min/max
    pub(super) fn clamp_value(&self, value: f64) -> f64 {
        let mut v = value;
        if let Some(min) = self.min {
            v = v.max(min);
        }
        if let Some(max) = self.max {
            v = v.min(max);
        }
        v
    }

    /// Format value for display
    pub(super) fn format_value(&self) -> String {
        if self.precision == 0 {
            format!("{}", self.value as i64)
        } else {
            format!("{:.prec$}", self.value, prec = self.precision)
        }
    }

    /// Can increment
    pub(super) fn can_increment(&self) -> bool {
        !self.disabled && self.max.map(|m| self.value < m).unwrap_or(true)
    }

    /// Can decrement
    pub(super) fn can_decrement(&self) -> bool {
        !self.disabled && self.min.map(|m| self.value > m).unwrap_or(true)
    }
}

impl EventEmitter<NumberInputEvent> for NumberInput {}

impl Focusable for NumberInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
