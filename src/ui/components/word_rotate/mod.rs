//! Word rotation components - Animated text switching
//!
//! Provides components for rotating through words/phrases with animations.

mod counting_number;
mod flip_words;
mod gradient_text;
mod text_scramble;
mod types;
mod typewriter;
mod word_rotate;

pub use counting_number::CountingNumber;
pub use flip_words::FlipWords;
pub use gradient_text::GradientText;
pub use text_scramble::TextScramble;
pub use types::*;
pub use typewriter::Typewriter;
pub use word_rotate::WordRotate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_rotate_animations() {
        let fade = WordRotate::new("f").animation(RotateAnimation::Fade);
        let slide = WordRotate::new("s").animation(RotateAnimation::SlideUp);
        let flip = WordRotate::new("fl").animation(RotateAnimation::Flip);

        assert_eq!(fade.animation, RotateAnimation::Fade);
        assert_eq!(slide.animation, RotateAnimation::SlideUp);
        assert_eq!(flip.animation, RotateAnimation::Flip);
    }

    #[test]
    fn test_word_rotate_speeds() {
        let slow = WordRotate::new("s").speed(RotateSpeed::Slow);
        let normal = WordRotate::new("n").speed(RotateSpeed::Normal);
        let fast = WordRotate::new("f").speed(RotateSpeed::Fast);

        assert_eq!(slow.speed, RotateSpeed::Slow);
        assert_eq!(normal.speed, RotateSpeed::Normal);
        assert_eq!(fast.speed, RotateSpeed::Fast);
    }

    #[test]
    fn test_typewriter() {
        let tw = Typewriter::new("tw", "Hello World")
            .visible_chars(5)
            .cursor_visible(true);

        assert_eq!(tw.text.as_ref(), "Hello World");
        assert_eq!(tw.visible_chars, 5);
        assert!(tw.cursor_visible);
    }

    #[test]
    fn test_text_scramble() {
        let scramble = TextScramble::new("sc", "Secret Message").revealed_count(6);

        assert_eq!(scramble.revealed_count, 6);
    }

    #[test]
    fn test_counting_number() {
        let num = CountingNumber::new("n", 1234.56)
            .decimals(2)
            .prefix("$")
            .suffix(" USD");

        assert_eq!(num.value, 1234.56);
        assert_eq!(num.decimals, 2);
        assert_eq!(num.prefix.as_ref(), "$");
        assert_eq!(num.suffix.as_ref(), " USD");
    }
}
