//! Toggle components - Simple toggle buttons
//!
//! Provides toggle button components that can be pressed/unpressed.

mod bookmark_toggle;
mod favorite_toggle;
mod icon_toggle;
mod like_toggle;
mod text_style_toggle;
mod toggle;
mod types;

pub use bookmark_toggle::BookmarkToggle;
pub use favorite_toggle::FavoriteToggle;
pub use icon_toggle::IconToggle;
pub use like_toggle::LikeToggle;
pub use text_style_toggle::TextStyleToggle;
pub use toggle::Toggle;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_pressed() {
        let unpressed = Toggle::new("t1").pressed(false);
        let pressed = Toggle::new("t2").pressed(true);

        assert!(!unpressed.pressed);
        assert!(pressed.pressed);
    }

    #[test]
    fn test_toggle_sizes() {
        let small = Toggle::new("s").size(ToggleSize::Small);
        let medium = Toggle::new("m").size(ToggleSize::Medium);
        let large = Toggle::new("l").size(ToggleSize::Large);

        assert_eq!(small.size, ToggleSize::Small);
        assert_eq!(medium.size, ToggleSize::Medium);
        assert_eq!(large.size, ToggleSize::Large);
    }

    #[test]
    fn test_toggle_variants() {
        let default = Toggle::new("d").variant(ToggleVariant::Default);
        let outline = Toggle::new("o").variant(ToggleVariant::Outline);
        let ghost = Toggle::new("g").variant(ToggleVariant::Ghost);

        assert_eq!(default.variant, ToggleVariant::Default);
        assert_eq!(outline.variant, ToggleVariant::Outline);
        assert_eq!(ghost.variant, ToggleVariant::Ghost);
    }

    #[test]
    fn test_text_style_icons() {
        let bold = TextStyleToggle::new("b", TextStyleType::Bold);
        let italic = TextStyleToggle::new("i", TextStyleType::Italic);

        assert_eq!(bold.get_icon(), "𝐁");
        assert_eq!(italic.get_icon(), "𝐼");
    }

    #[test]
    fn test_favorite_toggle() {
        let unfavorited = FavoriteToggle::new("f1").favorited(false);
        let favorited = FavoriteToggle::new("f2").favorited(true).count(42);

        assert!(!unfavorited.favorited);
        assert!(favorited.favorited);
        assert_eq!(favorited.count, Some(42));
    }

    #[test]
    fn test_like_toggle() {
        let unliked = LikeToggle::new("l1").liked(false);
        let liked = LikeToggle::new("l2").liked(true).count(100);

        assert!(!unliked.liked);
        assert!(liked.liked);
        assert_eq!(liked.count, Some(100));
    }
}
