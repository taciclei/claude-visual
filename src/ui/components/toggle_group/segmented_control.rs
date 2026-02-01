//! Segmented control (iOS-style toggle group)

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Segmented control (iOS-style toggle group)
#[derive(IntoElement)]
pub struct SegmentedControl {
    items: Vec<ToggleItem>,
    selected: Option<SharedString>,
    size: ToggleGroupSize,
    disabled: bool,
    background: Option<Hsla>,
    indicator_color: Option<Hsla>,
}

impl SegmentedControl {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            size: ToggleGroupSize::Medium,
            disabled: false,
            background: None,
            indicator_color: None,
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn indicator_color(mut self, color: Hsla) -> Self {
        self.indicator_color = Some(color);
        self
    }
}

impl Default for SegmentedControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.12,
            a: 1.0,
        });
        let indicator_color = self.indicator_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.22,
            a: 1.0,
        });

        let height = self.size.height();
        let font_size = self.size.font_size();

        let mut container = div()
            .flex()
            .items_center()
            .h(px(height))
            .bg(background)
            .rounded_lg()
            .p(px(2.0));

        if self.disabled {
            container = container.opacity(0.5);
        }

        for item in &self.items {
            let is_selected = self.selected.as_ref() == Some(&item.value);
            let is_disabled = self.disabled || item.disabled;

            let mut segment = div()
                .flex()
                .items_center()
                .justify_center()
                .gap_1()
                .flex_1()
                .h_full()
                .px_3()
                .text_size(px(font_size))
                .rounded_md()
                .cursor_pointer();

            if is_selected {
                segment = segment
                    .bg(indicator_color)
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.95,
                        a: 1.0,
                    })
                    .shadow_sm();
            } else {
                segment = segment.text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.6,
                    a: 1.0,
                });
                if !is_disabled {
                    segment = segment.hover(|s| {
                        s.text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.8,
                            a: 1.0,
                        })
                    });
                }
            }

            if is_disabled {
                segment = segment.cursor_not_allowed();
            }

            if let Some(icon) = &item.icon {
                segment = segment.child(div().text_size(px(font_size - 2.0)).child(icon.clone()));
            }

            segment = segment.child(item.label.clone());

            container = container.child(segment);
        }

        container
    }
}
