//! Sparkline components - Inline mini charts
//!
//! Provides small inline charts for displaying data trends.

mod comparison_sparkline;
mod mini_bar_chart;
mod progress_sparkline;
mod sparkline;
mod trend_indicator;
mod types;

pub use comparison_sparkline::ComparisonSparkline;
pub use mini_bar_chart::MiniBarChart;
pub use progress_sparkline::ProgressSparkline;
pub use sparkline::Sparkline;
pub use trend_indicator::TrendIndicator;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_variants() {
        let line = Sparkline::new("l").variant(SparklineVariant::Line);
        let area = Sparkline::new("a").variant(SparklineVariant::Area);
        let bar = Sparkline::new("b").variant(SparklineVariant::Bar);

        assert_eq!(line.variant, SparklineVariant::Line);
        assert_eq!(area.variant, SparklineVariant::Area);
        assert_eq!(bar.variant, SparklineVariant::Bar);
    }

    #[test]
    fn test_sparkline_sizes() {
        let xs = SparklineSize::Xs;
        let lg = SparklineSize::Lg;

        assert!(xs.height() < lg.height());
        assert!(xs.width() < lg.width());
    }

    #[test]
    fn test_sparkline_data() {
        let sparkline = Sparkline::new("s")
            .data(vec![1.0, 2.0, 3.0, 4.0, 5.0])
            .show_min_max(true);

        assert_eq!(sparkline.data.len(), 5);
        assert!(sparkline.show_min_max);
    }

    #[test]
    fn test_trend_indicator() {
        let up = TrendIndicator::new("u", TrendDirection::Up).value(15.5);
        let down = TrendIndicator::new("d", TrendDirection::Down).value(-8.3);

        assert_eq!(up.direction, TrendDirection::Up);
        assert_eq!(up.value, Some(15.5));
        assert_eq!(down.direction, TrendDirection::Down);
    }

    #[test]
    fn test_mini_bar_chart() {
        let chart = MiniBarChart::new("c")
            .data(vec![10.0, 20.0, 15.0, 30.0])
            .labels(vec!["A", "B", "C", "D"])
            .show_labels(true);

        assert_eq!(chart.data.len(), 4);
        assert_eq!(chart.labels.len(), 4);
        assert!(chart.show_labels);
    }

    #[test]
    fn test_progress_sparkline() {
        let progress = ProgressSparkline::new("p")
            .data(vec![20.0, 35.0, 50.0, 65.0, 80.0])
            .target(100.0);

        assert_eq!(progress.data.len(), 5);
        assert_eq!(progress.target, Some(100.0));
    }

    #[test]
    fn test_comparison_sparkline() {
        let comp = ComparisonSparkline::new("c")
            .primary_data(vec![10.0, 20.0, 30.0])
            .secondary_data(vec![8.0, 18.0, 25.0]);

        assert_eq!(comp.primary_data.len(), 3);
        assert_eq!(comp.secondary_data.len(), 3);
    }
}
