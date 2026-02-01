//! Emoji picker components
//!
//! Provides emoji selection interface with categories and search.

mod types;
mod data;
mod emoji_picker;
mod render;
mod emoji_button;
mod reaction_picker;
mod emoji_reaction;

// Re-export types
pub use types::{EmojiCategory, Emoji, EmojiPickerSize};

// Re-export components
pub use emoji_picker::EmojiPicker;
pub use emoji_button::EmojiButton;
pub use reaction_picker::ReactionPicker;
pub use emoji_reaction::EmojiReaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_categories() {
        let all = EmojiCategory::all();
        assert_eq!(all.len(), 10);
        assert!(all.contains(&EmojiCategory::Smileys));
    }

    #[test]
    fn test_emoji_picker_sizes() {
        let sm = EmojiPickerSize::Sm;
        let lg = EmojiPickerSize::Lg;

        assert!(sm.width() < lg.width());
        assert!(sm.emoji_size() < lg.emoji_size());
    }

    #[test]
    fn test_emoji_picker() {
        let picker = EmojiPicker::new("ep")
            .selected_category(EmojiCategory::Smileys)
            .search_query("smile")
            .show_preview(true);

        assert_eq!(picker.selected_category, EmojiCategory::Smileys);
        assert!(picker.show_preview);
    }

    #[test]
    fn test_emoji() {
        let emoji = Emoji::new("😀", "grinning face", EmojiCategory::Smileys)
            .keywords(vec!["happy", "smile"]);

        assert_eq!(emoji.emoji.as_ref(), "😀");
        assert_eq!(emoji.keywords.len(), 2);
    }

    #[test]
    fn test_reaction_picker() {
        let picker = ReactionPicker::new("rp")
            .quick_reactions(vec!["👍", "👎", "❤️"])
            .show_more(false);

        assert_eq!(picker.quick_reactions.len(), 3);
        assert!(!picker.show_more);
    }

    #[test]
    fn test_emoji_reaction() {
        let reaction = EmojiReaction::new("r", "👍", 5)
            .is_active(true);

        assert_eq!(reaction.count, 5);
        assert!(reaction.is_active);
    }
}
