//! Breadcrumb navigation component

mod component;
mod types;

pub use component::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_item_creation() {
        let item = BreadcrumbItem::new("test", "Test Label")
            .with_icon("📁")
            .non_clickable();

        assert_eq!(item.id, "test");
        assert_eq!(item.label, "Test Label");
        assert_eq!(item.icon, Some("📁".to_string()));
        assert!(!item.clickable);
    }
}
