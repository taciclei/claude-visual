//! Button group component (multiple buttons grouped together)

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Button group (multiple buttons grouped together)
#[derive(IntoElement)]
pub struct ButtonGroup {
    items: Vec<ButtonGroupItem>,
    size: ToggleGroupSize,
    variant: ButtonGroupVariant,
    vertical: bool,
    disabled: bool,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: ToggleGroupSize::Medium,
            variant: ButtonGroupVariant::Default,
            vertical: false,
            disabled: false,
        }
    }

    pub fn item(mut self, item: ButtonGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ButtonGroupItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ButtonGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let border_color = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        };

        let mut container = div().flex().overflow_hidden();

        if self.vertical {
            container = container.flex_col().rounded_lg();
        } else {
            container = container.flex_row().rounded_lg();
        }

        if self.disabled {
            container = container.opacity(0.5);
        }

        let item_count = self.items.len();

        for (i, item) in self.items.into_iter().enumerate() {
            let is_disabled = self.disabled || item.disabled;
            let is_first = i == 0;
            let is_last = i == item_count - 1;

            let mut button = div()
                .flex()
                .items_center()
                .justify_center()
                .gap_1()
                .h(px(height))
                .px(px(padding))
                .text_size(px(font_size))
                .cursor_pointer();

            // Apply variant styles
            button = match self.variant {
                ButtonGroupVariant::Default => {
                    let bg = if item.active {
                        Hsla {
                            h: 0.58,
                            s: 0.7,
                            l: 0.5,
                            a: 1.0,
                        }
                    } else {
                        Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.2,
                            a: 1.0,
                        }
                    };
                    let text = if item.active {
                        Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        }
                    } else {
                        Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.8,
                            a: 1.0,
                        }
                    };
                    button.bg(bg).text_color(text)
                }
                ButtonGroupVariant::Outline => {
                    let text = if item.active {
                        Hsla {
                            h: 0.58,
                            s: 0.7,
                            l: 0.6,
                            a: 1.0,
                        }
                    } else {
                        Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.7,
                            a: 1.0,
                        }
                    };
                    button
                        .border_1()
                        .border_color(border_color)
                        .text_color(text)
                }
                ButtonGroupVariant::Ghost => {
                    let text = if item.active {
                        Hsla {
                            h: 0.58,
                            s: 0.7,
                            l: 0.6,
                            a: 1.0,
                        }
                    } else {
                        Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.6,
                            a: 1.0,
                        }
                    };
                    button.text_color(text)
                }
            };

            // Apply rounded corners
            if self.vertical {
                if is_first {
                    button = button.rounded_t_lg();
                }
                if is_last {
                    button = button.rounded_b_lg();
                }
            } else {
                if is_first {
                    button = button.rounded_l_lg();
                }
                if is_last {
                    button = button.rounded_r_lg();
                }
            }

            if !is_disabled {
                button = button.hover(|s| {
                    s.bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.25,
                        a: 1.0,
                    })
                });
            } else {
                button = button.cursor_not_allowed();
            }

            if let Some(icon) = item.icon {
                button = button.child(div().text_size(px(font_size - 2.0)).child(icon));
            }

            button = button.child(item.label);

            container = container.child(button);
        }

        container
    }
}
