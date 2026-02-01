//! Section separator with icon

use gpui::prelude::*;
use gpui::*;

/// Section separator with icon
#[derive(IntoElement)]
pub struct SectionSeparator {
    icon: Option<SharedString>,
    color: Option<Hsla>,
    icon_color: Option<Hsla>,
    icon_background: Option<Hsla>,
}

impl SectionSeparator {
    pub fn new() -> Self {
        Self {
            icon: None,
            color: None,
            icon_color: None,
            icon_background: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn icon_background(mut self, color: Hsla) -> Self {
        self.icon_background = Some(color);
        self
    }
}

impl Default for SectionSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SectionSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });
        let icon_color = self.icon_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });
        let icon_bg = self.icon_background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.15,
            a: 1.0,
        });

        let line = || div().h(px(1.0)).flex_1().bg(line_color);

        let mut container = div().flex().items_center().w_full().my_4();

        container = container.child(line());

        if let Some(icon) = self.icon {
            container = container.child(
                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .mx_4()
                    .bg(icon_bg)
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .text_color(icon_color)
                    .child(icon),
            );
        }

        container.child(line())
    }
}
