use super::Announcement;

/// Common announcements for the application
pub struct CommonAnnouncements;

impl CommonAnnouncements {
    /// Announce that content has loaded
    pub fn content_loaded() -> Announcement {
        Announcement::polite("Content loaded")
    }

    /// Announce a loading state
    pub fn loading() -> Announcement {
        Announcement::polite("Loading...")
    }

    /// Announce an error
    pub fn error(message: impl Into<String>) -> Announcement {
        Announcement::assertive(format!("Error: {}", message.into()))
    }

    /// Announce a success
    pub fn success(message: impl Into<String>) -> Announcement {
        Announcement::polite(format!("Success: {}", message.into()))
    }

    /// Announce dialog opened
    pub fn dialog_opened(title: impl Into<String>) -> Announcement {
        Announcement::assertive(format!("Dialog opened: {}", title.into()))
    }

    /// Announce dialog closed
    pub fn dialog_closed() -> Announcement {
        Announcement::polite("Dialog closed")
    }

    /// Announce navigation
    pub fn navigated_to(location: impl Into<String>) -> Announcement {
        Announcement::polite(format!("Navigated to {}", location.into()))
    }

    /// Announce new message received
    pub fn new_message(sender: impl Into<String>) -> Announcement {
        Announcement::polite(format!("New message from {}", sender.into()))
    }

    /// Announce message sent
    pub fn message_sent() -> Announcement {
        Announcement::polite("Message sent")
    }

    /// Announce Claude is thinking
    pub fn thinking() -> Announcement {
        Announcement::polite("Claude is thinking...")
    }

    /// Announce Claude finished responding
    pub fn response_complete() -> Announcement {
        Announcement::polite("Claude finished responding")
    }

    /// Announce file copied to clipboard
    pub fn copied_to_clipboard() -> Announcement {
        Announcement::polite("Copied to clipboard")
    }

    /// Announce selected items count
    pub fn items_selected(count: usize) -> Announcement {
        let message = if count == 1 {
            "1 item selected".to_string()
        } else {
            format!("{} items selected", count)
        };
        Announcement::polite(message)
    }
}
