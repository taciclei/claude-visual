//! Mini FAB - smaller floating action button

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Mini FAB - smaller floating action button
#[derive(IntoElement)]
pub struct MiniFab {
    id: ElementId,
    icon: SharedString,
    variant: FabVariant,
    disabled: bool,
}

impl MiniFab {
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            variant: FabVariant::default(),
            disabled: false,
        }
    }

    pub fn variant(mut self, variant: FabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for MiniFab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg_color, text_color) = self.variant.colors();

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .w(px(40.0))
            .h(px(40.0))
            .bg(bg_color)
            .rounded_full()
            .shadow_md()
            .opacity(opacity)
            .when(!self.disabled, |el| el.cursor_pointer())
            .child(
                div()
                    .text_size(px(18.0))
                    .text_color(text_color)
                    .child(self.icon.clone())
            )
    }
}
