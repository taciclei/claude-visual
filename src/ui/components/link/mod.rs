//! Link and anchor components

use gpui::*;
use gpui::prelude::*;

mod types;
mod link;
mod nav_link;
mod breadcrumb;
mod skip_link;
mod footer_link;
mod link_list;
mod anchor_link;

// Re-export types
pub use types::{LinkVariant, LinkSize, LinkListDirection};

// Re-export components
pub use link::Link;
pub use nav_link::NavLink;
pub use breadcrumb::BreadcrumbLink;
pub use skip_link::SkipLink;
pub use footer_link::FooterLink;
pub use link_list::LinkList;
pub use anchor_link::AnchorLink;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link() {
        let link = Link::new("Click me", "https://example.com")
            .variant(LinkVariant::Underline)
            .size(LinkSize::Large)
            .external(true);
        assert_eq!(link.label.as_ref(), "Click me");
        assert_eq!(link.href.as_ref(), "https://example.com");
        assert!(link.external);
    }

    #[test]
    fn test_nav_link() {
        let link = NavLink::new("Home", "/")
            .active(true)
            .icon("🏠")
            .badge("3");
        assert!(link.active);
        assert_eq!(link.icon.as_deref(), Some("🏠"));
        assert_eq!(link.badge.as_deref(), Some("3"));
    }

    #[test]
    fn test_breadcrumb_link() {
        let link = BreadcrumbLink::new("Products")
            .href("/products")
            .current(false);
        assert_eq!(link.label.as_ref(), "Products");
        assert!(!link.current);
    }

    #[test]
    fn test_link_list() {
        let list = LinkList::new()
            .title("Resources")
            .link(FooterLink::new("Blog", "/blog"))
            .link(FooterLink::new("Docs", "/docs"))
            .direction(LinkListDirection::Vertical);
        assert_eq!(list.title.as_deref(), Some("Resources"));
        assert_eq!(list.links.len(), 2);
    }

    #[test]
    fn test_anchor_link() {
        let link = AnchorLink::new("Introduction", "intro")
            .show_hash(true);
        assert_eq!(link.label.as_ref(), "Introduction");
        assert_eq!(link.anchor_id.as_ref(), "intro");
        assert!(link.show_hash);
    }
}
