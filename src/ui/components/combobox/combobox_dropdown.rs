//! Combobox dropdown list component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Combobox dropdown list
#[derive(IntoElement)]
pub struct ComboboxDropdown {
    items: Vec<ComboboxItem>,
    selected: Option<SharedString>,
    highlighted_index: Option<usize>,
    max_height: f32,
    empty_text: SharedString,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl ComboboxDropdown {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            highlighted_index: None,
            max_height: 200.0,
            empty_text: "No results found".into(),
            background: None,
            border_color: None,
        }
    }

    pub fn items(mut self, items: Vec<ComboboxItem>) -> Self {
        self.items = items;
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn highlighted_index(mut self, index: usize) -> Self {
        self.highlighted_index = Some(index);
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
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

impl Default for ComboboxDropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ComboboxDropdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.25, 1.0));

        div()
            .w_full()
            .max_h(px(self.max_height))
            .id("scroll-combobox-dropdown")
            .overflow_y_scroll()
            .rounded_b(px(6.0))
            .border_1()
            .border_t_0()
            .border_color(border)
            .bg(bg)
            .shadow_lg()
            .when(self.items.is_empty(), |el| {
                el.child(
                    div()
                        .px(px(12.0))
                        .py(px(12.0))
                        .text_size(px(14.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .text_center()
                        .child(self.empty_text.clone()),
                )
            })
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = self.selected.as_ref() == Some(&item.value);
                let is_highlighted = self.highlighted_index == Some(index);

                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .when(item.disabled, |el| el.opacity(0.5).cursor_not_allowed())
                    .when(!item.disabled && !is_highlighted, |el| {
                        el.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                    })
                    .when(is_highlighted, |el| el.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                    .when(is_selected, |el| el.bg(hsla(0.6, 0.5, 0.3, 0.3)))
                    .when(item.icon.is_some(), |el| {
                        el.child(
                            div()
                                .text_size(px(14.0))
                                .child(item.icon.clone().unwrap_or_default()),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(item.label.clone()),
                            )
                            .when(item.description.is_some(), |el| {
                                el.child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child(item.description.unwrap_or_default()),
                                )
                            }),
                    )
                    .when(is_selected, |el| {
                        el.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(hsla(0.6, 0.7, 0.5, 1.0))
                                .child("✓"),
                        )
                    })
            }))
    }
}
