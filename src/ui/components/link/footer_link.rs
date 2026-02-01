//! Footer link component

use gpui::*;
use gpui::prelude::*;

use super::types::LinkSize;

/// Footer link
#[derive(IntoElement)]
pub struct FooterLink {
    label: SharedString,
    href: SharedString,
    size: LinkSize,
    color: Option<Hsla>,
}

impl FooterLink {
    pub fn new(label: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            size: LinkSize::Small,
            color: None,
        }
    }

    pub fn size(mut self, size: LinkSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for FooterLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });

        div()
            .text_size(px(self.size.font_size()))
            .text_color(color)
            .cursor_pointer()
            .hover(|s| {
                s.text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.7,
                    a: 1.0,
                })
            })
            .child(self.label)
    }
}
