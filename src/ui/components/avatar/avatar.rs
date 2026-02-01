//! Main avatar component

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// Avatar component
pub struct Avatar {
    app_state: Arc<AppState>,
    /// Content to display
    content: AvatarContent,
    /// Size variant
    size: AvatarSize,
    /// Shape
    shape: AvatarShape,
    /// Role (for default styling)
    role: AvatarRole,
    /// Custom background color
    bg_color: Option<Hsla>,
    /// Custom text/icon color
    fg_color: Option<Hsla>,
    /// Presence status indicator
    presence: PresenceStatus,
    /// Whether to show a border
    bordered: bool,
    /// Name for tooltip/accessibility
    name: Option<String>,
}

impl Avatar {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            content: AvatarContent::Default,
            size: AvatarSize::default(),
            shape: AvatarShape::default(),
            role: AvatarRole::default(),
            bg_color: None,
            fg_color: None,
            presence: PresenceStatus::None,
            bordered: false,
            name: None,
        }
    }

    /// Create an avatar for a user
    pub fn user(app_state: Arc<AppState>, name: Option<String>, cx: &mut Context<Self>) -> Self {
        let mut avatar = Self::new(app_state, cx);
        avatar.role = AvatarRole::User;
        avatar.name = name.clone();
        if let Some(n) = name {
            avatar.content = AvatarContent::Initials(Self::extract_initials(&n));
        }
        avatar
    }

    /// Create an avatar for the assistant
    pub fn assistant(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut avatar = Self::new(app_state, cx);
        avatar.role = AvatarRole::Assistant;
        avatar.content = AvatarContent::Icon("🤖".to_string());
        avatar.name = Some("Claude".to_string());
        avatar
    }

    /// Create an avatar for system messages
    pub fn system(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut avatar = Self::new(app_state, cx);
        avatar.role = AvatarRole::System;
        avatar.content = AvatarContent::Icon("⚙️".to_string());
        avatar.name = Some("System".to_string());
        avatar
    }

    /// Extract initials from a name
    pub(crate) fn extract_initials(name: &str) -> String {
        let parts: Vec<&str> = name.split_whitespace().collect();
        match parts.len() {
            0 => "?".to_string(),
            1 => parts[0].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default(),
            _ => {
                let first = parts[0].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                let last = parts.last().and_then(|s| s.chars().next()).map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                format!("{}{}", first, last)
            }
        }
    }

    /// Set the content
    pub fn set_content(&mut self, content: AvatarContent, cx: &mut Context<Self>) {
        self.content = content;
        cx.notify();
    }

    /// Set initials from a name
    pub fn set_name(&mut self, name: impl Into<String>, cx: &mut Context<Self>) {
        let name = name.into();
        self.content = AvatarContent::Initials(Self::extract_initials(&name));
        self.name = Some(name);
        cx.notify();
    }

    /// Set an icon
    pub fn set_icon(&mut self, icon: impl Into<String>, cx: &mut Context<Self>) {
        self.content = AvatarContent::Icon(icon.into());
        cx.notify();
    }

    /// Set the size
    pub fn set_size(&mut self, size: AvatarSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set the shape
    pub fn set_shape(&mut self, shape: AvatarShape, cx: &mut Context<Self>) {
        self.shape = shape;
        cx.notify();
    }

    /// Set the role
    pub fn set_role(&mut self, role: AvatarRole, cx: &mut Context<Self>) {
        self.role = role;
        cx.notify();
    }

    /// Set custom background color
    pub fn set_bg_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.bg_color = color;
        cx.notify();
    }

    /// Set custom foreground color
    pub fn set_fg_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.fg_color = color;
        cx.notify();
    }

    /// Set presence status
    pub fn set_presence(&mut self, status: PresenceStatus, cx: &mut Context<Self>) {
        self.presence = status;
        cx.notify();
    }

    /// Enable/disable border
    pub fn set_bordered(&mut self, bordered: bool, cx: &mut Context<Self>) {
        self.bordered = bordered;
        cx.notify();
    }

    /// Get default background color for role
    fn default_bg_for_role(&self, theme: &crate::app::theme::Theme) -> Hsla {
        match self.role {
            AvatarRole::User => theme.colors.accent.opacity(0.2),
            AvatarRole::Assistant => theme.colors.success.opacity(0.2),
            AvatarRole::System => theme.colors.text_muted.opacity(0.2),
            AvatarRole::Team => theme.colors.warning.opacity(0.2),
            AvatarRole::Bot => theme.colors.text_muted.opacity(0.2),
        }
    }

    /// Get default foreground color for role
    fn default_fg_for_role(&self, theme: &crate::app::theme::Theme) -> Hsla {
        match self.role {
            AvatarRole::User => theme.colors.accent,
            AvatarRole::Assistant => theme.colors.success,
            AvatarRole::System => theme.colors.text_muted,
            AvatarRole::Team => theme.colors.warning,
            AvatarRole::Bot => theme.colors.text_muted,
        }
    }

    /// Get presence indicator color
    fn presence_color(&self, theme: &crate::app::theme::Theme) -> Option<Hsla> {
        match self.presence {
            PresenceStatus::None => None,
            PresenceStatus::Online => Some(theme.colors.success),
            PresenceStatus::Away => Some(theme.colors.warning),
            PresenceStatus::Busy => Some(theme.colors.error),
            PresenceStatus::Offline => Some(theme.colors.text_muted.opacity(0.5)),
        }
    }
}

impl Render for Avatar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let size = self.size.pixels();
        let font_size = self.size.font_size();

        let bg_color = self.bg_color.unwrap_or_else(|| self.default_bg_for_role(theme));
        let fg_color = self.fg_color.unwrap_or_else(|| self.default_fg_for_role(theme));
        let presence_color = self.presence_color(theme);

        // Calculate border radius
        let radius = match self.shape {
            AvatarShape::Circle => size / 2.0,
            AvatarShape::Rounded => 6.0,
            AvatarShape::Square => 0.0,
        };

        // Get content to display
        let display_content: String = match &self.content {
            AvatarContent::Initials(s) => s.clone(),
            AvatarContent::Icon(s) => s.clone(),
            AvatarContent::Image(_) => "".to_string(), // Would use actual image
            AvatarContent::Default => self.role.default_icon().to_string(),
        };

        div()
            .id("avatar")
            .relative()
            .size(px(size))
            .flex_shrink_0()
            // Avatar circle/shape
            .child(
                div()
                    .size(px(size))
                    .rounded(px(radius))
                    .bg(bg_color)
                    .when(self.bordered, |d| d.border_2().border_color(theme.colors.border))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(fg_color)
                    .text_size(px(font_size))
                    .font_weight(FontWeight::MEDIUM)
                    .child(display_content)
            )
            // Presence indicator
            .when_some(presence_color, |this, color| {
                let indicator_size = (size * 0.3).max(8.0);
                this.child(
                    div()
                        .absolute()
                        .right_0()
                        .bottom_0()
                        .size(px(indicator_size))
                        .rounded_full()
                        .bg(color)
                        .border_2()
                        .border_color(theme.colors.surface)
                )
            })
    }
}
