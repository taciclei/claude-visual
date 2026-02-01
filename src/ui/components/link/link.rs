//! Basic link component

use gpui::prelude::*;
use gpui::*;

use super::types::{LinkSize, LinkVariant};

/// A basic link component
#[derive(IntoElement)]
pub struct Link {
    href: SharedString,
    label: SharedString,
    variant: LinkVariant,
    size: LinkSize,
    color: Option<Hsla>,
    hover_color: Option<Hsla>,
    visited_color: Option<Hsla>,
    external: bool,
    disabled: bool,
    icon_before: Option<SharedString>,
    icon_after: Option<SharedString>,
}

impl Link {
    pub fn new(label: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            href: href.into(),
            label: label.into(),
            variant: LinkVariant::Default,
            size: LinkSize::Medium,
            color: None,
            hover_color: None,
            visited_color: None,
            external: false,
            disabled: false,
            icon_before: None,
            icon_after: None,
        }
    }

    pub fn variant(mut self, variant: LinkVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: LinkSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn hover_color(mut self, color: Hsla) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn visited_color(mut self, color: Hsla) -> Self {
        self.visited_color = Some(color);
        self
    }

    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_before(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon_before = Some(icon.into());
        self
    }

    pub fn icon_after(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon_after = Some(icon.into());
        self
    }
}

impl RenderOnce for Link {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let default_color = Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        };
        let color = self.color.unwrap_or(default_color);
        let hover_color = self.hover_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.7,
            a: 1.0,
        });

        let font_size = self.size.font_size();

        let mut link = div()
            .flex()
            .items_center()
            .gap_1()
            .text_size(px(font_size))
            .text_color(color)
            .cursor_pointer();

        // Apply variant styles
        link = match self.variant {
            LinkVariant::Default => link.hover(|s| s.text_color(hover_color)),
            LinkVariant::Subtle => link
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.6,
                    a: 1.0,
                })
                .hover(|s| s.text_color(color)),
            LinkVariant::Underline => link.underline().hover(|s| s.text_color(hover_color)),
            LinkVariant::Bold => link
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .hover(|s| s.text_color(hover_color)),
            LinkVariant::Button => link
                .bg(color)
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 1.0,
                    a: 1.0,
                })
                .px_3()
                .py_1()
                .rounded(px(4.0))
                .hover(|s| s.bg(hover_color)),
        };

        if self.disabled {
            link = link.opacity(0.5).cursor_not_allowed();
        }

        // Icon before
        if let Some(icon) = self.icon_before {
            link = link.child(div().text_size(px(font_size - 2.0)).child(icon));
        }

        // Label
        link = link.child(self.label);

        // External link indicator or custom icon after
        if self.external {
            link = link.child(
                div()
                    .text_size(px(font_size - 2.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.5,
                        a: 1.0,
                    })
                    .child("↗"),
            );
        } else if let Some(icon) = self.icon_after {
            link = link.child(div().text_size(px(font_size - 2.0)).child(icon));
        }

        link
    }
}
