//! Checkbox component

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// A simple checkbox component
pub struct Checkbox {
    app_state: Arc<AppState>,
    /// Whether checkbox is checked
    checked: bool,
    /// Indeterminate state (for partial selection)
    indeterminate: bool,
    /// Whether checkbox is disabled
    disabled: bool,
    /// Label
    label: Option<String>,
    /// Size
    size: f32,
}

impl Checkbox {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            checked: false,
            indeterminate: false,
            disabled: false,
            label: None,
            size: 18.0,
        }
    }

    /// Create with label
    pub fn with_label(app_state: Arc<AppState>, label: impl Into<String>, cx: &mut Context<Self>) -> Self {
        let mut checkbox = Self::new(app_state, cx);
        checkbox.label = Some(label.into());
        checkbox
    }

    /// Set checked
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        self.checked = checked;
        self.indeterminate = false;
        cx.notify();
    }

    /// Set indeterminate
    pub fn set_indeterminate(&mut self, indeterminate: bool, cx: &mut Context<Self>) {
        self.indeterminate = indeterminate;
        cx.notify();
    }

    /// Toggle
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            self.indeterminate = false;
            cx.emit(CheckboxEvent::Changed(self.checked));
            cx.notify();
        }
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: f32, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl EventEmitter<CheckboxEvent> for Checkbox {}

impl Render for Checkbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let (bg_color, border_color, icon) = if self.indeterminate {
            (theme.colors.accent, theme.colors.accent, Some("−"))
        } else if self.checked {
            (theme.colors.accent, theme.colors.accent, Some("✓"))
        } else {
            (theme.colors.surface, theme.colors.border, None)
        };

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("checkbox")
            .flex()
            .items_center()
            .gap_2()
            .opacity(opacity)
            .when(!self.disabled, |d| d.cursor_pointer())
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle(cx);
            }))
            // Checkbox box
            .child(
                div()
                    .size(px(self.size))
                    .rounded(px(4.0))
                    .bg(bg_color)
                    .border_2()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .when_some(icon, |d, icon| {
                        d.text_color(gpui::white())
                            .text_size(px(self.size * 0.7))
                            .font_weight(FontWeight::BOLD)
                            .child(icon)
                    })
            )
            // Label
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text)
                        .child(label)
                )
            })
    }
}
