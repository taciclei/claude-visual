//! Radio button component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Radio button component
pub struct RadioButton {
    app_state: Arc<AppState>,
    /// Whether radio is selected
    selected: bool,
    /// Whether radio is disabled
    disabled: bool,
    /// Label
    label: Option<String>,
    /// Value (for radio groups)
    value: String,
    /// Size
    size: f32,
}

impl RadioButton {
    pub fn new(
        app_state: Arc<AppState>,
        value: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            selected: false,
            disabled: false,
            label: None,
            value: value.into(),
            size: 18.0,
        }
    }

    /// Create with label
    pub fn with_label(
        app_state: Arc<AppState>,
        value: impl Into<String>,
        label: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut radio = Self::new(app_state, value, cx);
        radio.label = Some(label.into());
        radio
    }

    /// Set selected
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        self.selected = selected;
        cx.notify();
    }

    /// Select this radio
    pub fn select(&mut self, cx: &mut Context<Self>) {
        if !self.disabled && !self.selected {
            self.selected = true;
            cx.emit(RadioButtonEvent::Selected(self.value.clone()));
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

impl EventEmitter<RadioButtonEvent> for RadioButton {}

impl Render for RadioButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let border_color = if self.selected {
            theme.colors.accent
        } else {
            theme.colors.border
        };
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id("radio-button")
            .flex()
            .items_center()
            .gap_2()
            .opacity(opacity)
            .when(!self.disabled, |d| d.cursor_pointer())
            .on_click(cx.listener(|this, _, _window, cx| {
                this.select(cx);
            }))
            // Radio circle
            .child(
                div()
                    .size(px(self.size))
                    .rounded_full()
                    .bg(theme.colors.surface)
                    .border_2()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .when(self.selected, |d| {
                        d.child(
                            div()
                                .size(px(self.size * 0.5))
                                .rounded_full()
                                .bg(theme.colors.accent),
                        )
                    }),
            )
            // Label
            .when_some(self.label.clone(), |d, label| {
                d.child(div().text_sm().text_color(theme.colors.text).child(label))
            })
    }
}
