//! Select dropdown menu (the dropdown portion)

use gpui::prelude::*;
use gpui::*;

use super::types::SelectOption;

/// Select dropdown menu (the dropdown portion)
#[derive(IntoElement)]
pub struct SelectDropdown {
    options: Vec<SelectOption>,
    selected: Option<SharedString>,
    max_height: f32,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl SelectDropdown {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            max_height: 200.0,
            background: None,
            border_color: None,
        }
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl Default for SelectDropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SelectDropdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.25, 1.0));

        div()
            .w_full()
            .max_h(px(self.max_height))
            .id("scroll-select-dropdown")
            .overflow_y_scroll()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .shadow_lg()
            .py(px(4.0))
            .children(self.options.into_iter().map(|option| {
                let is_selected = self.selected.as_ref() == Some(&option.value);

                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .when(option.disabled, |el| el.opacity(0.5).cursor_not_allowed())
                    .when(!option.disabled, |el| {
                        el.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                    })
                    .when(is_selected, |el| el.bg(hsla(0.6, 0.5, 0.3, 0.3)))
                    .when(option.icon.is_some(), |el| {
                        el.child(
                            div()
                                .text_size(px(14.0))
                                .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                .child(option.icon.clone().unwrap_or_default()),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(option.label.clone()),
                            )
                            .when(option.description.is_some(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child(option.description.unwrap_or_default()),
                                )
                            }),
                    )
                    .when(is_selected, |el| {
                        el.child(
                            div()
                                .ml_auto()
                                .text_size(px(12.0))
                                .text_color(hsla(0.6, 0.7, 0.5, 1.0))
                                .child("✓"),
                        )
                    })
            }))
    }
}
