use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// A single radio button
#[derive(IntoElement)]
pub struct Radio {
    value: SharedString,
    label: SharedString,
    checked: bool,
    disabled: bool,
    size: RadioSize,
    description: Option<SharedString>,
    error: bool,
    color: Option<Hsla>,
    label_color: Option<Hsla>,
}

impl Radio {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            size: RadioSize::Medium,
            description: None,
            error: false,
            color: None,
            label_color: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn label_color(mut self, color: Hsla) -> Self {
        self.label_color = Some(color);
        self
    }
}

impl RenderOnce for Radio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent_color = self.color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });
        let label_color = self.label_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });
        let border_color = if self.error {
            Hsla {
                h: 0.0,
                s: 0.7,
                l: 0.5,
                a: 1.0,
            }
        } else if self.checked {
            accent_color
        } else {
            Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.4,
                a: 1.0,
            }
        };

        let outer_size = self.size.outer_size();
        let inner_size = self.size.inner_size();
        let font_size = self.size.font_size();

        let mut container = div()
            .flex()
            .items_start()
            .gap_2()
            .cursor_pointer();

        if self.disabled {
            container = container.opacity(0.5).cursor_not_allowed();
        }

        // Radio circle
        let mut radio = div()
            .w(px(outer_size))
            .h(px(outer_size))
            .rounded_full()
            .border_2()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .mt(px(2.0));

        if self.checked {
            radio = radio.bg(accent_color).child(
                div()
                    .w(px(inner_size))
                    .h(px(inner_size))
                    .rounded_full()
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 1.0,
                        a: 1.0,
                    }),
            );
        }

        // Label and description
        let mut label_container = div().flex().flex_col().gap(px(2.0));

        label_container = label_container.child(
            div()
                .text_size(px(font_size))
                .text_color(label_color)
                .child(self.label),
        );

        if let Some(description) = self.description {
            label_container = label_container.child(
                div()
                    .text_size(px(font_size - 2.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.5,
                        a: 1.0,
                    })
                    .child(description),
            );
        }

        container.child(radio).child(label_container)
    }
}
