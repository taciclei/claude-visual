//! Tests for theme functionality

use super::*;
use crate::app::theme::colors::hsla;

#[test]
fn test_theme_variants() {
    for variant in ThemeVariant::all() {
        let theme = Theme::from_variant(*variant);
        assert_eq!(theme.variant, *variant);
        assert_eq!(theme.is_dark, variant.is_dark());
    }
}

#[test]
fn test_toggle_mode() {
    let mut theme = Theme::dark();
    assert!(theme.is_dark);

    theme.toggle_mode();
    assert!(!theme.is_dark);
    assert_eq!(theme.variant, ThemeVariant::Light);

    theme.toggle_mode();
    assert!(theme.is_dark);
    assert_eq!(theme.variant, ThemeVariant::Dark);
}

#[test]
fn test_toggle_high_contrast() {
    let mut theme = Theme::dark();
    assert!(!theme.is_high_contrast());

    theme.toggle_high_contrast();
    assert!(theme.is_high_contrast());
    assert_eq!(theme.variant, ThemeVariant::HighContrastDark);

    theme.toggle_high_contrast();
    assert!(!theme.is_high_contrast());
    assert_eq!(theme.variant, ThemeVariant::Dark);
}

#[test]
fn test_high_contrast_preserves_dark_light() {
    let mut theme = Theme::light();
    theme.toggle_high_contrast();
    assert_eq!(theme.variant, ThemeVariant::HighContrastLight);
    assert!(!theme.is_dark);

    theme.toggle_mode();
    assert_eq!(theme.variant, ThemeVariant::HighContrastDark);
    assert!(theme.is_dark);
}

#[test]
fn test_contrast_ratio() {
    // White on black should have maximum contrast
    let white = hsla(0.0, 0.0, 1.0, 1.0);
    let black = hsla(0.0, 0.0, 0.0, 1.0);
    let ratio = contrast_ratio(white, black);
    assert!(ratio > 20.0, "White on black contrast ratio should be ~21");
}

#[test]
fn test_wcag_compliance() {
    let dark_theme = Theme::high_contrast_dark();
    // High contrast dark text on background should meet WCAG AAA
    assert!(meets_wcag_aaa(
        dark_theme.colors.text,
        dark_theme.colors.background
    ));

    let light_theme = Theme::high_contrast_light();
    // High contrast light text on background should meet WCAG AAA
    assert!(meets_wcag_aaa(
        light_theme.colors.text,
        light_theme.colors.background
    ));
}
