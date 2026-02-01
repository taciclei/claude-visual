//! Main chip/tag component

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::types::*;
use crate::app::state::AppState;

/// Single chip/tag component
pub struct Chip {
    app_state: Arc<AppState>,
    /// Chip label
    label: String,
    /// Optional leading icon
    icon: Option<String>,
    /// Optional avatar/image text
    avatar: Option<String>,
    /// Size variant
    size: ChipSize,
    /// Style variant
    variant: ChipVariant,
    /// Whether chip is deletable
    deletable: bool,
    /// Whether chip is selected
    selected: bool,
    /// Whether chip is disabled
    disabled: bool,
    /// Custom color (HSLA)
    color: Option<Hsla>,
}

impl Chip {
    pub fn new(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            label: label.into(),
            icon: None,
            avatar: None,
            size: ChipSize::default(),
            variant: ChipVariant::default(),
            deletable: false,
            selected: false,
            disabled: false,
            color: None,
        }
    }

    /// Set label
    pub fn set_label(&mut self, label: impl Into<String>, cx: &mut Context<Self>) {
        self.label = label.into();
        cx.notify();
    }

    /// Set icon
    pub fn set_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.icon = icon;
        cx.notify();
    }

    /// Set avatar
    pub fn set_avatar(&mut self, avatar: Option<String>, cx: &mut Context<Self>) {
        self.avatar = avatar;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: ChipSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set variant
    pub fn set_variant(&mut self, variant: ChipVariant, cx: &mut Context<Self>) {
        self.variant = variant;
        cx.notify();
    }

    /// Set deletable
    pub fn set_deletable(&mut self, deletable: bool, cx: &mut Context<Self>) {
        self.deletable = deletable;
        cx.notify();
    }

    /// Set selected
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        self.selected = selected;
        cx.notify();
    }

    /// Toggle selected
    pub fn toggle_selected(&mut self, cx: &mut Context<Self>) {
        self.selected = !self.selected;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set color
    pub fn set_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }
}

impl EventEmitter<ChipEvent> for Chip {}

impl Render for Chip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let base_color = self.color.unwrap_or(theme.colors.accent);
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        let (bg_color, border_color, text_color) = match self.variant {
            ChipVariant::Filled => {
                if self.selected {
                    (base_color, base_color, white())
                } else {
                    (
                        theme.colors.surface_hover,
                        theme.colors.border,
                        theme.colors.text,
                    )
                }
            }
            ChipVariant::Outlined => {
                if self.selected {
                    (base_color.opacity(0.1), base_color, base_color)
                } else {
                    (
                        theme.colors.surface.opacity(0.0),
                        theme.colors.border,
                        theme.colors.text,
                    )
                }
            }
            ChipVariant::Soft => {
                if self.selected {
                    (base_color.opacity(0.2), base_color.opacity(0.0), base_color)
                } else {
                    (
                        theme.colors.surface_hover.opacity(0.5),
                        theme.colors.surface_hover.opacity(0.0),
                        theme.colors.text_muted,
                    )
                }
            }
        };

        div()
            .id("chip")
            .h(px(height))
            .px(px(padding))
            .rounded_full()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .opacity(opacity)
            .flex()
            .items_center()
            .gap(px(6.0))
            .when(!self.disabled, |d| {
                d.cursor_pointer()
                    .hover(|s| s.opacity(0.9))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        cx.emit(ChipEvent::Clicked);
                    }))
            })
            // Avatar
            .when_some(self.avatar.clone(), |d, avatar| {
                d.child(
                    div()
                        .size(px(height - 8.0))
                        .rounded_full()
                        .bg(base_color.opacity(0.2))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size - 2.0))
                        .ml(px(-padding + 4.0))
                        .child(avatar),
                )
            })
            // Icon
            .when_some(self.icon.clone(), |d, icon| {
                d.child(
                    div()
                        .text_size(px(font_size))
                        .text_color(text_color)
                        .child(icon),
                )
            })
            // Label
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(text_color)
                    .child(self.label.clone()),
            )
            // Delete button
            .when(self.deletable, |d| {
                d.child(
                    div()
                        .id("chip-delete")
                        .size(px(height - 12.0))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size - 2.0))
                        .text_color(text_color.opacity(0.7))
                        .mr(px(-padding + 6.0))
                        .hover(|s| s.bg(theme.colors.surface_hover))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            cx.emit(ChipEvent::Deleted);
                        }))
                        .child("×"),
                )
            })
    }
}
