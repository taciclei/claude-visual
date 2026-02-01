//! Simple toggle checkbox component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Simple toggle checkbox (without label text)
#[derive(IntoElement)]
pub struct CheckboxToggle {
    checked: bool,
    disabled: bool,
    size: CheckboxSize,
    color: Option<Hsla>,
}

impl CheckboxToggle {
    pub fn new() -> Self {
        Self {
            checked: false,
            disabled: false,
            size: CheckboxSize::Medium,
            color: None,
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

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for CheckboxToggle {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CheckboxToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent_color = self.color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let box_size = self.size.box_size();
        let check_size = self.size.check_size();

        let border_color = if self.checked {
            accent_color
        } else {
            Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.4,
                a: 1.0,
            }
        };

        let mut checkbox = div()
            .w(px(box_size))
            .h(px(box_size))
            .rounded(px(4.0))
            .border_2()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer();

        if self.checked {
            checkbox = checkbox.bg(accent_color).child(
                div()
                    .text_size(px(check_size))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 1.0,
                        a: 1.0,
                    })
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("✓"),
            );
        }

        if self.disabled {
            checkbox = checkbox.opacity(0.5).cursor_not_allowed();
        } else {
            checkbox = checkbox.hover(|s| {
                s.border_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.6,
                    a: 1.0,
                })
            });
        }

        checkbox
    }
}
