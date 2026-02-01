//! Appearance settings panel component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::theme_preview::ThemePreview;

/// Appearance settings panel
#[derive(IntoElement)]
pub struct AppearanceSettings {
    id: ElementId,
    current_mode: ThemeMode,
    show_accent_color: bool,
    accent_colors: Vec<Hsla>,
    selected_accent: usize,
}

impl AppearanceSettings {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            current_mode: ThemeMode::default(),
            show_accent_color: true,
            accent_colors: vec![
                hsla(0.6, 0.7, 0.5, 1.0),  // Blue
                hsla(0.8, 0.7, 0.5, 1.0),  // Purple
                hsla(0.35, 0.7, 0.45, 1.0), // Green
                hsla(0.08, 0.8, 0.5, 1.0), // Orange
                hsla(0.95, 0.7, 0.5, 1.0), // Pink
            ],
            selected_accent: 0,
        }
    }

    pub fn current_mode(mut self, mode: ThemeMode) -> Self {
        self.current_mode = mode;
        self
    }

    pub fn show_accent_color(mut self, show: bool) -> Self {
        self.show_accent_color = show;
        self
    }

    pub fn accent_colors(mut self, colors: Vec<Hsla>) -> Self {
        self.accent_colors = colors;
        self
    }

    pub fn selected_accent(mut self, index: usize) -> Self {
        self.selected_accent = index;
        self
    }
}

impl RenderOnce for AppearanceSettings {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(24.0))
            // Theme selection
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .child("Theme")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .child(ThemePreview::new("light", ThemeMode::Light)
                                .selected(self.current_mode == ThemeMode::Light))
                            .child(ThemePreview::new("dark", ThemeMode::Dark)
                                .selected(self.current_mode == ThemeMode::Dark))
                            .child(ThemePreview::new("system", ThemeMode::System)
                                .selected(self.current_mode == ThemeMode::System))
                    )
            )
            // Accent color
            .when(self.show_accent_color, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                                .child("Accent Color")
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(8.0))
                                .children(self.accent_colors.iter().enumerate().map(|(i, &color)| {
                                    let is_selected = i == self.selected_accent;
                                    div()
                                        .w(px(32.0))
                                        .h(px(32.0))
                                        .bg(color)
                                        .rounded_full()
                                        .cursor_pointer()
                                        .when(is_selected, |el| {
                                            el.border_2()
                                                .border_color(hsla(0.0, 0.0, 1.0, 1.0))
                                        })
                                }))
                        )
                )
            })
    }
}
