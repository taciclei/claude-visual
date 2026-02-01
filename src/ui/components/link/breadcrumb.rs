//! Breadcrumb link component

use gpui::*;
use gpui::prelude::*;

/// Breadcrumb link
#[derive(IntoElement)]
pub struct BreadcrumbLink {
    label: SharedString,
    href: Option<SharedString>,
    current: bool,
    icon: Option<SharedString>,
    color: Option<Hsla>,
    current_color: Option<Hsla>,
}

impl BreadcrumbLink {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: None,
            current: false,
            icon: None,
            color: None,
            current_color: None,
        }
    }

    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn current_color(mut self, color: Hsla) -> Self {
        self.current_color = Some(color);
        self
    }
}

impl RenderOnce for BreadcrumbLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.6,
            l: 0.6,
            a: 1.0,
        });
        let current_color = self.current_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });

        let text_color = if self.current { current_color } else { color };

        let mut link = div()
            .flex()
            .items_center()
            .gap_1()
            .text_size(px(13.0))
            .text_color(text_color);

        if !self.current && self.href.is_some() {
            link = link.cursor_pointer().hover(|s| {
                s.text_color(Hsla {
                    h: 0.58,
                    s: 0.7,
                    l: 0.7,
                    a: 1.0,
                })
            });
        }

        if let Some(icon) = self.icon {
            link = link.child(div().text_size(px(12.0)).child(icon));
        }

        link.child(self.label)
    }
}
