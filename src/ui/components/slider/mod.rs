//! Slider component for range input

mod range_slider;
mod render;
mod slider;
mod types;

pub use range_slider::RangeSlider;
pub use slider::Slider;
pub use types::{RangeSliderEvent, SliderEvent, SliderMark, SliderSize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_size() {
        assert_eq!(SliderSize::Small.thumb_size(), 12.0);
        assert_eq!(SliderSize::Medium.thumb_size(), 16.0);
        assert_eq!(SliderSize::Large.thumb_size(), 20.0);
    }

    #[test]
    fn test_slider_mark() {
        let mark = SliderMark::new(50.0).with_label("50%");
        assert_eq!(mark.value, 50.0);
        assert_eq!(mark.label, Some("50%".to_string()));
    }

    #[test]
    fn test_value_to_percent() {
        // Test percentage calculation
        let min = 0.0_f32;
        let max = 100.0_f32;
        let value = 50.0_f32;
        let percent = ((value - min) / (max - min)) * 100.0;
        assert_eq!(percent, 50.0);
    }
}
