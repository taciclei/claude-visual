//! Typewriter and text animation components
//!
//! Provides typing animations, character reveal effects, and animated text displays.

mod types;
mod typewriter_text;
mod typing_indicator;
mod animated_text;
mod character_reveal;
mod text_scramble;
mod word_reveal;

// Re-export all public types
pub use types::*;
pub use typewriter_text::TypewriterText;
pub use typing_indicator::TypingIndicator;
pub use animated_text::AnimatedText;
pub use character_reveal::CharacterReveal;
pub use text_scramble::TextScramble;
pub use word_reveal::WordReveal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typewriter_text() {
        let tw = TypewriterText::new("tw-1", "Hello, World!")
            .visible_chars(5)
            .typing_speed(100);
        assert_eq!(tw.visible_text(), "Hello");
        assert!(!tw.is_complete());
    }

    #[test]
    fn test_typewriter_complete() {
        let tw = TypewriterText::new("tw-2", "Hi").visible_chars(2);
        assert!(tw.is_complete());
    }

    #[test]
    fn test_typing_indicator() {
        let indicator = TypingIndicator::new("ti-1")
            .variant(TypingIndicatorVariant::Wave)
            .size(TypingIndicatorSize::Lg)
            .label("Someone is typing");
        assert_eq!(indicator.variant, TypingIndicatorVariant::Wave);
        assert_eq!(indicator.size, TypingIndicatorSize::Lg);
    }

    #[test]
    fn test_animated_text() {
        let anim = AnimatedText::new("anim-1", "Animated!")
            .effect(TextEffect::SlideUp)
            .duration(1000)
            .stagger(100);
        assert_eq!(anim.effect, TextEffect::SlideUp);
        assert_eq!(anim.duration, 1000);
    }

    #[test]
    fn test_character_reveal() {
        let reveal = CharacterReveal::new("cr-1", "Secret")
            .revealed_count(3)
            .reveal_style(RevealStyle::Glow);
        assert!(!reveal.is_complete());
        assert_eq!(reveal.progress(), 0.5);
    }

    #[test]
    fn test_text_scramble() {
        let scramble = TextScramble::new("ts-1", "Target")
            .progress(0.5)
            .current_text("T@rg#t");
        assert!(!scramble.is_complete());
    }

    #[test]
    fn test_word_reveal() {
        let wr = WordReveal::new("wr-1", "Hello World Test").revealed_words(2);
        assert!(!wr.is_complete());
        assert_eq!(wr.words.len(), 3);
    }
}
