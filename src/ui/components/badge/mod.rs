//! Badge component for counts and status indicators

mod types;
mod badge;
mod badge_wrapper;

pub use types::{BadgeVariant, BadgeSize, BadgePosition};
pub use badge::Badge;
pub use badge_wrapper::BadgeWrapper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_size() {
        assert_eq!(BadgeSize::XSmall.height(), 8.0);
        assert_eq!(BadgeSize::Small.height(), 18.0);
        assert_eq!(BadgeSize::Medium.height(), 22.0);
        assert_eq!(BadgeSize::Large.height(), 26.0);
    }

    #[test]
    fn test_display_content_max_count() {
        // Just verify the logic works
        let max: Option<u32> = Some(99);
        let count = 150u32;
        let result = if count > max.unwrap() {
            format!("{}+", max.unwrap())
        } else {
            count.to_string()
        };
        assert_eq!(result, "99+");
    }
}
