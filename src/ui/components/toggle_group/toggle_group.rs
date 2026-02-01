//! Toggle group component (single selection)

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Toggle group component (single selection)
#[derive(IntoElement)]
pub struct ToggleGroup {
    items: Vec<ToggleItem>,
    selected: Option<SharedString>,
    size: ToggleGroupSize,
    variant: ToggleGroupVariant,
    disabled: bool,
    full_width: bool,
    background: Option<Hsla>,
    selected_background: Option<Hsla>,
    text_color: Option<Hsla>,
    selected_text_color: Option<Hsla>,
}

impl ToggleGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            size: ToggleGroupSize::Medium,
            variant: ToggleGroupVariant::Default,
            disabled: false,
            full_width: false,
            background: None,
            selected_background: None,
            text_color: None,
            selected_text_color: None,
        }
    }

    pub fn item(mut self, item: ToggleItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ToggleItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn selected_background(mut self, color: Hsla) -> Self {
        self.selected_background = Some(color);
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn selected_text_color(mut self, color: Hsla) -> Self {
        self.selected_text_color = Some(color);
        self
    }
}

impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.15,
            a: 1.0,
        });
        let selected_bg = self.selected_background.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });
        let selected_text = self.selected_text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 1.0,
        });

        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let mut container = div()
            .flex()
            .items_center()
            .h(px(height))
            .rounded_lg()
            .overflow_hidden();

        // Apply variant-specific container styles
        container = match self.variant {
            ToggleGroupVariant::Default => container.bg(background).p(px(2.0)),
            ToggleGroupVariant::Outline => container
                .border_1()
                .border_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.3,
                    a: 1.0,
                }),
            ToggleGroupVariant::Ghost => container,
            ToggleGroupVariant::Pill => container.bg(background).rounded_full().p(px(2.0)),
        };

        if self.full_width {
            container = container.w_full();
        }

        if self.disabled {
            container = container.opacity(0.5);
        }

        for (i, item) in self.items.iter().enumerate() {
            let is_selected = self.selected.as_ref() == Some(&item.value);
            let is_disabled = self.disabled || item.disabled;

            let mut button = div()
                .flex()
                .items_center()
                .justify_center()
                .gap_1()
                .h_full()
                .px(px(padding))
                .text_size(px(font_size))
                .cursor_pointer();

            if self.full_width {
                button = button.flex_1();
            }

            // Apply selection styles
            if is_selected {
                button = match self.variant {
                    ToggleGroupVariant::Default | ToggleGroupVariant::Pill => {
                        button.bg(selected_bg).text_color(selected_text).rounded_md()
                    }
                    ToggleGroupVariant::Outline => button
                        .bg(selected_bg)
                        .text_color(selected_text),
                    ToggleGroupVariant::Ghost => button
                        .text_color(selected_bg)
                        .font_weight(gpui::FontWeight::SEMIBOLD),
                };
            } else {
                button = button.text_color(text_color);
                if !is_disabled {
                    button = button.hover(|s| {
                        s.bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.2,
                            a: 1.0,
                        })
                    });
                }
            }

            if is_disabled {
                button = button.cursor_not_allowed();
            }

            // Add separator for outline variant
            if matches!(self.variant, ToggleGroupVariant::Outline) && i > 0 {
                container = container.child(
                    div()
                        .w(px(1.0))
                        .h(px(height - 8.0))
                        .bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.3,
                            a: 1.0,
                        }),
                );
            }

            // Icon
            if let Some(icon) = &item.icon {
                button = button.child(
                    div()
                        .text_size(px(font_size - 2.0))
                        .child(icon.clone()),
                );
            }

            // Label
            button = button.child(item.label.clone());

            container = container.child(button);
        }

        container
    }
}
