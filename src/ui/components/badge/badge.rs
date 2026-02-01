//! Badge component for counts and status indicators

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Badge component for displaying counts, status, or labels
pub struct Badge {
    app_state: Arc<AppState>,
    /// Content to display
    content: String,
    /// Style variant
    variant: BadgeVariant,
    /// Size
    size: BadgeSize,
    /// Whether to show as a dot (no content)
    dot: bool,
    /// Maximum count (displays "99+" if exceeded)
    max_count: Option<u32>,
    /// Whether badge is pulsing/animated
    pulse: bool,
}

impl Badge {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            content: String::new(),
            variant: BadgeVariant::default(),
            size: BadgeSize::default(),
            dot: false,
            max_count: None,
            pulse: false,
        }
    }

    /// Create a badge with text content
    pub fn with_text(
        app_state: Arc<AppState>,
        text: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut badge = Self::new(app_state, cx);
        badge.content = text.into();
        badge
    }

    /// Create a badge with a count
    pub fn with_count(app_state: Arc<AppState>, count: u32, cx: &mut Context<Self>) -> Self {
        let mut badge = Self::new(app_state, cx);
        badge.content = count.to_string();
        badge.max_count = Some(99);
        badge
    }

    /// Create a dot badge (no content)
    pub fn dot(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut badge = Self::new(app_state, cx);
        badge.dot = true;
        badge.size = BadgeSize::XSmall;
        badge
    }

    /// Set the content
    pub fn set_content(&mut self, content: impl Into<String>, cx: &mut Context<Self>) {
        self.content = content.into();
        self.dot = false;
        cx.notify();
    }

    /// Set the count
    pub fn set_count(&mut self, count: u32, cx: &mut Context<Self>) {
        self.content = if let Some(max) = self.max_count {
            if count > max {
                format!("{}+", max)
            } else {
                count.to_string()
            }
        } else {
            count.to_string()
        };
        self.dot = false;
        cx.notify();
    }

    /// Set the variant
    pub fn set_variant(&mut self, variant: BadgeVariant, cx: &mut Context<Self>) {
        self.variant = variant;
        cx.notify();
    }

    /// Set the size
    pub fn set_size(&mut self, size: BadgeSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set as dot badge
    pub fn set_dot(&mut self, dot: bool, cx: &mut Context<Self>) {
        self.dot = dot;
        if dot {
            self.size = BadgeSize::XSmall;
        }
        cx.notify();
    }

    /// Set maximum count
    pub fn set_max_count(&mut self, max: Option<u32>, cx: &mut Context<Self>) {
        self.max_count = max;
        cx.notify();
    }

    /// Set pulse animation
    pub fn set_pulse(&mut self, pulse: bool, cx: &mut Context<Self>) {
        self.pulse = pulse;
        cx.notify();
    }

    /// Get display content
    fn display_content(&self) -> String {
        if self.dot {
            return String::new();
        }

        if let Some(max) = self.max_count {
            if let Ok(count) = self.content.parse::<u32>() {
                if count > max {
                    return format!("{}+", max);
                }
            }
        }

        self.content.clone()
    }
}

impl Render for Badge {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        // Determine colors based on variant
        let (bg_color, text_color, border_color) = match self.variant {
            BadgeVariant::Default => (
                theme.colors.surface,
                theme.colors.text_muted,
                Some(theme.colors.border),
            ),
            BadgeVariant::Primary => (theme.colors.accent, gpui::white(), None),
            BadgeVariant::Success => (theme.colors.success, gpui::white(), None),
            BadgeVariant::Warning => (theme.colors.warning, gpui::black(), None),
            BadgeVariant::Error => (theme.colors.error, gpui::white(), None),
            BadgeVariant::Outline => (
                gpui::transparent_black(),
                theme.colors.text_muted,
                Some(theme.colors.border),
            ),
        };

        let content = self.display_content();
        let min_width = if self.dot {
            height
        } else {
            height.max(content.len() as f32 * font_size * 0.6 + padding * 2.0)
        };

        div()
            .id("badge")
            .h(px(height))
            .min_w(px(min_width))
            .when(!self.dot, |d| d.px(px(padding)))
            .when(self.dot, |d| d.w(px(height)))
            .rounded_full()
            .bg(bg_color)
            .when_some(border_color, |d, color| d.border_1().border_color(color))
            .flex()
            .items_center()
            .justify_center()
            .text_color(text_color)
            .text_size(px(font_size))
            .font_weight(FontWeight::MEDIUM)
            .when(!self.dot, |d| d.child(content))
    }
}
