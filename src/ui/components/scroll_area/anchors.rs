//! Scroll anchor and spy components

use gpui::prelude::*;
use gpui::*;

/// Anchor links for scroll navigation
#[derive(IntoElement)]
pub struct ScrollAnchor {
    id: SharedString,
    label: SharedString,
    active: bool,
    text_color: Option<Hsla>,
    active_color: Option<Hsla>,
}

impl ScrollAnchor {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            active: false,
            text_color: None,
            active_color: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn active_color(mut self, color: Hsla) -> Self {
        self.active_color = Some(color);
        self
    }
}

impl RenderOnce for ScrollAnchor {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });
        let active_color = self.active_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        });

        let color = if self.active {
            active_color
        } else {
            text_color
        };

        div()
            .py(px(4.0))
            .px_2()
            .cursor_pointer()
            .text_size(px(13.0))
            .text_color(color)
            .when(self.active, |d| {
                d.border_l_2().border_color(active_color).pl(px(6.0))
            })
            .hover(|s| s.text_color(active_color))
            .child(self.label)
    }
}

/// Table of contents / scroll spy navigation
#[derive(IntoElement)]
pub struct ScrollSpy {
    anchors: Vec<ScrollAnchor>,
    title: Option<SharedString>,
    sticky: bool,
}

impl ScrollSpy {
    pub fn new() -> Self {
        Self {
            anchors: Vec::new(),
            title: None,
            sticky: false,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn anchor(mut self, anchor: ScrollAnchor) -> Self {
        self.anchors.push(anchor);
        self
    }

    pub fn anchors(mut self, anchors: impl IntoIterator<Item = ScrollAnchor>) -> Self {
        self.anchors.extend(anchors);
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
}

impl Default for ScrollSpy {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ScrollSpy {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div().flex().flex_col();

        if self.sticky {
            container = container.top_0();
        }

        if let Some(title) = self.title {
            container = container.child(
                div()
                    .text_size(px(11.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.5,
                        a: 1.0,
                    })
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .mb_2()
                    .child(title.to_uppercase()),
            );
        }

        for anchor in self.anchors {
            container = container.child(anchor);
        }

        container
    }
}
