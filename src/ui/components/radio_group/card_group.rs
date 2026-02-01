use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Radio group with card-style options
#[derive(IntoElement)]
pub struct RadioCardGroup {
    options: Vec<RadioCardOption>,
    selected: Option<SharedString>,
    orientation: RadioGroupOrientation,
    columns: Option<usize>,
    gap: f32,
    disabled: bool,
    accent_color: Option<Hsla>,
}

impl RadioCardGroup {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            orientation: RadioGroupOrientation::Vertical,
            columns: None,
            gap: 12.0,
            disabled: false,
            accent_color: None,
        }
    }

    pub fn option(mut self, option: RadioCardOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = RadioCardOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
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

impl Default for RadioCardGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for RadioCardGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent_color = self.accent_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let mut container = div().flex().gap(px(self.gap));

        if self.columns.is_some() {
            container = container.flex_wrap();
        } else {
            container = match self.orientation {
                RadioGroupOrientation::Vertical => container.flex_col(),
                RadioGroupOrientation::Horizontal => container.flex_row().flex_wrap(),
            };
        }

        for option in &self.options {
            let is_selected = self.selected.as_ref() == Some(&option.value);
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
                .cursor_pointer()
                .relative();

            if let Some(cols) = self.columns {
                let width_percent = 100.0 / cols as f32;
                card = card.w(px(width_percent * 3.0)); // Approximate width
            } else if matches!(self.orientation, RadioGroupOrientation::Horizontal) {
                card = card.flex_1();
            } else {
                card = card.w_full();
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

            // Badge
            if let Some(badge) = &option.badge {
                card = card.child(
                    div()
                        .absolute()
                        .top(px(-8.0))
                        .right(px(8.0))
                        .bg(accent_color)
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        })
                        .text_size(px(11.0))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .px_2()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .child(badge.clone()),
                );
            }

            // Header row with icon and radio indicator
            let mut header = div().flex().items_center().justify_between();

            let mut left_content = div().flex().items_center().gap_2();

            if let Some(icon) = &option.icon {
                left_content = left_content.child(
                    div()
                        .text_size(px(20.0))
                        .child(icon.clone()),
                );
            }

            left_content = left_content.child(
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

            header = header.child(left_content);

            // Radio indicator
            let mut radio_indicator = div()
                .w(px(20.0))
                .h(px(20.0))
                .rounded_full()
                .border_2()
                .flex()
                .items_center()
                .justify_center();

            if is_selected {
                radio_indicator = radio_indicator.border_color(accent_color).child(
                    div()
                        .w(px(10.0))
                        .h(px(10.0))
                        .rounded_full()
                        .bg(accent_color),
                );
            } else {
                radio_indicator = radio_indicator.border_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.4,
                    a: 1.0,
                });
            }

            header = header.child(radio_indicator);

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
                        .line_height(px(18.0))
                        .child(description.clone()),
                );
            }

            // Price
            if let Some(price) = &option.price {
                card = card.child(
                    div()
                        .text_size(px(16.0))
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(accent_color)
                        .mt_1()
                        .child(price.clone()),
                );
            }

            container = container.child(card);
        }

        container
    }
}
