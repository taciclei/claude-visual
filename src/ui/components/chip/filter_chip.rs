//! Simple filter chip component

use gpui::prelude::*;
use gpui::*;

/// Simple filter chip
#[derive(Clone)]
pub struct FilterChip {
    pub(crate) label: String,
    pub(crate) selected: bool,
    pub(crate) count: Option<u32>,
}

impl FilterChip {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            selected: false,
            count: None,
        }
    }

    pub fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}

impl RenderOnce for FilterChip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let (bg, border_col, text_col) = if self.selected {
            (accent.opacity(0.15), accent, accent)
        } else {
            (surface, border, text)
        };

        div()
            .h(px(32.0))
            .px_3()
            .rounded_full()
            .bg(bg)
            .border_1()
            .border_color(border_col)
            .flex()
            .items_center()
            .gap_2()
            .cursor_pointer()
            .hover(|s| {
                s.bg(if self.selected {
                    accent.opacity(0.2)
                } else {
                    surface_hover
                })
            })
            // Checkmark when selected
            .when(self.selected, |d| {
                d.child(div().text_xs().text_color(text_col).child("✓"))
            })
            // Label
            .child(div().text_sm().text_color(text_col).child(self.label))
            // Count badge
            .when_some(self.count, |d, count| {
                d.child(
                    div()
                        .px_1p5()
                        .rounded_full()
                        .bg(if self.selected {
                            accent.opacity(0.3)
                        } else {
                            surface_hover
                        })
                        .text_xs()
                        .text_color(if self.selected { accent } else { text_muted })
                        .child(count.to_string()),
                )
            })
    }
}
