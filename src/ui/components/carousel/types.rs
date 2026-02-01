//! Type definitions for carousel components

use gpui::*;
use gpui::prelude::*;

/// Carousel navigation style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CarouselNavigation {
    /// Arrows on sides (default)
    #[default]
    Arrows,
    /// Dots below
    Dots,
    /// Both arrows and dots
    Both,
    /// No navigation (swipe only)
    None,
}

/// Carousel animation style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CarouselAnimation {
    /// Slide horizontally (default)
    #[default]
    Slide,
    /// Fade transition
    Fade,
    /// No animation
    None,
}

/// Single carousel slide
#[derive(Clone)]
pub struct CarouselSlide {
    /// Unique ID
    pub id: String,
    /// Title
    pub title: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Image/icon
    pub image: Option<String>,
    /// Background color
    pub background: Option<Hsla>,
}

impl CarouselSlide {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            description: None,
            image: None,
            background: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }
}

/// Events emitted by Carousel
#[derive(Debug, Clone)]
pub enum CarouselEvent {
    /// Slide changed
    SlideChanged(usize),
    /// Navigation clicked
    NavigationClicked { direction: i32 },
    /// Dot clicked
    DotClicked(usize),
}

/// Gallery image data
#[derive(Clone)]
pub struct GalleryImage {
    pub src: String,
    pub alt: Option<String>,
    pub caption: Option<String>,
}

impl GalleryImage {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: None,
            caption: None,
        }
    }

    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

/// Testimonial data
#[derive(Clone)]
pub struct Testimonial {
    pub quote: String,
    pub author: String,
    pub role: Option<String>,
    pub avatar: Option<String>,
    pub rating: Option<u8>,
}

impl Testimonial {
    pub fn new(quote: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: author.into(),
            role: None,
            avatar: None,
            rating: None,
        }
    }

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = Some(rating.min(5));
        self
    }
}
