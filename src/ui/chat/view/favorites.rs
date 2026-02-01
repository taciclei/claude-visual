//! Favorites-related methods for ChatView
//!
//! This module contains all favorites/saved prompts functionality:
//! - Toggle favorites panel visibility
//! - Save, use, and remove favorite prompts
//! - Sort favorites by usage count

use gpui::*;
use super::core::ChatView;
use super::types::{FavoritePrompt, NotificationType};

impl ChatView {
    // ==================== Favorite Prompts ====================

    /// Toggle favorites panel
    pub fn toggle_favorites_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.favorites_panel = !self.panels.favorites_panel;
        cx.notify();
    }

    /// Add current input as favorite
    pub fn save_input_as_favorite(&mut self, label: impl Into<String>, cx: &mut Context<Self>) {
        let text = self.input.read(cx).text().to_string();
        if !text.trim().is_empty() {
            self.favorite_prompts.push(FavoritePrompt::new(text, label));
            self.show_notification("Prompt saved to favorites".to_string(), NotificationType::Success, cx);
        }
    }

    /// Use a favorite prompt
    pub fn use_favorite(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(favorite) = self.favorite_prompts.iter_mut().find(|f| f.id == id) {
            favorite.usage_count += 1;
            let text = favorite.text.clone();
            self.input.update(cx, |input, cx| {
                input.set_text(text, cx);
            });
            self.panels.favorites_panel = false;
            cx.notify();
        }
    }

    /// Remove a favorite prompt
    pub fn remove_favorite(&mut self, id: &str, cx: &mut Context<Self>) {
        self.favorite_prompts.retain(|f| f.id != id);
        self.show_notification("Favorite removed".to_string(), NotificationType::Info, cx);
        cx.notify();
    }

    /// Get favorites sorted by usage
    pub fn favorites_by_usage(&self) -> Vec<&FavoritePrompt> {
        let mut favorites: Vec<_> = self.favorite_prompts.iter().collect();
        favorites.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        favorites
    }
}
