//! Emoji data

use super::types::*;

/// Default emoji set
pub(super) fn default_emojis() -> Vec<Emoji> {
    vec![
        // Smileys
        Emoji::new("😀", "grinning face", EmojiCategory::Smileys),
        Emoji::new("😃", "grinning face with big eyes", EmojiCategory::Smileys),
        Emoji::new(
            "😄",
            "grinning face with smiling eyes",
            EmojiCategory::Smileys,
        ),
        Emoji::new("😁", "beaming face", EmojiCategory::Smileys),
        Emoji::new("😅", "grinning face with sweat", EmojiCategory::Smileys),
        Emoji::new("😂", "face with tears of joy", EmojiCategory::Smileys),
        Emoji::new(
            "🤣",
            "rolling on the floor laughing",
            EmojiCategory::Smileys,
        ),
        Emoji::new(
            "😊",
            "smiling face with smiling eyes",
            EmojiCategory::Smileys,
        ),
        Emoji::new("😇", "smiling face with halo", EmojiCategory::Smileys),
        Emoji::new("🙂", "slightly smiling face", EmojiCategory::Smileys),
        Emoji::new("😉", "winking face", EmojiCategory::Smileys),
        Emoji::new("😍", "smiling face with heart-eyes", EmojiCategory::Smileys),
        Emoji::new("🥰", "smiling face with hearts", EmojiCategory::Smileys),
        Emoji::new("😘", "face blowing a kiss", EmojiCategory::Smileys),
        Emoji::new("😋", "face savoring food", EmojiCategory::Smileys),
        Emoji::new("😎", "smiling face with sunglasses", EmojiCategory::Smileys),
        Emoji::new("🤓", "nerd face", EmojiCategory::Smileys),
        Emoji::new("🧐", "face with monocle", EmojiCategory::Smileys),
        Emoji::new("🤔", "thinking face", EmojiCategory::Smileys),
        Emoji::new("🤨", "face with raised eyebrow", EmojiCategory::Smileys),
        Emoji::new("😐", "neutral face", EmojiCategory::Smileys),
        Emoji::new("😑", "expressionless face", EmojiCategory::Smileys),
        Emoji::new("😶", "face without mouth", EmojiCategory::Smileys),
        Emoji::new("🙄", "face with rolling eyes", EmojiCategory::Smileys),
        // People
        Emoji::new("👋", "waving hand", EmojiCategory::People),
        Emoji::new("👍", "thumbs up", EmojiCategory::People),
        Emoji::new("👎", "thumbs down", EmojiCategory::People),
        Emoji::new("👏", "clapping hands", EmojiCategory::People),
        Emoji::new("🙌", "raising hands", EmojiCategory::People),
        Emoji::new("🤝", "handshake", EmojiCategory::People),
        Emoji::new("🙏", "folded hands", EmojiCategory::People),
        Emoji::new("💪", "flexed biceps", EmojiCategory::People),
        // Objects
        Emoji::new("💻", "laptop", EmojiCategory::Objects),
        Emoji::new("📱", "mobile phone", EmojiCategory::Objects),
        Emoji::new("💡", "light bulb", EmojiCategory::Objects),
        Emoji::new("🔧", "wrench", EmojiCategory::Objects),
        Emoji::new("🔨", "hammer", EmojiCategory::Objects),
        Emoji::new("📝", "memo", EmojiCategory::Objects),
        Emoji::new("📁", "file folder", EmojiCategory::Objects),
        Emoji::new("📂", "open file folder", EmojiCategory::Objects),
        // Symbols
        Emoji::new("❤️", "red heart", EmojiCategory::Symbols),
        Emoji::new("💔", "broken heart", EmojiCategory::Symbols),
        Emoji::new("✅", "check mark button", EmojiCategory::Symbols),
        Emoji::new("❌", "cross mark", EmojiCategory::Symbols),
        Emoji::new("⭐", "star", EmojiCategory::Symbols),
        Emoji::new("🔥", "fire", EmojiCategory::Symbols),
        Emoji::new("✨", "sparkles", EmojiCategory::Symbols),
        Emoji::new("💯", "hundred points", EmojiCategory::Symbols),
    ]
}
