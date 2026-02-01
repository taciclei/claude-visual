//! Navigation link with active state

use gpui::prelude::*;
use gpui::*;

/// Navigation link with active state
#[derive(IntoElement)]
pub struct NavLink {
    label: SharedString,
    href: SharedString,
    active: bool,
    icon: Option<SharedString>,
    badge: Option<SharedString>,
    color: Option<Hsla>,
    active_color: Option<Hsla>,
    active_bg: Option<Hsla>,
}

impl NavLink {
    pub fn new(label: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            active: false,
            icon: None,
            badge: None,
            color: None,
            active_color: None,
            active_bg: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn active_color(mut self, color: Hsla) -> Self {
        self.active_color = Some(color);
        self
    }

    pub fn active_bg(mut self, color: Hsla) -> Self {
        self.active_bg = Some(color);
        self
    }
}

impl RenderOnce for NavLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.7,
            a: 1.0,
        });
        let active_color = self.active_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        });
        let active_bg = self.active_bg.unwrap_or(Hsla {
            h: 0.58,
            s: 0.5,
            l: 0.2,
            a: 1.0,
        });

        let text_color = if self.active { active_color } else { color };

        let mut link = div()
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .text_size(px(14.0))
            .text_color(text_color);

        if self.active {
            link = link.bg(active_bg);
        } else {
            link = link.hover(|s| {
                s.bg(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.2,
                    a: 1.0,
                })
            });
        }

        let content = div()
            .flex()
            .items_center()
            .gap_2()
            .when_some(self.icon, |d, icon| {
                d.child(div().text_size(px(16.0)).child(icon))
            })
            .child(self.label);

        link = link.child(content);

        if let Some(badge) = self.badge {
            link = link.child(
                div()
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    })
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 1.0,
                        a: 1.0,
                    })
                    .text_size(px(11.0))
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_full()
                    .child(badge),
            );
        }

        link
    }
}
