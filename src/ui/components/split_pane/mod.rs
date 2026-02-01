//! Split pane component for resizable panels

mod types;
mod split_pane;
mod horizontal_split;
mod vertical_split;
mod collapsible_sidebar;

pub use types::*;
pub use split_pane::SplitPane;
pub use horizontal_split::HorizontalSplit;
pub use vertical_split::VerticalSplit;
pub use collapsible_sidebar::CollapsibleSidebar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_split() {
        let split = HorizontalSplit::new(0.3)
            .divider_width(6.0);

        assert!((split.position - 0.3).abs() < f64::EPSILON as f32);
        assert!((split.divider_width - 6.0).abs() < f64::EPSILON as f32);
    }

    #[test]
    fn test_vertical_split() {
        let split = VerticalSplit::new(0.7)
            .divider_height(8.0);

        assert!((split.position - 0.7).abs() < f64::EPSILON as f32);
        assert!((split.divider_height - 8.0).abs() < f64::EPSILON as f32);
    }

    #[test]
    fn test_collapsible_sidebar() {
        let sidebar = CollapsibleSidebar::new(250.0)
            .right()
            .collapsed();

        assert!((sidebar.width - 250.0).abs() < f64::EPSILON as f32);
        assert!(sidebar.collapsed);
        assert_eq!(sidebar.position, SidebarPosition::Right);
    }
}
