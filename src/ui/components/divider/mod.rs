//! Divider component for visual separation

mod types;
mod divider;
mod horizontal_rule;
mod vertical_rule;

pub use types::*;
pub use divider::Divider;
pub use horizontal_rule::HorizontalRule;
pub use vertical_rule::VerticalRule;

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::*;

    #[test]
    fn test_thickness_pixels() {
        assert_eq!(DividerThickness::Thin.pixels(), 1.0);
        assert_eq!(DividerThickness::Normal.pixels(), 1.0);
        assert_eq!(DividerThickness::Thick.pixels(), 2.0);
    }

    #[test]
    fn test_horizontal_rule_builder() {
        let rule = HorizontalRule::new()
            .with_margin(16.0)
            .with_color(hsla(0.0, 1.0, 0.5, 1.0));

        assert_eq!(rule.margin, 16.0);
        assert!(rule.color.is_some());
    }
}
