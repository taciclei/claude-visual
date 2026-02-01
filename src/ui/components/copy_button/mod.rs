//! Copy button components - Copy to clipboard functionality
//!
//! Provides copy button components with visual feedback.

mod copy_button;
mod copy_code_button;
mod copy_link;
mod share_button;
mod types;

// Re-export types
pub use types::*;

// Re-export components
pub use copy_button::CopyButton;
pub use copy_code_button::CopyCodeButton;
pub use copy_link::CopyLink;
pub use share_button::ShareButton;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_button_sizes() {
        let small = CopyButton::new("s", "text").size(CopyButtonSize::Small);
        let medium = CopyButton::new("m", "text").size(CopyButtonSize::Medium);
        let large = CopyButton::new("l", "text").size(CopyButtonSize::Large);

        assert_eq!(small.size, CopyButtonSize::Small);
        assert_eq!(medium.size, CopyButtonSize::Medium);
        assert_eq!(large.size, CopyButtonSize::Large);
    }

    #[test]
    fn test_copy_button_states() {
        let idle = CopyButton::new("i", "text").state(CopyState::Idle);
        let copying = CopyButton::new("c", "text").state(CopyState::Copying);
        let copied = CopyButton::new("d", "text").state(CopyState::Copied);
        let error = CopyButton::new("e", "text").state(CopyState::Error);

        assert_eq!(idle.state, CopyState::Idle);
        assert_eq!(copying.state, CopyState::Copying);
        assert_eq!(copied.state, CopyState::Copied);
        assert_eq!(error.state, CopyState::Error);
    }

    #[test]
    fn test_copy_button_variants() {
        let default = CopyButton::new("d", "text").variant(CopyButtonVariant::Default);
        let ghost = CopyButton::new("g", "text").variant(CopyButtonVariant::Ghost);
        let outline = CopyButton::new("o", "text").variant(CopyButtonVariant::Outline);

        assert_eq!(default.variant, CopyButtonVariant::Default);
        assert_eq!(ghost.variant, CopyButtonVariant::Ghost);
        assert_eq!(outline.variant, CopyButtonVariant::Outline);
    }

    #[test]
    fn test_copy_code_positions() {
        let tr = CopyCodeButton::new("tr", "code").position(CopyCodePosition::TopRight);
        let tl = CopyCodeButton::new("tl", "code").position(CopyCodePosition::TopLeft);
        let br = CopyCodeButton::new("br", "code").position(CopyCodePosition::BottomRight);
        let bl = CopyCodeButton::new("bl", "code").position(CopyCodePosition::BottomLeft);

        assert_eq!(tr.position, CopyCodePosition::TopRight);
        assert_eq!(tl.position, CopyCodePosition::TopLeft);
        assert_eq!(br.position, CopyCodePosition::BottomRight);
        assert_eq!(bl.position, CopyCodePosition::BottomLeft);
    }
}
