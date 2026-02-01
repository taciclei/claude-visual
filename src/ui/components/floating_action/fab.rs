//! Main floating action button component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Floating action button component
#[derive(IntoElement)]
pub struct Fab {
    id: ElementId,
    icon: SharedString,
    label: Option<SharedString>,
    size: FabSize,
    variant: FabVariant,
    disabled: bool,
    loading: bool,
}

impl Fab {
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: None,
            size: FabSize::default(),
            variant: FabVariant::default(),
            disabled: false,
            loading: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self.size = FabSize::Extended;
        self
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: FabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl RenderOnce for Fab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg_color, text_color) = self.variant.colors();

        let (width, height) = self.size.dimensions();
        let icon_size = self.size.icon_size();
        let icon = self.icon.clone();

        let opacity = if self.disabled { 0.5 } else { 1.0 };
        let icon_str = if self.loading {
            "⏳".into()
        } else {
            icon.clone()
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .when(self.size != FabSize::Extended, |el| el.w(px(width)))
            .when(self.size == FabSize::Extended, |el| el.px(px(16.0)))
            .h(px(height))
            .bg(bg_color)
            .rounded(px(height / 2.0))
            .opacity(opacity)
            .shadow_lg()
            .when(!self.disabled, |el| el.cursor_pointer())
            .child(
                div()
                    .text_size(px(icon_size))
                    .text_color(text_color)
                    .child(icon_str),
            )
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(label),
                )
            })
    }
}
