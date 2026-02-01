//! Collapsible/expandable content components

mod types;
mod collapsible;
mod trigger;
mod group;
mod details;
mod expandable_text;
mod faq;

pub use types::CollapsibleAnimation;
pub use collapsible::Collapsible;
pub use trigger::CollapsibleTrigger;
pub use group::CollapsibleGroup;
pub use details::Details;
pub use expandable_text::ExpandableText;
pub use faq::{FaqItem, FaqSection};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible() {
        let collapsible = Collapsible::new("test")
            .expanded(true)
            .border(true)
            .disabled(false);
        assert!(collapsible.expanded);
        assert!(collapsible.border);
        assert!(!collapsible.disabled);
    }

    #[test]
    fn test_collapsible_trigger() {
        let trigger = CollapsibleTrigger::new("Test")
            .expanded(true)
            .bold(true)
            .font_size(16.0);
        assert!(trigger.expanded);
        assert!(trigger.bold);
        assert_eq!(trigger.font_size, 16.0);
    }

    #[test]
    fn test_details() {
        let details = Details::new("Summary")
            .open(true)
            .border_bottom(true);
        assert!(details.open);
        assert!(details.border_bottom);
    }

    #[test]
    fn test_expandable_text() {
        let text = ExpandableText::new("Long text here")
            .expanded(false)
            .max_lines(2);
        assert!(!text.expanded);
        assert_eq!(text.max_lines, 2);
    }

    #[test]
    fn test_faq_item() {
        let faq = FaqItem::new("Question?", "Answer.")
            .open(true);
        assert!(faq.open);
        assert_eq!(faq.question.as_ref(), "Question?");
        assert_eq!(faq.answer.as_ref(), "Answer.");
    }
}
