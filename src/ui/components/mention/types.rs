//! Mention types and enums

use gpui::*;

/// Mention variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MentionVariant {
    #[default]
    User,
    Channel,
    Team,
    Document,
    Link,
}

/// Mention size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MentionSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl MentionSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 16.0,
        }
    }

    pub(crate) fn padding_x(&self) -> f32 {
        match self {
            Self::Sm => 4.0,
            Self::Md => 6.0,
            Self::Lg => 8.0,
        }
    }

    pub(crate) fn padding_y(&self) -> f32 {
        match self {
            Self::Sm => 1.0,
            Self::Md => 2.0,
            Self::Lg => 4.0,
        }
    }
}

/// Mentionable user item - for mention dropdown
#[derive(Clone)]
pub struct MentionableUser {
    pub id: SharedString,
    pub name: SharedString,
    pub username: SharedString,
    pub avatar: Option<SharedString>,
    pub status: Option<SharedString>,
    pub is_online: bool,
}

impl MentionableUser {
    pub fn new(
        id: impl Into<SharedString>,
        name: impl Into<SharedString>,
        username: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            username: username.into(),
            avatar: None,
            status: None,
            is_online: false,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<SharedString>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    pub fn status(mut self, status: impl Into<SharedString>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn is_online(mut self, is_online: bool) -> Self {
        self.is_online = is_online;
        self
    }
}
