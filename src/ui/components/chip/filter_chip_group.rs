//! Filter chip group component

use gpui::prelude::*;
use gpui::*;

use super::filter_chip::FilterChip;

/// Filter chip group
#[derive(Clone)]
pub struct FilterChipGroup {
    pub(crate) chips: Vec<FilterChip>,
    pub(crate) allow_multiple: bool,
}

impl FilterChipGroup {
    pub fn new() -> Self {
        Self {
            chips: Vec::new(),
            allow_multiple: false,
        }
    }

    pub fn chip(mut self, chip: FilterChip) -> Self {
        self.chips.push(chip);
        self
    }

    pub fn allow_multiple(mut self) -> Self {
        self.allow_multiple = true;
        self
    }
}

impl Default for FilterChipGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FilterChipGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .flex()
            .flex_wrap()
            .gap_2()
            .children(self.chips.into_iter().map(|chip| {
                let (bg, border_col, text_col) = if chip.selected {
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
                        s.bg(if chip.selected {
                            accent.opacity(0.2)
                        } else {
                            surface_hover
                        })
                    })
                    .when(chip.selected, |d| {
                        d.child(div().text_xs().text_color(text_col).child("✓"))
                    })
                    .child(div().text_sm().text_color(text_col).child(chip.label))
                    .when_some(chip.count, |d, count| {
                        d.child(
                            div()
                                .px_1p5()
                                .rounded_full()
                                .bg(if chip.selected {
                                    accent.opacity(0.3)
                                } else {
                                    surface_hover
                                })
                                .text_xs()
                                .text_color(if chip.selected { accent } else { text_muted })
                                .child(count.to_string()),
                        )
                    })
            }))
    }
}
