//! Radio group component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// A group of radio buttons
pub struct RadioGroup {
    app_state: Arc<AppState>,
    /// Options
    options: Vec<RadioGroupOption>,
    /// Selected value
    selected: Option<String>,
    /// Whether group is disabled
    disabled: bool,
    /// Layout direction
    horizontal: bool,
}

impl RadioGroup {
    pub fn new(
        app_state: Arc<AppState>,
        options: Vec<RadioGroupOption>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            options,
            selected: None,
            disabled: false,
            horizontal: false,
        }
    }

    pub fn set_selected(&mut self, value: Option<String>, cx: &mut Context<Self>) {
        self.selected = value;
        cx.notify();
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    pub fn set_horizontal(&mut self, horizontal: bool, cx: &mut Context<Self>) {
        self.horizontal = horizontal;
        cx.notify();
    }

    fn select(&mut self, value: String, cx: &mut Context<Self>) {
        if self.selected.as_ref() != Some(&value) {
            self.selected = Some(value.clone());
            cx.emit(RadioGroupEvent::Changed(value));
            cx.notify();
        }
    }
}

impl EventEmitter<RadioGroupEvent> for RadioGroup {}

impl Render for RadioGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("radio-group")
            .flex()
            .when(self.horizontal, |d| d.flex_row().gap_4())
            .when(!self.horizontal, |d| d.flex_col().gap_2())
            .when(self.disabled, |d| d.opacity(0.5))
            .children(self.options.iter().enumerate().map(|(i, option)| {
                let is_selected = self.selected.as_ref() == Some(&option.value);
                let is_disabled = self.disabled || option.disabled;
                let option_value = option.value.clone();

                div()
                    .id(SharedString::from(format!("radio-{}", i)))
                    .flex()
                    .items_start()
                    .gap_2()
                    .when(!is_disabled, |d| d.cursor_pointer())
                    .when(!is_disabled, |d| {
                        d.on_click(cx.listener(move |this, _, _window, cx| {
                            this.select(option_value.clone(), cx);
                        }))
                    })
                    // Radio circle
                    .child(
                        div()
                            .size(px(18.0))
                            .rounded_full()
                            .bg(theme.colors.surface)
                            .border_2()
                            .border_color(if is_selected {
                                theme.colors.accent
                            } else {
                                theme.colors.border
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .flex_shrink_0()
                            .mt(px(1.0))
                            .when(is_selected, |d| {
                                d.child(div().size(px(9.0)).rounded_full().bg(theme.colors.accent))
                            }),
                    )
                    // Label and description
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0p5()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(option.label.clone()),
                            )
                            .when_some(option.description.clone(), |d, desc| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(desc),
                                )
                            }),
                    )
            }))
    }
}
