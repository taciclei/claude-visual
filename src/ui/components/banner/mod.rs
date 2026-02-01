//! Banner component for system-wide notifications

mod banner;
mod cookie_banner;
mod notification_banner;
mod types;

pub use banner::Banner;
pub use cookie_banner::CookieBanner;
pub use notification_banner::NotificationBanner;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_banner() {
        let banner = NotificationBanner::success("Operation completed!").not_dismissible();

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
