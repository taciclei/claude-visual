//! Link list component

use gpui::prelude::*;
use gpui::*;

use super::footer_link::FooterLink;
use super::types::LinkListDirection;

/// Link list for navigation or footer
#[derive(IntoElement)]
pub struct LinkList {
    title: Option<SharedString>,
    links: Vec<FooterLink>,
    direction: LinkListDirection,
    gap: f32,
    title_color: Option<Hsla>,
}

impl LinkList {
    pub fn new() -> Self {
        Self {
            title: None,
            links: Vec::new(),
            direction: LinkListDirection::Vertical,
            gap: 8.0,
            title_color: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn link(mut self, link: FooterLink) -> Self {
        self.links.push(link);
        self
    }

    pub fn links(mut self, links: impl IntoIterator<Item = FooterLink>) -> Self {
        self.links.extend(links);
        self
    }

    pub fn direction(mut self, direction: LinkListDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn title_color(mut self, color: Hsla) -> Self {
        self.title_color = Some(color);
        self
    }
}

impl Default for LinkList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for LinkList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let title_color = self.title_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });

        let mut container = div().flex().flex_col().gap(px(self.gap));

        if let Some(title) = self.title {
            container = container.child(
                div()
                    .text_size(px(13.0))
                    .text_color(title_color)
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .mb_1()
                    .child(title),
            );
        }

        let mut links_container = match self.direction {
            LinkListDirection::Vertical => div().flex().flex_col().gap(px(self.gap)),
            LinkListDirection::Horizontal => div().flex().items_center().gap(px(self.gap)),
        };

        for link in self.links {
            links_container = links_container.child(link);
        }

        container.child(links_container)
    }
}
