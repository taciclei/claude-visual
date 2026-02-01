//! Card component for content containers

mod types;
mod main_card;
mod simple_card;
mod collapsible_card;

// Re-export types
pub use types::{CardVariant, CardPadding, CardEvent};

// Re-export components
pub use main_card::Card;
pub use simple_card::SimpleCard;
pub use collapsible_card::CollapsibleCard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_padding() {
        assert_eq!(CardPadding::None.pixels(), 0.0);
        assert_eq!(CardPadding::Small.pixels(), 8.0);
        assert_eq!(CardPadding::Medium.pixels(), 16.0);
        assert_eq!(CardPadding::Large.pixels(), 24.0);
    }

    #[test]
    fn test_simple_card_builder() {
        let card = SimpleCard::new()
            .with_padding(24.0)
            .with_rounded(12.0)
            .without_border();

        assert_eq!(card.padding, 24.0);
        assert_eq!(card.rounded, 12.0);
        assert!(!card.bordered);
    }
}
