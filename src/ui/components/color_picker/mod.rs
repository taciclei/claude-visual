//! Color picker component

mod types;
mod picker;
mod swatch;
mod palette;
mod gradient;

pub use types::*;
pub use picker::*;
pub use swatch::*;
pub use palette::*;
pub use gradient::*;

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::*;

    #[test]
    fn test_color_swatch() {
        let swatch = ColorSwatch::new(hsla(0.5, 0.8, 0.5, 1.0))
            .size(32.0)
            .rounded();

        assert!((swatch.size - 32.0).abs() < f32::EPSILON);
        assert!(swatch.rounded);
    }

    #[test]
    fn test_color_palette() {
        let palette = ColorPalette::new()
            .color("Red", hsla(0.0, 0.8, 0.5, 1.0))
            .color("Blue", hsla(0.6, 0.8, 0.5, 1.0))
            .columns(4);

        assert_eq!(palette.colors.len(), 2);
        assert_eq!(palette.columns, 4);
    }

    #[test]
    fn test_gradient_bar() {
        let gradient = GradientBar::new(
            hsla(0.0, 0.8, 0.5, 1.0),
            hsla(0.5, 0.8, 0.5, 1.0)
        ).height(24.0);

        assert!((gradient.height - 24.0).abs() < f32::EPSILON);
    }
}
