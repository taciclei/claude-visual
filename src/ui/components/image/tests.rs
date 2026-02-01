//! Tests for image components

use super::*;

#[test]
fn test_image() {
    let image = Image::new("test.png")
        .alt("Test image")
        .width(200.0)
        .height(150.0)
        .shape(ImageShape::Rounded)
        .state(ImageState::Loaded);
    assert_eq!(image.src.as_ref(), "test.png");
    assert_eq!(image.alt.as_ref(), "Test image");
    assert_eq!(image.width, Some(200.0));
    assert_eq!(image.height, Some(150.0));
}

#[test]
fn test_figure() {
    let image = Image::new("photo.jpg");
    let figure = Figure::new(image)
        .caption("A beautiful photo")
        .caption_position(CaptionPosition::Bottom);
    assert_eq!(figure.caption.as_deref(), Some("A beautiful photo"));
}

#[test]
fn test_thumbnail() {
    let thumb = Thumbnail::new("avatar.png")
        .size(64.0)
        .shape(ImageShape::Circle)
        .badge("3");
    assert_eq!(thumb.size, 64.0);
    assert_eq!(thumb.shape, ImageShape::Circle);
    assert_eq!(thumb.badge.as_deref(), Some("3"));
}

#[test]
fn test_image_comparison() {
    let comparison = ImageComparison::new("before.png", "after.png")
        .slider_position(0.5)
        .size(800.0, 600.0);
    assert_eq!(comparison.slider_position, 0.5);
    assert_eq!(comparison.width, 800.0);
}

#[test]
fn test_image_placeholder() {
    let placeholder = ImagePlaceholder::new()
        .size(300.0, 200.0)
        .label("No image")
        .shape(ImageShape::Rounded);
    assert_eq!(placeholder.width, 300.0);
    assert_eq!(placeholder.height, 200.0);
}
