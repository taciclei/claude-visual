//! Mention input component

use gpui::*;
use gpui::prelude::*;

/// Mention input - text input with mention support
#[derive(IntoElement)]
pub struct MentionInput {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    mentions: Vec<(usize, usize, SharedString)>, // (start, end, user_id)
    show_dropdown: bool,
    dropdown_query: SharedString,
}

impl MentionInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "".into(),
            placeholder: "Type @ to mention someone...".into(),
            mentions: Vec::new(),
            show_dropdown: false,
            dropdown_query: "".into(),
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn mentions(mut self, mentions: Vec<(usize, usize, SharedString)>) -> Self {
        self.mentions = mentions;
        self
    }

    pub fn show_dropdown(mut self, show: bool) -> Self {
        self.show_dropdown = show;
        self
    }

    pub fn dropdown_query(mut self, query: impl Into<SharedString>) -> Self {
        self.dropdown_query = query.into();
        self
    }
}

impl RenderOnce for MentionInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_value = !self.value.is_empty();

        div()
            .id(self.id)
            .relative()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .min_h(px(40.0))
                    .px(px(12.0))
                    .py(px(10.0))
                    .bg(hsla(0.0, 0.0, 0.08, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(if has_value {
                                hsla(0.0, 0.0, 0.9, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.5, 1.0)
                            })
                            .child(if has_value {
                                self.value.to_string()
                            } else {
                                self.placeholder.to_string()
                            })
                    )
            )
    }
}
