//! Aspect ratio container components

mod aspect_card;
mod aspect_grid;
mod aspect_ratio;
mod embed_container;
mod ratio_image;
mod types;
mod video_placeholder;

pub use aspect_card::AspectCard;
pub use aspect_grid::AspectGrid;
pub use aspect_ratio::AspectRatio;
pub use embed_container::EmbedContainer;
pub use ratio_image::RatioImage;
pub use types::{ImageFit, Ratio};
pub use video_placeholder::VideoPlaceholder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio_values() {
        assert_eq!(Ratio::Square.value(), 1.0);
        assert!((Ratio::Video.value() - 1.778).abs() < 0.01);
        assert!((Ratio::Classic.value() - 1.333).abs() < 0.01);
        assert_eq!(Ratio::Custom(2.0).value(), 2.0);
    }

    #[test]
    fn test_aspect_ratio() {
        let ar = AspectRatio::new(Ratio::Video)
            .width(400.0)
            .rounded(true)
            .border(true);
        assert_eq!(ar.width, Some(400.0));
        assert!(ar.rounded);
        assert!(ar.border);
    }

    #[test]
    fn test_video_placeholder() {
        let placeholder = VideoPlaceholder::new()
            .width(640.0)
            .duration("10:32")
            .title("Introduction Video");
        assert_eq!(placeholder.width, 640.0);
        assert_eq!(placeholder.duration.as_deref(), Some("10:32"));
    }

    #[test]
    fn test_embed_container() {
        let embed = EmbedContainer::new("https://example.com/video", "Example Video")
            .ratio(Ratio::Video)
            .loading(true);
        assert!(embed.loading);
    }

    #[test]
    fn test_ratio_image() {
        let image = RatioImage::new("photo.jpg")
            .ratio(Ratio::Photo)
            .width(300.0)
            .fit(ImageFit::Cover);
        assert_eq!(image.ratio, Ratio::Photo);
        assert_eq!(image.width, Some(300.0));
    }

    #[test]
    fn test_aspect_card() {
        let card = AspectCard::new().header_ratio(Ratio::Classic).width(350.0);
        assert_eq!(card.width, 350.0);
        assert_eq!(card.header_ratio, Ratio::Classic);
    }
}
