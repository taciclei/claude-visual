//! Checkbox card group component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Checkbox group with card-style options
#[derive(IntoElement)]
pub struct CheckboxCardGroup {
    options: Vec<CheckboxCardOption>,
    selected: Vec<SharedString>,
    columns: Option<usize>,
    gap: f32,
    disabled: bool,
    accent_color: Option<Hsla>,
}

impl CheckboxCardGroup {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            columns: None,
            gap: 12.0,
            disabled: false,
            accent_color: None,
        }
    }

    pub fn option(mut self, option: CheckboxCardOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = CheckboxCardOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn selected(mut self, selected: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.selected = selected.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn accent_color(mut self, color: Hsla) -> Self {
        self.accent_color = Some(color);
        self
    }
}

impl Default for CheckboxCardGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for CheckboxCardGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent_color = self.accent_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let mut container = div().flex().flex_wrap().gap(px(self.gap));

        for option in &self.options {
            let is_selected = self.selected.contains(&option.id);
            let is_disabled = self.disabled || option.disabled;

            let border_color = if is_selected {
                accent_color
            } else {
                Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.25,
                    a: 1.0,
                }
            };

            let bg_color = if is_selected {
                Hsla {
                    h: accent_color.h,
                    s: accent_color.s * 0.3,
                    l: 0.15,
                    a: 1.0,
                }
            } else {
                Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.1,
                    a: 1.0,
                }
            };

            let mut card = div()
                .flex()
                .flex_col()
                .gap_2()
                .p_4()
                .border_2()
                .border_color(border_color)
                .bg(bg_color)
                .rounded_lg()
                .cursor_pointer();

            if let Some(cols) = self.columns {
                let width = (100.0 / cols as f32) - 2.0;
                card = card.w(px(width * 3.0));
            } else {
                card = card.flex_1().min_w(px(200.0));
            }

            if is_disabled {
                card = card.opacity(0.5).cursor_not_allowed();
            } else {
                card = card.hover(|s| {
                    s.border_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.4,
                        a: 1.0,
                    })
                });
            }

            // Header with checkbox
            let mut header = div().flex().items_center().justify_between();

            let mut left = div().flex().items_center().gap_2();

            // Checkbox
            let mut checkbox_box = div()
                .w(px(20.0))
                .h(px(20.0))
                .rounded(px(4.0))
                .border_2()
                .flex()
                .items_center()
                .justify_center();

            if is_selected {
                checkbox_box = checkbox_box.border_color(accent_color).bg(accent_color).child(
                    div()
                        .text_size(px(12.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        })
                        .font_weight(gpui::FontWeight::BOLD)
                        .child("✓"),
                );
            } else {
                checkbox_box = checkbox_box.border_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.4,
                    a: 1.0,
                });
            }

            left = left.child(checkbox_box);

            if let Some(icon) = &option.icon {
                left = left.child(div().text_size(px(20.0)).child(icon.clone()));
            }

            left = left.child(
                div()
                    .text_size(px(15.0))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.95,
                        a: 1.0,
                    })
                    .child(option.title.clone()),
            );

            header = header.child(left);

            if let Some(price) = &option.price {
                header = header.child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(accent_color)
                        .child(price.clone()),
                );
            }

            card = card.child(header);

            // Description
            if let Some(description) = &option.description {
                card = card.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.6,
                            a: 1.0,
                        })
                        .pl(px(28.0)) // Align with title after checkbox
                        .child(description.clone()),
                );
            }

            container = container.child(card);
        }

        container
    }
}
