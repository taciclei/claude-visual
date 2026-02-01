//! Wrapper component that shows a badge on top of another element

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// A wrapper component that shows a badge on top of another element
pub struct BadgeWrapper {
    app_state: Arc<AppState>,
    /// Badge content (count or text)
    badge_content: Option<String>,
    /// Badge variant
    variant: BadgeVariant,
    /// Whether to show as dot
    dot: bool,
    /// Badge position
    position: BadgePosition,
    /// Whether to hide when zero/empty
    hide_zero: bool,
}

impl BadgeWrapper {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            badge_content: None,
            variant: BadgeVariant::Error, // Default to red for notifications
            dot: false,
            position: BadgePosition::TopRight,
            hide_zero: true,
        }
    }

    pub fn set_count(&mut self, count: u32, cx: &mut Context<Self>) {
        if count == 0 && self.hide_zero {
            self.badge_content = None;
        } else {
            self.badge_content = Some(if count > 99 {
                "99+".to_string()
            } else {
                count.to_string()
            });
        }
        cx.notify();
    }

    pub fn set_content(&mut self, content: Option<String>, cx: &mut Context<Self>) {
        self.badge_content = content;
        cx.notify();
    }

    pub fn set_dot(&mut self, show: bool, cx: &mut Context<Self>) {
        self.dot = show;
        if show {
            self.badge_content = Some(String::new());
        }
        cx.notify();
    }

    pub fn set_variant(&mut self, variant: BadgeVariant, cx: &mut Context<Self>) {
        self.variant = variant;
        cx.notify();
    }

    pub fn set_position(&mut self, position: BadgePosition, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }

    pub fn set_hide_zero(&mut self, hide: bool, cx: &mut Context<Self>) {
        self.hide_zero = hide;
        cx.notify();
    }

    /// Check if badge should be visible
    pub fn is_visible(&self) -> bool {
        self.badge_content.is_some()
    }
}

impl Render for BadgeWrapper {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Determine colors based on variant
        let bg_color = match self.variant {
            BadgeVariant::Default => theme.colors.surface,
            BadgeVariant::Primary => theme.colors.accent,
            BadgeVariant::Success => theme.colors.success,
            BadgeVariant::Warning => theme.colors.warning,
            BadgeVariant::Error => theme.colors.error,
            BadgeVariant::Outline => theme.colors.surface,
        };

        let text_color = match self.variant {
            BadgeVariant::Default | BadgeVariant::Outline => theme.colors.text_muted,
            BadgeVariant::Warning => gpui::black(),
            _ => gpui::white(),
        };

        let badge_size = if self.dot { 8.0 } else { 18.0 };

        div().id("badge-wrapper").relative().when_some(
            self.badge_content.clone(),
            |this, content| {
                this.child(
                    div()
                        .absolute()
                        .when(
                            matches!(
                                self.position,
                                BadgePosition::TopRight | BadgePosition::TopLeft
                            ),
                            |d| d.top(px(-4.0)),
                        )
                        .when(
                            matches!(
                                self.position,
                                BadgePosition::BottomRight | BadgePosition::BottomLeft
                            ),
                            |d| d.bottom(px(-4.0)),
                        )
                        .when(
                            matches!(
                                self.position,
                                BadgePosition::TopRight | BadgePosition::BottomRight
                            ),
                            |d| d.right(px(-4.0)),
                        )
                        .when(
                            matches!(
                                self.position,
                                BadgePosition::TopLeft | BadgePosition::BottomLeft
                            ),
                            |d| d.left(px(-4.0)),
                        )
                        .h(px(badge_size))
                        .min_w(px(badge_size))
                        .when(!self.dot && !content.is_empty(), |d| d.px(px(4.0)))
                        .rounded_full()
                        .bg(bg_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(text_color)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::MEDIUM)
                        .when(!self.dot && !content.is_empty(), |d| d.child(content)),
                )
            },
        )
    }
}
