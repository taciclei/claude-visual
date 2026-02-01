//! Multi-select component - allows selecting multiple options

use gpui::prelude::*;
use gpui::*;

use super::types::{SelectOption, SelectSize};

/// Multi-select component - allows selecting multiple options
#[derive(IntoElement)]
pub struct MultiSelect {
    id: ElementId,
    options: Vec<SelectOption>,
    selected: Vec<SharedString>,
    placeholder: SharedString,
    size: SelectSize,
    disabled: bool,
    max_items: Option<usize>,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl MultiSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: Vec::new(),
            selected: Vec::new(),
            placeholder: "Select...".into(),
            size: SelectSize::default(),
            disabled: false,
            max_items: None,
            background: None,
            border_color: None,
        }
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn selected(mut self, values: Vec<SharedString>) -> Self {
        self.selected = values;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
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

impl RenderOnce for MultiSelect {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let selected_labels: Vec<SharedString> = self
            .selected
            .iter()
            .filter_map(|v| self.options.iter().find(|o| &o.value == v))
            .map(|o| o.label.clone())
            .collect();

        div()
            .id(self.id)
            .min_h(px(36.0))
            .px(px(12.0))
            .py(px(6.0))
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .when(selected_labels.is_empty(), |el| {
                el.child(
                    div()
                        .text_size(px(14.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child(self.placeholder.clone()),
                )
            })
            .children(selected_labels.into_iter().map(|label| {
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(hsla(0.0, 0.0, 0.3, 1.0))
                    .text_size(px(12.0))
                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(label)
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                            .child("×"),
                    )
            }))
    }
}
