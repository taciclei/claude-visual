//! Accordion component for collapsible content sections

mod types;
mod accordion;
mod simple_accordion;
mod faq_accordion;

// Re-export types
pub use types::{
    AccordionMode,
    AccordionStyle,
    AccordionEvent,
    AccordionItem,
    SimpleAccordionItem,
    FaqItem,
};

// Re-export components
pub use accordion::Accordion;
pub use simple_accordion::SimpleAccordion;
pub use faq_accordion::FaqAccordion;

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
        let item = SimpleAccordionItem::new("Title", "Content")
            .icon("📁");

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
