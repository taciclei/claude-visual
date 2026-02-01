//! Accordion component for collapsible content sections

mod accordion;
mod faq_accordion;
mod simple_accordion;
mod types;

// Re-export types
pub use types::{
    AccordionEvent, AccordionItem, AccordionMode, AccordionStyle, FaqItem, SimpleAccordionItem,
};

// Re-export components
pub use accordion::Accordion;
pub use faq_accordion::FaqAccordion;
pub use simple_accordion::SimpleAccordion;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_item() {
        let item = AccordionItem::new("Title")
            .subtitle("Subtitle")
            .icon("📁")
            .disabled();

        assert_eq!(item.title, "Title");
        assert_eq!(item.subtitle, Some("Subtitle".to_string()));
        assert_eq!(item.icon, Some("📁".to_string()));
        assert!(item.disabled);
    }

    #[test]
    fn test_simple_accordion_item() {
        let item = SimpleAccordionItem::new("Title", "Content").icon("📁");

        assert_eq!(item.title, "Title");
        assert_eq!(item.content, "Content");
        assert_eq!(item.icon, Some("📁".to_string()));
    }

    #[test]
    fn test_faq_accordion() {
        let faq = FaqAccordion::new()
            .item("Question 1?", "Answer 1")
            .item("Question 2?", "Answer 2");

        assert_eq!(faq.items.len(), 2);
    }
}
