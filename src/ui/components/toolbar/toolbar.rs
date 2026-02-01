//! Main toolbar component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Toolbar component
#[derive(IntoElement)]
pub struct Toolbar {
    id: ElementId,
    items: Vec<ToolbarItem>,
    variant: ToolbarVariant,
    size: ToolbarSize,
    position: ToolbarPosition,
    show_labels: bool,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl Toolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            variant: ToolbarVariant::default(),
            size: ToolbarSize::default(),
            position: ToolbarPosition::default(),
            show_labels: false,
            background: None,
            border_color: None,
        }
    }

    pub fn items(mut self, items: Vec<ToolbarItem>) -> Self {
        self.items = items;
        self
    }

    pub fn variant(mut self, variant: ToolbarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToolbarSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, position: ToolbarPosition) -> Self {
        self.position = position;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            ToolbarSize::Small => (32.0, 28.0, 12.0),
            ToolbarSize::Medium => (40.0, 36.0, 14.0),
            ToolbarSize::Large => (48.0, 44.0, 16.0),
        }
    }
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bar_height, button_size, icon_size) = self.get_size_styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.1, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.2, 1.0));

        let is_vertical = matches!(self.position, ToolbarPosition::Left | ToolbarPosition::Right);

        let mut toolbar = div().id(self.id);

        // Apply variant styles
        toolbar = match self.variant {
            ToolbarVariant::Default => {
                toolbar
                    .bg(bg)
                    .border_1()
                    .border_color(border)
            }
            ToolbarVariant::Floating => {
                toolbar
                    .bg(bg)
                    .rounded(px(8.0))
                    .shadow_lg()
            }
            ToolbarVariant::Attached => {
                toolbar.bg(bg)
            }
            ToolbarVariant::Minimal => {
                toolbar
            }
        };

        // Apply position-based layout
        toolbar = if is_vertical {
            toolbar
                .w(px(bar_height))
                .flex()
                .flex_col()
                .items_center()
                .py(px(4.0))
                .gap(px(2.0))
        } else {
            toolbar
                .h(px(bar_height))
                .flex()
                .items_center()
                .px(px(4.0))
                .gap(px(2.0))
        };

        toolbar.children(self.items.into_iter().map(|item| {
            let mut button = div()
                .size(px(button_size))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(2.0))
                .rounded(px(6.0))
                .cursor_pointer();

            if item.active {
                button = button.bg(hsla(0.6, 0.5, 0.4, 0.3));
            }

            if item.disabled {
                button = button.opacity(0.5).cursor_not_allowed();
            } else if !item.active {
                button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)));
            }

            button
                .child(
                    div()
                        .text_size(px(icon_size))
                        .text_color(if item.active {
                            hsla(0.6, 0.7, 0.6, 1.0)
                        } else {
                            hsla(0.0, 0.0, 0.7, 1.0)
                        })
                        .child(item.icon.clone())
                )
                .when(self.show_labels && item.label.is_some(), |el| {
                    el.child(
                        div()
                            .text_size(px(10.0))
                            .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                            .child(item.label.unwrap_or_default())
                    )
                })
        }))
    }
}
