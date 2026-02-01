//! Shared types for avatar group components

use gpui::*;

/// Avatar group size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarGroupSize {
    /// Extra small (20px)
    XSmall,
    /// Small (24px)
    Small,
    /// Medium (32px, default)
    #[default]
    Medium,
    /// Large (40px)
    Large,
    /// Extra large (48px)
    XLarge,
}

impl AvatarGroupSize {
    pub(crate) fn avatar_size(&self) -> f32 {
        match self {
            AvatarGroupSize::XSmall => 20.0,
            AvatarGroupSize::Small => 24.0,
            AvatarGroupSize::Medium => 32.0,
            AvatarGroupSize::Large => 40.0,
            AvatarGroupSize::XLarge => 48.0,
        }
    }

    pub(crate) fn overlap(&self) -> f32 {
        self.avatar_size() * 0.3
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            AvatarGroupSize::XSmall => 8.0,
            AvatarGroupSize::Small => 10.0,
            AvatarGroupSize::Medium => 12.0,
            AvatarGroupSize::Large => 14.0,
            AvatarGroupSize::XLarge => 16.0,
        }
    }
}

/// Individual avatar in the group
#[derive(Clone)]
pub struct GroupAvatar {
    /// Display name
    pub name: String,
    /// Initials (computed if not provided)
    pub initials: Option<String>,
    /// Background color
    pub color: Option<Hsla>,
    /// Image URL (placeholder for future)
    pub image_url: Option<String>,
    /// Online status
    pub is_online: Option<bool>,
}

impl GroupAvatar {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            initials: None,
            color: None,
            image_url: None,
            is_online: None,
        }
    }

    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn online(mut self, is_online: bool) -> Self {
        self.is_online = Some(is_online);
        self
    }

    pub(crate) fn get_initials(&self) -> String {
        if let Some(initials) = &self.initials {
            return initials.clone();
        }

        // Compute from name
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    pub(crate) fn get_color(&self, index: usize) -> Hsla {
        if let Some(color) = self.color {
            return color;
        }

        // Generate color from index
        let colors = [
            hsla(0.6, 0.7, 0.55, 1.0),  // Blue
            hsla(0.38, 0.65, 0.45, 1.0), // Green
            hsla(0.0, 0.7, 0.55, 1.0),   // Red
            hsla(0.75, 0.6, 0.55, 1.0),  // Purple
            hsla(0.12, 0.8, 0.5, 1.0),   // Orange
            hsla(0.55, 0.7, 0.5, 1.0),   // Cyan
            hsla(0.92, 0.6, 0.55, 1.0),  // Pink
            hsla(0.45, 0.65, 0.45, 1.0), // Teal
        ];

        colors[index % colors.len()]
    }
}
