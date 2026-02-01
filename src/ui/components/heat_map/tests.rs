use super::*;

#[test]
fn test_heat_map_creation() {
    let hm = HeatMap::new("hm-1")
        .scale(HeatMapScale::Green)
        .cell_size(14.0)
        .show_legend(true);
    assert_eq!(hm.scale, HeatMapScale::Green);
    assert_eq!(hm.cell_size, 14.0);
}

#[test]
fn test_heat_map_cell() {
    let cell = HeatMapCell::new(5.0)
        .with_label("Test")
        .with_date("2024-01-15");
    assert_eq!(cell.value, 5.0);
    assert!(cell.label.is_some());
    assert!(cell.date.is_some());
}

#[test]
fn test_heat_map_value_to_level() {
    let hm = HeatMap::new("hm-test").levels(5);
    assert_eq!(hm.value_to_level(0.0, 100.0), 0);
    assert_eq!(hm.value_to_level(100.0, 100.0), 4);
    assert_eq!(hm.value_to_level(50.0, 100.0), 2);
}

#[test]
fn test_contribution_calendar() {
    let cc = ContributionCalendar::new("cc-1")
        .weeks_to_show(26)
        .scale(HeatMapScale::Blue);
    assert_eq!(cc.weeks_to_show, 26);
    assert_eq!(cc.scale, HeatMapScale::Blue);
}

#[test]
fn test_activity_streak() {
    let streak = ActivityStreak::new("as-1")
        .current_streak(30)
        .longest_streak(45)
        .total_days(365);
    assert_eq!(streak.current_streak, 30);
    assert_eq!(streak.longest_streak, 45);
    assert_eq!(streak.total_days, 365);
}

#[test]
fn test_data_grid_heat_map() {
    let dg = DataGridHeatMap::new("dg-1")
        .rows(vec!["A", "B", "C"])
        .cols(vec!["X", "Y", "Z"])
        .values(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);
    assert_eq!(dg.rows.len(), 3);
    assert_eq!(dg.cols.len(), 3);
    assert_eq!(dg.get_max(), 9.0);
}

#[test]
fn test_heat_map_scale_colors() {
    let green = HeatMapScale::Green.colors();
    let blue = HeatMapScale::Blue.colors();
    assert_eq!(green.len(), 5);
    assert_eq!(blue.len(), 5);
}
