//! Main list component

use gpui::prelude::*;
use gpui::*;

use super::types::{ListItem, ListSize, ListStyle};

/// Simple list component
#[derive(Clone)]
pub struct List {
    items: Vec<ListItem>,
    size: ListSize,
    style: ListStyle,
    selected_index: Option<usize>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: ListSize::default(),
            style: ListStyle::default(),
            selected_index: None,
        }
    }

    pub fn items(mut self, items: Vec<ListItem>) -> Self {
        self.items = items;
        self
    }

    pub fn item(mut self, item: ListItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn size(mut self, size: ListSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: ListStyle) -> Self {
        self.style = style;
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let surface_selected = hsla(0.6, 0.8, 0.6, 0.15);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let (px_h, py_v) = self.size.item_padding();
        let gap = self.size.gap();
        let selected_idx = self.selected_index;
        let style = self.style;

        div().w_full().flex().flex_col().gap(px(gap)).children(
            self.items.into_iter().enumerate().map(move |(idx, item)| {
                let is_selected = selected_idx == Some(idx) || item.selected;
                let is_disabled = item.disabled;
                let is_clickable = item.clickable && !is_disabled;
                let is_striped = matches!(style, ListStyle::Striped) && idx % 2 == 1;

                let mut row = div()
                    .w_full()
                    .px(px(px_h))
                    .py(px(py_v))
                    .flex()
                    .items_center()
                    .gap_3();

                // Style-specific rendering
                match style {
                    ListStyle::Plain => {}
                    ListStyle::Separated => {
                        if idx > 0 {
                            row = row.border_t_1().border_color(border);
                        }
                    }
                    ListStyle::Card => {
                        row = row
                            .bg(surface)
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(border);
                    }
                    ListStyle::Striped => {
                        if is_striped {
                            row = row.bg(hsla(0.0, 0.0, 0.08, 1.0));
                        }
                    }
                }

                // Selection state
                if is_selected {
                    row = row
                        .bg(surface_selected)
                        .when(matches!(style, ListStyle::Card), |d| d.border_color(accent));
                }

                // Hover state
                if is_clickable {
                    row = row.cursor_pointer().hover(|s| s.bg(surface_hover));
                }

                // Disabled state
                if is_disabled {
                    row = row.opacity(0.5);
                }

                // Leading element
                if let Some(leading) = item.leading {
                    row = row.child(
                        div()
                            .w(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_lg()
                            .text_color(if is_selected { accent } else { text_muted })
                            .child(leading),
                    );
                }

                // Content
                row = row.child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .text_color(if is_selected { accent } else { text })
                                .child(item.primary),
                        )
                        .when_some(item.secondary, |d, sec| {
                            d.child(div().text_xs().text_color(text_muted).child(sec))
                        }),
                );

                // Trailing element
                if let Some(trailing) = item.trailing {
                    row = row.child(
                        div()
                            .flex_shrink_0()
                            .text_sm()
                            .text_color(text_muted)
                            .child(trailing),
                    );
                }

                row
            }),
        )
    }
}
