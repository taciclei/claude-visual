//! Icon toggle group component (icons only)

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Icon toggle group (icons only)
#[derive(IntoElement)]
pub struct IconToggleGroup {
    icons: Vec<IconToggleItem>,
    selected: Option<SharedString>,
    size: f32,
    gap: f32,
    background: Option<Hsla>,
    selected_background: Option<Hsla>,
}

impl IconToggleGroup {
    pub fn new() -> Self {
        Self {
            icons: Vec::new(),
            selected: None,
            size: 32.0,
            gap: 4.0,
            background: None,
            selected_background: None,
        }
    }

    pub fn icon(mut self, item: IconToggleItem) -> Self {
        self.icons.push(item);
        self
    }

    pub fn icons(mut self, items: impl IntoIterator<Item = IconToggleItem>) -> Self {
        self.icons.extend(items);
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
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
}

impl Default for IconToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for IconToggleGroup {
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

        let mut container = div()
            .flex()
            .items_center()
            .gap(px(self.gap))
            .bg(background)
            .rounded_lg()
            .p(px(4.0));

        for item in &self.icons {
            let is_selected = self.selected.as_ref() == Some(&item.value);

            let mut icon_button = div()
                .w(px(self.size))
                .h(px(self.size))
                .flex()
                .items_center()
                .justify_center()
                .rounded_md()
                .cursor_pointer()
                .text_size(px(self.size * 0.5));

            if is_selected {
                icon_button = icon_button.bg(selected_bg).text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 1.0,
                    a: 1.0,
                });
            } else {
                icon_button = icon_button.text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.6,
                    a: 1.0,
                });
                if !item.disabled {
                    icon_button = icon_button.hover(|s| {
                        s.bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.25,
                            a: 1.0,
                        })
                    });
                }
            }

            if item.disabled {
                icon_button = icon_button.opacity(0.5).cursor_not_allowed();
            }

            icon_button = icon_button.child(item.icon.clone());

            container = container.child(icon_button);
        }

        container
    }
}
