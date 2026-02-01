//! Tests for hover card components

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_hover_card() {
        let card = HoverCard::new()
            .position(HoverCardPosition::Bottom)
            .open(true)
            .delay(300)
            .max_width(400.0);
        assert!(card.open);
        assert_eq!(card.delay, 300);
        assert_eq!(card.max_width, 400.0);
    }

    #[test]
    fn test_user_hover_card() {
        let card = UserHoverCard::new("John Doe", "johndoe")
            .bio("Software developer")
            .followers(1500)
            .following(200)
            .verified(true)
            .online(true);
        assert!(card.verified);
        assert!(card.online);
        assert_eq!(card.followers, Some(1500));
    }

    #[test]
    fn test_link_preview_card() {
        let card = LinkPreviewCard::new("https://example.com")
            .title("Example Site")
            .description("A sample website")
            .loading(false);
        assert_eq!(card.title.as_deref(), Some("Example Site"));
        assert!(!card.loading);
    }

    #[test]
    fn test_definition_card() {
        let card = DefinitionCard::new("calculate", "function")
            .signature("fn calculate(x: i32) -> i32")
            .documentation("Performs a calculation")
            .file_path("src/math.rs")
            .line_number(42);
        assert_eq!(card.line_number, Some(42));
    }

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(500), "500");
        assert_eq!(format_count(1500), "1.5K");
        assert_eq!(format_count(1500000), "1.5M");
    }
}
