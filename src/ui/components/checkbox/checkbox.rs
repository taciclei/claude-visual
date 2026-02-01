//! Basic checkbox component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// A single checkbox component
#[derive(IntoElement)]
pub struct Checkbox {
    id: SharedString,
    label: SharedString,
    state: CheckboxState,
    disabled: bool,
    size: CheckboxSize,
    description: Option<SharedString>,
    error: bool,
    color: Option<Hsla>,
    label_color: Option<Hsla>,
}

impl Checkbox {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: CheckboxState::Unchecked,
            disabled: false,
            size: CheckboxSize::Medium,
            description: None,
            error: false,
            color: None,
            label_color: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.state = if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };
        self
    }

    pub fn state(mut self, state: CheckboxState) -> Self {
        self.state = state;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.state = CheckboxState::Indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
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

impl RenderOnce for Checkbox {
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

        let is_checked = matches!(self.state, CheckboxState::Checked | CheckboxState::Indeterminate);

        let border_color = if self.error {
            Hsla {
                h: 0.0,
                s: 0.7,
                l: 0.5,
                a: 1.0,
            }
        } else if is_checked {
            accent_color
        } else {
            Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.4,
                a: 1.0,
            }
        };

        let box_size = self.size.box_size();
        let check_size = self.size.check_size();
        let font_size = self.size.font_size();

        let mut container = div()
            .flex()
            .items_start()
            .gap_2()
            .cursor_pointer();

        if self.disabled {
            container = container.opacity(0.5).cursor_not_allowed();
        }

        // Checkbox box
        let mut checkbox_box = div()
            .w(px(box_size))
            .h(px(box_size))
            .rounded(px(4.0))
            .border_2()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .mt(px(2.0));

        if is_checked {
            checkbox_box = checkbox_box.bg(accent_color);

            let check_icon = match self.state {
                CheckboxState::Checked => "✓",
                CheckboxState::Indeterminate => "−",
                CheckboxState::Unchecked => "",
            };

            checkbox_box = checkbox_box.child(
                div()
                    .text_size(px(check_size))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 1.0,
                        a: 1.0,
                    })
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(check_icon),
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

        container.child(checkbox_box).child(label_container)
    }
}
