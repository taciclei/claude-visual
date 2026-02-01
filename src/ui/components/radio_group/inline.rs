use gpui::*;
use gpui::prelude::*;

/// Simple inline radio buttons
#[derive(IntoElement)]
pub struct InlineRadio {
    options: Vec<(SharedString, SharedString)>, // (value, label)
    selected: Option<SharedString>,
    disabled: bool,
    color: Option<Hsla>,
}

impl InlineRadio {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            disabled: false,
            color: None,
        }
    }

    pub fn option(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for InlineRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for InlineRadio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent_color = self.color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let mut container = div().flex().items_center().gap_4();

        if self.disabled {
            container = container.opacity(0.5);
        }

        for (value, label) in &self.options {
            let is_selected = self.selected.as_ref() == Some(value);

            let mut option = div()
                .flex()
                .items_center()
                .gap_2()
                .cursor_pointer();

            if self.disabled {
                option = option.cursor_not_allowed();
            }

            // Radio circle
            let mut circle = div()
                .w(px(16.0))
                .h(px(16.0))
                .rounded_full()
                .border_2()
                .flex()
                .items_center()
                .justify_center();

            if is_selected {
                circle = circle.border_color(accent_color).child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(accent_color),
                );
            } else {
                circle = circle.border_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.4,
                    a: 1.0,
                });
            }

            option = option.child(circle).child(
                div()
                    .text_size(px(14.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.8,
                        a: 1.0,
                    })
                    .child(label.clone()),
            );

            container = container.child(option);
        }

        container
    }
}
