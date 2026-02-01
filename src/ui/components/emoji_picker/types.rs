//! Shared types for emoji picker components

use gpui::*;

/// Emoji category
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EmojiCategory {
    #[default]
    Recent,
    Smileys,
    People,
    Animals,
    Food,
    Travel,
    Activities,
    Objects,
    Symbols,
    Flags,
}

impl EmojiCategory {
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            Self::Recent => "🕐",
            Self::Smileys => "😀",
            Self::People => "👋",
            Self::Animals => "🐱",
            Self::Food => "🍔",
            Self::Travel => "✈️",
            Self::Activities => "⚽",
            Self::Objects => "💡",
            Self::Symbols => "❤️",
            Self::Flags => "🏳️",
        }
    }

    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Recent => "Recent",
            Self::Smileys => "Smileys & Emotion",
            Self::People => "People & Body",
            Self::Animals => "Animals & Nature",
            Self::Food => "Food & Drink",
            Self::Travel => "Travel & Places",
            Self::Activities => "Activities",
            Self::Objects => "Objects",
            Self::Symbols => "Symbols",
            Self::Flags => "Flags",
        }
    }

    pub(crate) fn all() -> Vec<Self> {
        vec![
            Self::Recent,
            Self::Smileys,
            Self::People,
            Self::Animals,
            Self::Food,
            Self::Travel,
            Self::Activities,
            Self::Objects,
            Self::Symbols,
            Self::Flags,
        ]
    }
}

/// Emoji item
#[derive(Clone)]
pub struct Emoji {
    pub emoji: SharedString,
    pub name: SharedString,
    pub keywords: Vec<SharedString>,
    pub category: EmojiCategory,
}

impl Emoji {
    pub fn new(
        emoji: impl Into<SharedString>,
        name: impl Into<SharedString>,
        category: EmojiCategory,
    ) -> Self {
        Self {
            emoji: emoji.into(),
            name: name.into(),
            keywords: Vec::new(),
            category,
        }
    }

    pub fn keywords(mut self, keywords: Vec<impl Into<SharedString>>) -> Self {
        self.keywords = keywords.into_iter().map(|k| k.into()).collect();
        self
    }
}

/// Emoji picker size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EmojiPickerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl EmojiPickerSize {
    pub(crate) fn emoji_size(&self) -> f32 {
        match self {
            Self::Sm => 24.0,
            Self::Md => 32.0,
            Self::Lg => 40.0,
        }
    }

    pub(crate) fn grid_cols(&self) -> usize {
        match self {
            Self::Sm => 6,
            Self::Md => 8,
            Self::Lg => 10,
        }
    }

    pub(crate) fn width(&self) -> f32 {
        match self {
            Self::Sm => 200.0,
            Self::Md => 320.0,
            Self::Lg => 440.0,
        }
    }
}
