//! Banner component for system-wide notifications

mod types;
mod banner;
mod notification_banner;
mod cookie_banner;

pub use types::*;
pub use banner::Banner;
pub use notification_banner::NotificationBanner;
pub use cookie_banner::CookieBanner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_banner() {
        let banner = NotificationBanner::success("Operation completed!")
            .not_dismissible();

        assert_eq!(banner.message, "Operation completed!");
        assert!(!banner.dismissible);
    }

    #[test]
    fn test_cookie_banner() {
        let banner = CookieBanner::new()
            .message("Custom message")
            .accept_label("OK");

        assert_eq!(banner.message, "Custom message");
        assert_eq!(banner.accept_label, "OK");
    }
}
