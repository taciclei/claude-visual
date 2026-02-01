//! Carousel and slider components for content display

use gpui::prelude::*;
use gpui::*;

mod carousel;
mod image_gallery;
mod testimonial_carousel;
mod types;

// Re-export types
pub use types::{
    CarouselAnimation, CarouselEvent, CarouselNavigation, CarouselSlide, GalleryImage, Testimonial,
};

// Re-export components
pub use carousel::Carousel;
pub use image_gallery::ImageGallery;
pub use testimonial_carousel::TestimonialCarousel;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_slide() {
        let slide = CarouselSlide::new("1")
            .title("Welcome")
            .description("Get started with our app")
            .image("🎉");

        assert_eq!(slide.id, "1");
        assert!(slide.title.is_some());
    }

    #[test]
    fn test_carousel() {
        let carousel = Carousel::new()
            .slide(CarouselSlide::new("1").title("Slide 1"))
            .slide(CarouselSlide::new("2").title("Slide 2"))
            .navigation(CarouselNavigation::Both)
            .current(0);

        assert_eq!(carousel.slides.len(), 2);
        assert_eq!(carousel.current_index, 0);
    }

    #[test]
    fn test_testimonial() {
        let testimonial = Testimonial::new("Great product!", "John Doe")
            .role("CEO")
            .rating(5);

        assert_eq!(testimonial.author, "John Doe");
        assert_eq!(testimonial.rating, Some(5));
    }
}
