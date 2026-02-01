//! Anchor link with hash navigation

use gpui::prelude::*;
use gpui::*;

/// Anchor link with hash navigation
#[derive(IntoElement)]
pub struct AnchorLink {
    label: SharedString,
    anchor_id: SharedString,
    show_hash: bool,
    color: Option<Hsla>,
}

impl AnchorLink {
    pub fn new(label: impl Into<SharedString>, anchor_id: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            anchor_id: anchor_id.into(),
            show_hash: false,
            color: None,
        }
    }

    pub fn show_hash(mut self, show: bool) -> Self {
        self.show_hash = show;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for AnchorLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        });

        div()
            .flex()
            .items_center()
            .gap_1()
            .text_size(px(14.0))
            .text_color(color)
            .cursor_pointer()
            .hover(|s| {
                s.text_color(Hsla {
                    h: 0.58,
                    s: 0.7,
                    l: 0.7,
                    a: 1.0,
                })
            })
            .when(self.show_hash, |d| {
                d.child(
                    div()
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.4,
                            a: 1.0,
                        })
                        .child("#"),
                )
            })
            .child(self.label)
    }
}
