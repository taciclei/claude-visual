//! Tests for message module

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use super::super::utils::*;
    use crate::claude::message::MessageRole;
    use chrono::Utc;

    #[test]
    fn test_message_action_labels() {
        assert_eq!(MessageAction::Copy.label(), "Copy");
        assert_eq!(MessageAction::CopyAsMarkdown.label(), "Copy as Markdown");
        assert_eq!(MessageAction::Regenerate.label(), "Regenerate Response");
        assert_eq!(MessageAction::Edit.label(), "Edit Message");
        assert_eq!(MessageAction::Quote.label(), "Quote in Reply");
        assert_eq!(MessageAction::Bookmark.label(), "Toggle Bookmark");
        assert_eq!(MessageAction::Delete.label(), "Delete");
    }

    #[test]
    fn test_message_action_shortcuts() {
        assert_eq!(MessageAction::Copy.shortcut(), Some("⌘C"));
        assert_eq!(MessageAction::Bookmark.shortcut(), Some("⌘B"));
        assert_eq!(MessageAction::Delete.shortcut(), Some("⌫"));
        assert_eq!(MessageAction::Regenerate.shortcut(), None);
    }

    #[test]
    fn test_message_action_role_availability() {
        // Copy, Quote, and Bookmark available for all
        assert!(MessageAction::Copy.available_for_role(MessageRole::User));
        assert!(MessageAction::Copy.available_for_role(MessageRole::Assistant));
        assert!(MessageAction::Quote.available_for_role(MessageRole::User));
        assert!(MessageAction::Quote.available_for_role(MessageRole::Assistant));
        assert!(MessageAction::Bookmark.available_for_role(MessageRole::User));
        assert!(MessageAction::Bookmark.available_for_role(MessageRole::Assistant));

        // Regenerate only for assistant
        assert!(!MessageAction::Regenerate.available_for_role(MessageRole::User));
        assert!(MessageAction::Regenerate.available_for_role(MessageRole::Assistant));

        // Edit only for user
        assert!(MessageAction::Edit.available_for_role(MessageRole::User));
        assert!(!MessageAction::Edit.available_for_role(MessageRole::Assistant));
    }

    #[test]
    fn test_message_action_destructive() {
        assert!(MessageAction::Delete.is_destructive());
        assert!(!MessageAction::Copy.is_destructive());
        assert!(!MessageAction::Edit.is_destructive());
        assert!(!MessageAction::Bookmark.is_destructive());
    }

    #[test]
    fn test_format_relative_time_just_now() {
        let now = Utc::now();
        assert_eq!(format_relative_time(now), "just now");

        let thirty_secs_ago = now - chrono::Duration::seconds(30);
        assert_eq!(format_relative_time(thirty_secs_ago), "just now");
    }

    #[test]
    fn test_format_relative_time_minutes() {
        let now = Utc::now();

        let one_min_ago = now - chrono::Duration::minutes(1);
        assert_eq!(format_relative_time(one_min_ago), "1 min ago");

        let five_min_ago = now - chrono::Duration::minutes(5);
        assert_eq!(format_relative_time(five_min_ago), "5 min ago");

        let thirty_min_ago = now - chrono::Duration::minutes(30);
        assert_eq!(format_relative_time(thirty_min_ago), "30 min ago");
    }

    #[test]
    fn test_format_relative_time_hours() {
        let now = Utc::now();

        let one_hour_ago = now - chrono::Duration::hours(1);
        assert_eq!(format_relative_time(one_hour_ago), "1 hour ago");

        let three_hours_ago = now - chrono::Duration::hours(3);
        assert_eq!(format_relative_time(three_hours_ago), "3 hours ago");
    }

    #[test]
    fn test_format_relative_time_days() {
        let now = Utc::now();

        let yesterday = now - chrono::Duration::days(1);
        assert_eq!(format_relative_time(yesterday), "yesterday");

        let three_days_ago = now - chrono::Duration::days(3);
        assert_eq!(format_relative_time(three_days_ago), "3 days ago");
    }

    #[test]
    fn test_message_reaction_emoji() {
        assert_eq!(MessageReaction::ThumbsUp.emoji(), "👍");
        assert_eq!(MessageReaction::ThumbsDown.emoji(), "👎");
    }

    #[test]
    fn test_message_reaction_label() {
        assert_eq!(MessageReaction::ThumbsUp.label(), "Good response");
        assert_eq!(MessageReaction::ThumbsDown.label(), "Poor response");
    }

    #[test]
    fn test_message_reaction_equality() {
        assert_eq!(MessageReaction::ThumbsUp, MessageReaction::ThumbsUp);
        assert_ne!(MessageReaction::ThumbsUp, MessageReaction::ThumbsDown);
    }
}
