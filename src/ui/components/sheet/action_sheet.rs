//! ActionSheet component (iOS-style)

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Action sheet (iOS-style)
#[derive(Clone, IntoElement)]
pub struct ActionSheet {
    title: Option<String>,
    message: Option<String>,
    pub(crate) actions: Vec<SheetAction>,
    cancel_label: String,
}

impl ActionSheet {
    pub fn new() -> Self {
        Self {
            title: None,
            message: None,
            actions: Vec::new(),
            cancel_label: "Cancel".to_string(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn action(mut self, action: SheetAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn actions(mut self, actions: Vec<SheetAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }
}

impl Default for ActionSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ActionSheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let danger = hsla(0.0, 0.7, 0.5, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.5);

        div()
            .absolute()
            .inset_0()
            .bg(backdrop)
            .flex()
            .flex_col()
            .justify_end()
            .p_4()
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Actions group
                    .child(
                        div()
                            .w_full()
                            .bg(surface)
                            .rounded(px(12.0))
                            .overflow_hidden()
                            // Title and message
                            .when(self.title.is_some() || self.message.is_some(), |d| {
                                d.child(
                                    div()
                                        .w_full()
                                        .px_4()
                                        .py_3()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_1()
                                        .border_b_1()
                                        .border_color(border)
                                        .when_some(self.title.clone(), |d, title| {
                                            d.child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(text)
                                                    .child(title)
                                            )
                                        })
                                        .when_some(self.message.clone(), |d, msg| {
                                            d.child(
                                                div()
                                                    .text_xs()
                                                    .text_color(text_muted)
                                                    .text_center()
                                                    .child(msg)
                                            )
                                        })
                                )
                            })
                            // Action buttons
                            .children(
                                self.actions.into_iter().enumerate().map(|(idx, action)| {
                                    let is_first = idx == 0 && self.title.is_none() && self.message.is_none();
                                    let text_color = if action.destructive { danger } else { accent };

                                    div()
                                        .w_full()
                                        .py_3()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(if action.disabled { text_muted } else { text_color })
                                        .when(!is_first, |d| d.border_t_1().border_color(border))
                                        .when(!action.disabled, |d| {
                                            d.cursor_pointer()
                                                .hover(|s| s.bg(surface_hover))
                                        })
                                        .when(action.disabled, |d| d.opacity(0.5))
                                        .child(action.label)
                                })
                            )
                    )
                    // Cancel button
                    .child(
                        div()
                            .w_full()
                            .py_3()
                            .bg(surface)
                            .rounded(px(12.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(accent)
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child(self.cancel_label)
                    )
            )
    }
}
