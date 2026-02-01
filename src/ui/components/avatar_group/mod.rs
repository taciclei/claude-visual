//! Avatar group components for displaying multiple avatars

mod types;
mod avatar_group;
mod avatar_stack;
mod assignee_picker;
mod team_member_item;

pub use types::*;
pub use avatar_group::AvatarGroup;
pub use avatar_stack::AvatarStack;
pub use assignee_picker::AssigneePicker;
pub use team_member_item::TeamMemberItem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_avatar_initials() {
        let avatar = GroupAvatar::new("John Doe");
        assert_eq!(avatar.get_initials(), "JD");

        let avatar2 = GroupAvatar::new("Alice");
        assert_eq!(avatar2.get_initials(), "A");

        let avatar3 = GroupAvatar::new("Mary Jane Watson");
        assert_eq!(avatar3.get_initials(), "MJ");
    }

    #[test]
    fn test_avatar_group() {
        let group = AvatarGroup::new()
            .avatar(GroupAvatar::new("Alice"))
            .avatar(GroupAvatar::new("Bob"))
            .avatar(GroupAvatar::new("Charlie"))
            .max_visible(2)
            .size(AvatarGroupSize::Large);

        assert_eq!(group.avatars.len(), 3);
        assert_eq!(group.max_visible, 2);
    }

    #[test]
    fn test_avatar_sizes() {
        assert_eq!(AvatarGroupSize::Small.avatar_size(), 24.0);
        assert_eq!(AvatarGroupSize::Medium.avatar_size(), 32.0);
        assert_eq!(AvatarGroupSize::Large.avatar_size(), 40.0);
    }

    #[test]
    fn test_team_member_item() {
        let member = TeamMemberItem::new("John Doe")
            .role("Developer")
            .email("john@example.com")
            .online(true);

        assert_eq!(member.avatar.name, "John Doe");
        assert_eq!(member.role, Some("Developer".to_string()));
        assert_eq!(member.avatar.is_online, Some(true));
    }
}
