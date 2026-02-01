//! Tag/Chip component for labels and categories

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::types::*;
use crate::app::state::AppState;

/// Tag/Chip component
pub struct Tag {
    app_state: Arc<AppState>,
    /// Tag label
    label: String,
    /// Optional icon/emoji prefix
    icon: Option<String>,
    /// Color variant
    color: TagColor,
    /// Size
    size: TagSize,
    /// Whether tag is closable (shows X button)
    closable: bool,
    /// Whether tag is clickable
    clickable: bool,
    /// Whether tag is selected/active
    selected: bool,
    /// Whether to use outline style
    outline: bool,
}

impl Tag {
    pub fn new(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            label: label.into(),
            icon: None,
            color: TagColor::default(),
            size: TagSize::default(),
            closable: false,
            clickable: false,
            selected: false,
            outline: false,
        }
    }

    /// Create a closable tag
    pub fn closable(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut tag = Self::new(app_state, label, cx);
        tag.closable = true;
        tag
    }

    /// Create a clickable/selectable tag
    pub fn selectable(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut tag = Self::new(app_state, label, cx);
        tag.clickable = true;
        tag
    }

    /// Set the label
    pub fn set_label(&mut self, label: impl Into<String>, cx: &mut Context<Self>) {
        self.label = label.into();
        cx.notify();
    }

    /// Set the icon
    pub fn set_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.icon = icon;
        cx.notify();
    }

    /// Set the color
    pub fn set_color(&mut self, color: TagColor, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }

    /// Set the size
    pub fn set_size(&mut self, size: TagSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set closable
    pub fn set_closable(&mut self, closable: bool, cx: &mut Context<Self>) {
        self.closable = closable;
        cx.notify();
    }

    /// Set clickable
    pub fn set_clickable(&mut self, clickable: bool, cx: &mut Context<Self>) {
        self.clickable = clickable;
        cx.notify();
    }

    /// Set selected state
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        self.selected = selected;
        cx.notify();
    }

    /// Set outline style
    pub fn set_outline(&mut self, outline: bool, cx: &mut Context<Self>) {
        self.outline = outline;
        cx.notify();
    }

    /// Toggle selected state
    pub fn toggle_selected(&mut self, cx: &mut Context<Self>) {
        self.selected = !self.selected;
        cx.notify();
    }

    /// Get colors for the current variant
    fn get_colors(&self, theme: &crate::app::theme::Theme) -> (Hsla, Hsla) {
        let (bg, fg) = match self.color {
            TagColor::Default => (theme.colors.surface, theme.colors.text_muted),
            TagColor::Primary => (theme.colors.accent.opacity(0.15), theme.colors.accent),
            TagColor::Success => (theme.colors.success.opacity(0.15), theme.colors.success),
            TagColor::Warning => (theme.colors.warning.opacity(0.15), theme.colors.warning),
            TagColor::Error => (theme.colors.error.opacity(0.15), theme.colors.error),
            TagColor::Info => (
                hsla(210.0 / 360.0, 0.8, 0.5, 0.15),
                hsla(210.0 / 360.0, 0.8, 0.5, 1.0),
            ),
            TagColor::Purple => (
                hsla(270.0 / 360.0, 0.6, 0.5, 0.15),
                hsla(270.0 / 360.0, 0.6, 0.5, 1.0),
            ),
            TagColor::Pink => (
                hsla(330.0 / 360.0, 0.7, 0.6, 0.15),
                hsla(330.0 / 360.0, 0.7, 0.6, 1.0),
            ),
        };

        if self.selected {
            // Stronger colors when selected
            (fg.opacity(0.25), fg)
        } else {
            (bg, fg)
        }
    }
}

impl EventEmitter<TagEvent> for Tag {}

impl Render for Tag {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();
        let (bg_color, fg_color) = self.get_colors(theme);

        div()
            .id("tag")
            .h(px(height))
            .px(px(padding_x))
            .flex()
            .items_center()
            .gap_1()
            .rounded(px(height / 2.0))
            .when(!self.outline, |d| d.bg(bg_color))
            .when(self.outline, |d| {
                d.border_1().border_color(fg_color.opacity(0.5))
            })
            .text_color(fg_color)
            .text_size(px(font_size))
            .font_weight(FontWeight::MEDIUM)
            .when(self.clickable, |d| {
                d.cursor_pointer().hover(|s| s.bg(fg_color.opacity(0.2)))
            })
            .when(self.clickable, |d| {
                d.on_click(cx.listener(|this, _, _window, cx| {
                    this.toggle_selected(cx);
                    cx.emit(TagEvent::Clicked);
                }))
            })
            // Icon
            .when_some(self.icon.clone(), |d, icon| {
                d.child(div().text_size(px(font_size - 2.0)).child(icon))
            })
            // Label
            .child(self.label.clone())
            // Close button
            .when(self.closable, |d| {
                d.child(
                    div()
                        .id("tag-close")
                        .ml_1()
                        .size(px(font_size + 2.0))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size - 2.0))
                        .hover(|s| s.bg(fg_color.opacity(0.2)))
                        .cursor_pointer()
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(TagEvent::Closed);
                        }))
                        .child("×"),
                )
            })
    }
}
