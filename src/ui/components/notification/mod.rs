//! Notification center and notification components

mod badge;
mod bell;
mod center;
mod group;
mod notification;
mod types;

pub use badge::NotificationBadge;
pub use bell::NotificationBell;
pub use center::NotificationCenter;
pub use group::NotificationGroup;
pub use notification::Notification;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification() {
        let notif = Notification::new("1", "New message")
            .message("You have a new message from John")
            .notification_type(NotificationType::Info)
            .action("reply", "Reply")
            .action("dismiss", "Dismiss");

        assert_eq!(notif.id, "1");
        assert_eq!(notif.actions.len(), 2);
        assert!(notif.actions[0].primary);
    }

    #[test]
    fn test_notification_badge() {
        let badge = NotificationBadge::new(5);
        assert_eq!(badge.display_count(), "5");

        let badge_large = NotificationBadge::new(150).max_count(99);
        assert_eq!(badge_large.display_count(), "99+");
    }

    #[test]
    fn test_notification_center() {
        let center = NotificationCenter::new().notifications(vec![
            Notification::new("1", "Test"),
            Notification::new("2", "Test 2").read(true),
        ]);

        assert_eq!(center.notifications.len(), 2);
    }
}
