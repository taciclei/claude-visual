//! Resizable panel and container components

mod corner_resize_handle;
mod resizable_panel;
mod resizable_split_view;
mod resize_handle;
mod types;

pub use corner_resize_handle::CornerResizeHandle;
pub use resizable_panel::ResizablePanel;
pub use resizable_split_view::ResizableSplitView;
pub use resize_handle::ResizeHandle;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizable_panel() {
        let panel = ResizablePanel::new()
            .direction(ResizeDirection::Horizontal)
            .min_size(150.0)
            .max_size(500.0)
            .default_size(300.0)
            .collapsible(true);
        assert_eq!(panel.min_size, 150.0);
        assert_eq!(panel.max_size, Some(500.0));
        assert_eq!(panel.default_size, 300.0);
        assert!(panel.collapsible);
    }

    #[test]
    fn test_resize_handle() {
        let handle = ResizeHandle::horizontal()
            .style(HandleStyle::Dots)
            .size(12.0)
            .active(true);
        assert!(handle.active);
        assert_eq!(handle.size, 12.0);
    }

    #[test]
    fn test_resizable_split_view() {
        let split = ResizableSplitView::new()
            .split_ratio(0.3)
            .direction(ResizeDirection::Horizontal)
            .min_first(200.0)
            .min_second(300.0);
        assert_eq!(split.split_ratio, 0.3);
        assert_eq!(split.min_first, 200.0);
        assert_eq!(split.min_second, 300.0);
    }

    #[test]
    fn test_corner_resize_handle() {
        let corner = CornerResizeHandle::new()
            .position(CornerPosition::BottomRight)
            .size(20.0)
            .visible(true);
        assert_eq!(corner.position, CornerPosition::BottomRight);
        assert_eq!(corner.size, 20.0);
        assert!(corner.visible);
    }
}
