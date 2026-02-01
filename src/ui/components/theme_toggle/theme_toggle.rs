//! Main theme toggle component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Theme toggle component
#[derive(IntoElement)]
pub struct ThemeToggle {
    id: ElementId,
    mode: ThemeMode,
    variant: ThemeToggleVariant,
    size: ThemeToggleSize,
    show_label: bool,
    include_system: bool,
}

impl ThemeToggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            mode: ThemeMode::default(),
            variant: ThemeToggleVariant::default(),
            size: ThemeToggleSize::default(),
            show_label: false,
            include_system: false,
        }
    }

    pub fn mode(mut self, mode: ThemeMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn variant(mut self, variant: ThemeToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ThemeToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn include_system(mut self, include: bool) -> Self {
        self.include_system = include;
        self
    }
}

impl RenderOnce for ThemeToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let icon_size = self.size.icon_size();

        match self.variant {
            ThemeToggleVariant::Switch => {
                let is_dark = self.mode == ThemeMode::Dark;
                let track_width = height * 1.8;
                let thumb_size = height - 8.0;
                let thumb_offset = if is_dark {
                    track_width - thumb_size - 4.0
                } else {
                    4.0
                };

                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .relative()
                            .w(px(track_width))
                            .h(px(height))
                            .bg(if is_dark {
                                hsla(0.7, 0.5, 0.3, 1.0)
                            } else {
                                hsla(0.15, 0.7, 0.6, 1.0)
                            })
                            .rounded_full()
                            .cursor_pointer()
                            .child(
                                div()
                                    .absolute()
                                    .top(px(4.0))
                                    .left(px(thumb_offset))
                                    .w(px(thumb_size))
                                    .h(px(thumb_size))
                                    .bg(hsla(0.0, 0.0, 1.0, 1.0))
                                    .rounded_full()
                                    .shadow_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(icon_size * 0.7))
                                            .child(self.mode.icon()),
                                    ),
                            ),
                    )
                    .when(self.show_label, |el| {
                        el.child(
                            div()
                                .text_size(px(13.0))
                                .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                .child(self.mode.label()),
                        )
                    })
            }
            ThemeToggleVariant::Button => div()
                .id(self.id)
                .flex()
                .items_center()
                .justify_center()
                .gap(px(6.0))
                .h(px(height))
                .px(px(if self.show_label { 12.0 } else { 0.0 }))
                .w(px(if self.show_label { 0.0 } else { height }))
                .when(!self.show_label, |el| el.w(px(height)))
                .bg(hsla(0.0, 0.0, 0.15, 1.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.25, 1.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .child(div().text_size(px(icon_size)).child(self.mode.icon()))
                .when(self.show_label, |el| {
                    el.child(
                        div()
                            .text_size(px(13.0))
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(self.mode.label()),
                    )
                }),
            ThemeToggleVariant::Segmented => {
                let modes: Vec<ThemeMode> = if self.include_system {
                    vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::System]
                } else {
                    vec![ThemeMode::Light, ThemeMode::Dark]
                };

                div()
                    .id(self.id)
                    .flex()
                    .gap(px(2.0))
                    .p(px(2.0))
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .rounded(px(8.0))
                    .children(modes.into_iter().map(|m| {
                        let is_selected = m == self.mode;
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(4.0))
                            .h(px(height - 4.0))
                            .px(px(12.0))
                            .bg(if is_selected {
                                hsla(0.0, 0.0, 0.2, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.0, 0.0)
                            })
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .child(div().text_size(px(icon_size)).child(m.icon()))
                            .when(self.show_label, |el| {
                                el.child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(if is_selected {
                                            hsla(0.0, 0.0, 0.95, 1.0)
                                        } else {
                                            hsla(0.0, 0.0, 0.6, 1.0)
                                        })
                                        .child(m.label()),
                                )
                            })
                    }))
            }
            ThemeToggleVariant::Dropdown => div()
                .id(self.id)
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .h(px(height))
                .px(px(12.0))
                .bg(hsla(0.0, 0.0, 0.12, 1.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(div().text_size(px(icon_size)).child(self.mode.icon()))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                .child(self.mode.label()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child("▼"),
                ),
        }
    }
}
