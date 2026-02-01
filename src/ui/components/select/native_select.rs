//! Native select - uses native browser select element styling

use gpui::prelude::*;
use gpui::*;

use super::types::{SelectOption, SelectSize};

/// Native select - uses native browser select element styling
#[derive(IntoElement)]
pub struct NativeSelect {
    id: ElementId,
    options: Vec<SelectOption>,
    selected: Option<SharedString>,
    size: SelectSize,
    disabled: bool,
    background: Option<gpui::Hsla>,
}

impl NativeSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: Vec::new(),
            selected: None,
            size: SelectSize::default(),
            disabled: false,
            background: None,
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
}

impl RenderOnce for NativeSelect {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));

        let (height, font_size) = match self.size {
            SelectSize::Small => (28.0, 12.0),
            SelectSize::Medium => (36.0, 14.0),
            SelectSize::Large => (44.0, 16.0),
        };

        let display_text = self
            .selected
            .as_ref()
            .and_then(|v| self.options.iter().find(|o| &o.value == v))
            .map(|o| o.label.clone())
            .unwrap_or("Select...".into());

        div()
            .id(self.id)
            .h(px(height))
            .px(px(12.0))
            .flex()
            .items_center()
            .justify_between()
            .rounded(px(6.0))
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
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
