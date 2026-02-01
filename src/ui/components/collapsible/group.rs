//! Group of collapsible sections (accordion-like but independent)

use gpui::*;
use gpui::prelude::*;
use super::collapsible::Collapsible;

/// A group of collapsible sections (accordion-like but independent)
#[derive(IntoElement)]
pub struct CollapsibleGroup {
    children: Vec<Collapsible>,
    gap: f32,
    border: bool,
    border_color: Option<Hsla>,
}

impl CollapsibleGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 8.0,
            border: false,
            border_color: None,
        }
    }

    pub fn child(mut self, collapsible: Collapsible) -> Self {
        self.children.push(collapsible);
        self
    }

    pub fn children(mut self, collapsibles: impl IntoIterator<Item = Collapsible>) -> Self {
        self.children.extend(collapsibles);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl Default for CollapsibleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CollapsibleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div().flex().flex_col().gap(px(self.gap)).w_full();

        if self.border {
            let border_color = self.border_color.unwrap_or(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.3,
                a: 1.0,
            });
            container = container
                .border_1()
                .border_color(border_color)
                .rounded_lg()
                .p_2();
        }

        for collapsible in self.children {
            container = container.child(collapsible);
        }

        container
    }
}
