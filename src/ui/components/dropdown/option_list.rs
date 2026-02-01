//! Simple option list component for inline selection

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// A simple option list for inline selection
pub struct OptionList {
    app_state: Arc<AppState>,
    /// Options
    options: Vec<DropdownOption>,
    /// Selected option ID
    selected: Option<String>,
    /// Whether selection is required
    required: bool,
}

impl OptionList {
    pub fn new(app_state: Arc<AppState>, options: Vec<DropdownOption>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            options,
            selected: None,
            required: false,
        }
    }

    pub fn set_selected(&mut self, id: Option<String>, cx: &mut Context<Self>) {
        self.selected = id;
        cx.notify();
    }

    pub fn set_required(&mut self, required: bool, cx: &mut Context<Self>) {
        self.required = required;
        cx.notify();
    }

    fn select(&mut self, id: String, cx: &mut Context<Self>) {
        if self.selected.as_ref() == Some(&id) && !self.required {
            // Deselect
            self.selected = None;
            cx.emit(OptionListEvent::Changed(None));
        } else {
            self.selected = Some(id.clone());
            cx.emit(OptionListEvent::Changed(Some(id)));
        }
        cx.notify();
    }
}

impl EventEmitter<OptionListEvent> for OptionList {}

impl Render for OptionList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("option-list")
            .flex()
            .flex_col()
            .gap_1()
            .children(self.options.iter().enumerate().map(|(i, option)| {
                let is_selected = self.selected.as_ref() == Some(&option.id);
                let option_id = option.id.clone();

                div()
                    .id(SharedString::from(format!("option-{}", i)))
                    .px_3()
                    .py_2()
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .gap_2()
                    .when(is_selected, |d| {
                        d.bg(theme.colors.accent.opacity(0.15))
                            .border_1()
                            .border_color(theme.colors.accent)
                    })
                    .when(!is_selected, |d| {
                        d.bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border)
                    })
                    .when(!option.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                    })
                    .when(option.disabled, |d| d.opacity(0.5))
                    .when(!option.disabled, |d| {
                        d.on_click(cx.listener(move |this, _, _window, cx| {
                            this.select(option_id.clone(), cx);
                        }))
                    })
                    // Radio indicator
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded_full()
                            .border_2()
                            .border_color(if is_selected { theme.colors.accent } else { theme.colors.border })
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(is_selected, |d| {
                                d.child(
                                    div()
                                        .size(px(8.0))
                                        .rounded_full()
                                        .bg(theme.colors.accent)
                                )
                            })
                    )
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
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(option.label.clone())
                            )
                            .when_some(option.description.clone(), |d, desc| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(desc)
                                )
                            })
                    )
            }))
    }
}
