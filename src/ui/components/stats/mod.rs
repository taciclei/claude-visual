//! Stats and metric display components

mod types;
mod stat_card;
mod simple_stat;
mod stats_grid;
mod metric_trend;
mod progress_stat;
mod comparison_stat;

pub use types::*;
pub use stat_card::StatCard;
pub use simple_stat::SimpleStat;
pub use stats_grid::StatsGrid;
pub use metric_trend::MetricTrend;
pub use progress_stat::ProgressStat;
pub use comparison_stat::ComparisonStat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_stat() {
        let stat = SimpleStat::new("Users", "1,234")
            .icon("👥");

        assert_eq!(stat.label, "Users");
        assert_eq!(stat.value, "1,234");
    }

    #[test]
    fn test_stats_grid() {
        let grid = StatsGrid::new()
            .stat(SimpleStat::new("A", "1"))
            .stat(SimpleStat::new("B", "2"))
            .columns(2);

        assert_eq!(grid.stats.len(), 2);
        assert_eq!(grid.columns, 2);
    }

    #[test]
    fn test_metric_trend() {
        let metric = MetricTrend::new("$1,234", "+12%", true)
            .label("Revenue");

        assert_eq!(metric.value, "$1,234");
        assert!(metric.is_positive);
    }

    #[test]
    fn test_progress_stat() {
        let stat = ProgressStat::percentage("Storage", 75.0);

        assert!((stat.max - 100.0).abs() < f32::EPSILON);
    }
}
