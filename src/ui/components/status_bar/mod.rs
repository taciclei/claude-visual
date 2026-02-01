//! Status bar component showing workspace state

mod helpers;
mod left_section;
mod right_section;
mod status_bar;
mod types;

pub use status_bar::StatusBar;
pub use types::{StatusBarConfig, StatusBarEvent, StatusItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_config_default() {
        let config = StatusBarConfig::default();
        assert!(config.left.is_empty());
        assert!(config.center.is_empty());
        assert!(config.right.is_empty());
    }

    #[test]
    fn test_status_item_variants() {
        let text = StatusItem::Text("test".to_string());
        let separator = StatusItem::Separator;
        let icon_text = StatusItem::IconText {
            icon: "🔥",
            text: "hot".to_string(),
        };
        let clickable = StatusItem::Clickable {
            text: "click me".to_string(),
            action: "action",
        };

        assert!(matches!(text, StatusItem::Text(_)));
        assert!(matches!(separator, StatusItem::Separator));
        assert!(matches!(icon_text, StatusItem::IconText { .. }));
        assert!(matches!(clickable, StatusItem::Clickable { .. }));
    }
}
