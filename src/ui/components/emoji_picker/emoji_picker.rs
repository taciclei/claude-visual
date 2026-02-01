//! Main emoji picker component

use gpui::prelude::*;
use gpui::*;

use super::data::default_emojis;
use super::types::*;

/// Emoji picker component
#[derive(IntoElement)]
pub struct EmojiPicker {
    pub(super) id: ElementId,
    pub(super) emojis: Vec<Emoji>,
    pub(super) recent: Vec<SharedString>,
    pub(crate) selected_category: EmojiCategory,
    pub(super) search_query: SharedString,
    pub(super) size: EmojiPickerSize,
    pub(super) show_search: bool,
    pub(crate) show_preview: bool,
    pub(super) show_categories: bool,
}

impl EmojiPicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            emojis: default_emojis(),
            recent: Vec::new(),
            selected_category: EmojiCategory::default(),
            search_query: "".into(),
            size: EmojiPickerSize::default(),
            show_search: true,
            show_preview: true,
            show_categories: true,
        }
    }

    pub fn emojis(mut self, emojis: Vec<Emoji>) -> Self {
        self.emojis = emojis;
        self
    }

    pub fn recent(mut self, recent: Vec<impl Into<SharedString>>) -> Self {
        self.recent = recent.into_iter().map(|e| e.into()).collect();
        self
    }

    pub fn selected_category(mut self, category: EmojiCategory) -> Self {
        self.selected_category = category;
        self
    }

    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = query.into();
        self
    }

    pub fn size(mut self, size: EmojiPickerSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_search(mut self, show: bool) -> Self {
        self.show_search = show;
        self
    }

    pub fn show_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    pub fn show_categories(mut self, show: bool) -> Self {
        self.show_categories = show;
        self
    }
}
