//! Rating component for star ratings and feedback

mod rating;
mod reaction_picker;
mod star_rating;
mod thumbs_feedback;
mod types;

pub use rating::Rating;
pub use reaction_picker::ReactionPicker;
pub use star_rating::StarRating;
pub use thumbs_feedback::ThumbsFeedback;
pub use types::{RatingEvent, RatingSize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_rating() {
        let rating = StarRating::new(3.5, 5).size(RatingSize::Large);

        assert!((rating.value - 3.5).abs() < f32::EPSILON);
        assert_eq!(rating.max, 5);
    }

    #[test]
    fn test_thumbs_feedback() {
        let up = ThumbsFeedback::up();
        assert_eq!(up.value, Some(true));

        let down = ThumbsFeedback::down();
        assert_eq!(down.value, Some(false));
    }

    #[test]
    fn test_reaction_picker() {
        let picker = ReactionPicker::new()
            .reaction("👍", 5)
            .reaction("❤️", 3)
            .selected(vec!["👍"]);

        assert_eq!(picker.reactions.len(), 2);
        assert_eq!(picker.selected, vec!["👍"]);
    }
}
