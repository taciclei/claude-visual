//! Autocomplete component - simple text autocomplete without selection

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Autocomplete - simple text autocomplete without selection
#[derive(IntoElement)]
pub struct Autocomplete {
    id: ElementId,
    suggestions: Vec<SharedString>,
    value: SharedString,
    placeholder: SharedString,
    size: ComboboxSize,
    disabled: bool,
    open: bool,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl Autocomplete {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            suggestions: Vec::new(),
            value: "".into(),
            placeholder: "Type to search...".into(),
            size: ComboboxSize::default(),
            disabled: false,
            open: false,
            background: None,
            border_color: None,
        }
    }

    pub fn suggestions(mut self, suggestions: Vec<SharedString>) -> Self {
        self.suggestions = suggestions;
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
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

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

impl RenderOnce for Autocomplete {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = match self.size {
            ComboboxSize::Small => (28.0, 8.0, 12.0),
            ComboboxSize::Medium => (36.0, 12.0, 14.0),
            ComboboxSize::Large => (44.0, 16.0, 16.0),
        };

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        div()
            .id(self.id)
            .h(px(height))
            .px(px(padding_x))
            .flex()
            .items_center()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .when(self.disabled, |el| {
                el.opacity(0.5).cursor_not_allowed()
            })
            .child(
                div()
                    .flex_1()
                    .text_size(px(font_size))
                    .text_color(if self.value.is_empty() {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    })
                    .child(if self.value.is_empty() {
                        self.placeholder.clone()
                    } else {
                        self.value.clone()
                    })
            )
    }
}
