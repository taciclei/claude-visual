//! Marquee components - Scrolling and animated text/content
//!
//! Provides marquee and scrolling text components.

mod types;
mod marquee;
mod text_marquee;
mod logo_carousel;
mod ticker;
mod news_ticker;

pub use types::*;
pub use marquee::Marquee;
pub use text_marquee::TextMarquee;
pub use logo_carousel::LogoCarousel;
pub use ticker::Ticker;
pub use news_ticker::NewsTicker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marquee_directions() {
        let left = Marquee::new("l").direction(MarqueeDirection::Left);
        let right = Marquee::new("r").direction(MarqueeDirection::Right);
        let up = Marquee::new("u").direction(MarqueeDirection::Up);
        let down = Marquee::new("d").direction(MarqueeDirection::Down);

        assert_eq!(left.direction, MarqueeDirection::Left);
        assert_eq!(right.direction, MarqueeDirection::Right);
        assert_eq!(up.direction, MarqueeDirection::Up);
        assert_eq!(down.direction, MarqueeDirection::Down);
    }

    #[test]
    fn test_marquee_speeds() {
        let slow = Marquee::new("s").speed(MarqueeSpeed::Slow);
        let normal = Marquee::new("n").speed(MarqueeSpeed::Normal);
        let fast = Marquee::new("f").speed(MarqueeSpeed::Fast);

        assert_eq!(slow.speed, MarqueeSpeed::Slow);
        assert_eq!(normal.speed, MarqueeSpeed::Normal);
        assert_eq!(fast.speed, MarqueeSpeed::Fast);
    }

    #[test]
    fn test_ticker_item() {
        let item = TickerItem::new("AAPL", "$150.00").change(2.5);

        assert_eq!(item.symbol.as_ref(), "AAPL");
        assert_eq!(item.value.as_ref(), "$150.00");
        assert_eq!(item.change, Some(2.5));
    }

    #[test]
    fn test_logo_carousel() {
        let carousel = LogoCarousel::new("logos")
            .logos(vec!["Logo1", "Logo2", "Logo3"])
            .logo_size(64.0)
            .grayscale(false);

        assert_eq!(carousel.logos.len(), 3);
        assert_eq!(carousel.logo_size, 64.0);
        assert!(!carousel.grayscale);
    }
}
