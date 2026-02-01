//! Tag input component - combobox that shows selected items as tags

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Tag input - combobox that shows selected items as tags
#[derive(IntoElement)]
pub struct TagInput {
    id: ElementId,
    items: Vec<ComboboxItem>,
    pub(crate) selected: Vec<SharedString>,
    query: SharedString,
    placeholder: SharedString,
    size: ComboboxSize,
    disabled: bool,
    pub(crate) allow_create: bool,
    pub(crate) max_tags: Option<usize>,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl TagInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: Vec::new(),
            query: "".into(),
            placeholder: "Add tags...".into(),
            size: ComboboxSize::default(),
            disabled: false,
            allow_create: false,
            max_tags: None,
            background: None,
            border_color: None,
        }
    }

    pub fn items(mut self, items: Vec<ComboboxItem>) -> Self {
        self.items = items;
        self
    }

    pub fn selected(mut self, values: Vec<SharedString>) -> Self {
        self.selected = values;
        self
    }

    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.query = query.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: ComboboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn allow_create(mut self, allow: bool) -> Self {
        self.allow_create = allow;
        self
    }

    pub fn max_tags(mut self, max: usize) -> Self {
        self.max_tags = Some(max);
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

impl RenderOnce for TagInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let selected_labels: Vec<SharedString> = self
            .selected
            .iter()
            .filter_map(|v| self.items.iter().find(|i| &i.value == v))
            .map(|i| i.label.clone())
            .collect();

        let can_add_more = self.max_tags.map_or(true, |max| self.selected.len() < max);

        div()
            .id(self.id)
            .min_h(px(36.0))
            .px(px(8.0))
            .py(px(6.0))
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .children(selected_labels.into_iter().map(|label| {
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(hsla(0.6, 0.5, 0.4, 0.3))
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
                            .cursor_pointer()
                            .child("×"),
                    )
            }))
            .when(can_add_more, |el| {
                el.child(
                    div()
                        .flex_1()
                        .min_w(px(80.0))
                        .text_size(px(14.0))
                        .text_color(if self.query.is_empty() {
                            hsla(0.0, 0.0, 0.5, 1.0)
                        } else {
                            hsla(0.0, 0.0, 0.9, 1.0)
                        })
                        .child(if self.query.is_empty() {
                            self.placeholder.clone()
                        } else {
                            self.query.clone()
                        }),
                )
            })
    }
}
