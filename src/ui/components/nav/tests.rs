use super::*;

#[test]
fn test_nav_item() {
    let item = NavItem::new("home", "Home")
        .icon("🏠")
        .href("/home")
        .active(true)
        .badge("5");

    assert_eq!(item.id.as_ref(), "home");
    assert_eq!(item.label.as_ref(), "Home");
    assert!(item.active);
    assert_eq!(item.badge.unwrap().as_ref(), "5");
}

#[test]
fn test_nav_orientations() {
    let horizontal = Nav::new("h").orientation(NavOrientation::Horizontal);
    let vertical = Nav::new("v").orientation(NavOrientation::Vertical);

    assert_eq!(horizontal.orientation, NavOrientation::Horizontal);
    assert_eq!(vertical.orientation, NavOrientation::Vertical);
}

#[test]
fn test_nav_section() {
    let items = vec![
        NavItem::new("a", "Item A"),
        NavItem::new("b", "Item B"),
    ];
    let section = NavSection::new()
        .title("Section")
        .items(items);

    assert_eq!(section.title.unwrap().as_ref(), "Section");
    assert_eq!(section.items.len(), 2);
}

#[test]
fn test_sidebar_collapsed() {
    let expanded = SidebarNav::new("s1").collapsed(false);
    let collapsed = SidebarNav::new("s2").collapsed(true);

    assert!(!expanded.collapsed);
    assert!(collapsed.collapsed);
}

#[test]
fn test_page_indicator() {
    let indicator = PageIndicator::new(2, 5)
        .show_numbers(true);

    assert_eq!(indicator.current, 2);
    assert_eq!(indicator.total, 5);
    assert!(indicator.show_numbers);
}
