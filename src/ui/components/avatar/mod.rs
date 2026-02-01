//! Avatar component for user/assistant display

mod types;
mod avatar;
mod avatar_group;

pub use types::*;
pub use avatar::Avatar;
pub use avatar_group::{AvatarGroup, AvatarGroupItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_initials() {
        assert_eq!(Avatar::extract_initials("John"), "J");
        assert_eq!(Avatar::extract_initials("John Doe"), "JD");
        assert_eq!(Avatar::extract_initials("John Michael Doe"), "JD");
        assert_eq!(Avatar::extract_initials(""), "?");
        assert_eq!(Avatar::extract_initials("alice"), "A");
    }

    #[test]
    fn test_avatar_size_pixels() {
        assert_eq!(AvatarSize::XSmall.pixels(), 16.0);
        assert_eq!(AvatarSize::Small.pixels(), 24.0);
        assert_eq!(AvatarSize::Medium.pixels(), 32.0);
        assert_eq!(AvatarSize::Large.pixels(), 48.0);
        assert_eq!(AvatarSize::XLarge.pixels(), 64.0);
    }
}
