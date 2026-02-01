//! Floating action button components
//!
//! Provides floating action buttons (FAB) for primary actions.

mod types;
mod fab;
mod speed_dial;
mod container;
mod mini_fab;

pub use types::*;
pub use fab::Fab;
pub use speed_dial::SpeedDial;
pub use container::FabContainer;
pub use mini_fab::MiniFab;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fab_sizes() {
        let sm = FabSize::Sm;
        let lg = FabSize::Lg;

        let (sm_w, sm_h) = sm.dimensions();
        let (lg_w, lg_h) = lg.dimensions();

        assert!(sm_w < lg_w);
        assert!(sm_h < lg_h);
    }

    #[test]
    fn test_fab_variants() {
        let primary = Fab::new("p", "+").variant(FabVariant::Primary);
        let secondary = Fab::new("s", "+").variant(FabVariant::Secondary);

        assert_eq!(primary.variant, FabVariant::Primary);
        assert_eq!(secondary.variant, FabVariant::Secondary);
    }

    #[test]
    fn test_fab_extended() {
        let fab = Fab::new("f", "+").label("Create");

        assert_eq!(fab.size, FabSize::Extended);
        assert!(fab.label.is_some());
    }

    #[test]
    fn test_speed_dial() {
        let items = vec![
            SpeedDialItem::new("edit", "✏️").label("Edit"),
            SpeedDialItem::new("delete", "🗑️").label("Delete"),
        ];

        let dial = SpeedDial::new("sd", "+")
            .items(items)
            .expanded(true)
            .direction(SpeedDialDirection::Up);

        assert_eq!(dial.items.len(), 2);
        assert!(dial.expanded);
        assert_eq!(dial.direction, SpeedDialDirection::Up);
    }

    #[test]
    fn test_fab_positions() {
        let br = FabContainer::new("br").position(FabPosition::BottomRight);
        let tl = FabContainer::new("tl").position(FabPosition::TopLeft);

        assert_eq!(br.position, FabPosition::BottomRight);
        assert_eq!(tl.position, FabPosition::TopLeft);
    }

    #[test]
    fn test_mini_fab() {
        let fab = MiniFab::new("m", "+")
            .variant(FabVariant::Secondary)
            .disabled(true);

        assert_eq!(fab.variant, FabVariant::Secondary);
        assert!(fab.disabled);
    }
}
