//! Mention components - @mention input functionality
//!
//! Provides components for mentioning users, channels, or other entities.

mod types;
mod mention;
mod mention_dropdown;
mod mention_input;
mod channel_mention;

pub use types::*;
pub use mention::Mention;
pub use mention_dropdown::MentionDropdown;
pub use mention_input::MentionInput;
pub use channel_mention::ChannelMention;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention_variants() {
        let user = Mention::new("u", "john").variant(MentionVariant::User);
        let channel = Mention::new("c", "general").variant(MentionVariant::Channel);
        let team = Mention::new("t", "engineering").variant(MentionVariant::Team);

        assert_eq!(user.variant, MentionVariant::User);
        assert_eq!(channel.variant, MentionVariant::Channel);
        assert_eq!(team.variant, MentionVariant::Team);
    }

    #[test]
    fn test_mention_sizes() {
        let sm = MentionSize::Sm;
        let lg = MentionSize::Lg;

        assert!(sm.font_size() < lg.font_size());
    }

    #[test]
    fn test_mention_self() {
        let mention = Mention::new("m", "me")
            .is_self(true)
            .avatar("👤");

        assert!(mention.is_self);
        assert!(mention.avatar.is_some());
    }

    #[test]
    fn test_mentionable_user() {
        let user = MentionableUser::new("1", "John Doe", "johnd")
            .avatar("J")
            .status("In a meeting")
            .is_online(true);

        assert_eq!(user.name.as_ref(), "John Doe");
        assert!(user.is_online);
    }

    #[test]
    fn test_mention_dropdown() {
        let users = vec![
            MentionableUser::new("1", "Alice", "alice"),
            MentionableUser::new("2", "Bob", "bob"),
        ];

        let dropdown = MentionDropdown::new("md")
            .users(users)
            .query("al")
            .selected_index(0);

        assert_eq!(dropdown.users.len(), 2);
        assert_eq!(dropdown.selected_index, Some(0));
    }

    #[test]
    fn test_channel_mention() {
        let channel = ChannelMention::new("ch", "general")
            .description("General discussion")
            .member_count(150)
            .is_private(false);

        assert!(!channel.is_private);
        assert_eq!(channel.member_count, Some(150));
    }
}
