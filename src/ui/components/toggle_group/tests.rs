//! Tests for toggle group components

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_toggle_item() {
        let item = ToggleItem::new("value", "Label")
            .icon("🏠")
            .disabled(false);
        assert_eq!(item.value.as_ref(), "value");
        assert_eq!(item.label.as_ref(), "Label");
        assert_eq!(item.icon.as_deref(), Some("🏠"));
    }

    #[test]
    fn test_toggle_group() {
        let group = ToggleGroup::new()
            .item(ToggleItem::new("a", "Option A"))
            .item(ToggleItem::new("b", "Option B"))
            .selected("a")
            .size(ToggleGroupSize::Medium)
            .variant(ToggleGroupVariant::Default);
        assert_eq!(group.items.len(), 2);
        assert_eq!(group.selected.as_deref(), Some("a"));
    }

    #[test]
    fn test_segmented_control() {
        let control = SegmentedControl::new()
            .item(ToggleItem::new("day", "Day"))
            .item(ToggleItem::new("week", "Week"))
            .item(ToggleItem::new("month", "Month"))
            .selected("week");
        assert_eq!(control.items.len(), 3);
        assert_eq!(control.selected.as_deref(), Some("week"));
    }

    #[test]
    fn test_button_group() {
        let group = ButtonGroup::new()
            .item(ButtonGroupItem::new("Left").active(true))
            .item(ButtonGroupItem::new("Center"))
            .item(ButtonGroupItem::new("Right"))
            .variant(ButtonGroupVariant::Outline);
        assert_eq!(group.items.len(), 3);
        assert!(group.items[0].active);
    }

    #[test]
    fn test_icon_toggle_group() {
        let group = IconToggleGroup::new()
            .icon(IconToggleItem::new("grid", "▦"))
            .icon(IconToggleItem::new("list", "☰"))
            .selected("grid")
            .size(36.0);
        assert_eq!(group.icons.len(), 2);
        assert_eq!(group.selected.as_deref(), Some("grid"));
    }
}
