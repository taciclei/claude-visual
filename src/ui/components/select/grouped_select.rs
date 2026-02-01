//! Grouped select - select with option groups

use gpui::prelude::*;
use gpui::*;

use super::types::{SelectGroup, SelectOption, SelectSize};

/// Grouped select - select with option groups
#[derive(IntoElement)]
pub struct GroupedSelect {
    id: ElementId,
    groups: Vec<SelectGroup>,
    selected: Option<SharedString>,
    placeholder: SharedString,
    size: SelectSize,
    disabled: bool,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl GroupedSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            groups: Vec::new(),
            selected: None,
            placeholder: "Select...".into(),
            size: SelectSize::default(),
            disabled: false,
            background: None,
            border_color: None,
        }
    }

    pub fn groups(mut self, groups: Vec<SelectGroup>) -> Self {
        self.groups = groups;
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
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

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    fn find_option(&self, value: &SharedString) -> Option<&SelectOption> {
        for group in &self.groups {
            if let Some(opt) = group.options.iter().find(|o| &o.value == value) {
                return Some(opt);
            }
        }
        None
    }
}

impl RenderOnce for GroupedSelect {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let display_text = self
            .selected
            .as_ref()
            .and_then(|v| self.find_option(v))
            .map(|o| o.label.clone())
            .unwrap_or(self.placeholder.clone());

        let is_placeholder = self.selected.is_none();

        div()
            .id(self.id)
            .h(px(36.0))
            .px(px(12.0))
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(if is_placeholder {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    })
                    .child(display_text),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("▼"),
            )
    }
}
