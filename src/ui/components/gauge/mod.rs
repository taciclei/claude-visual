//! Gauge and radial visualization components
//!
//! Provides gauge meters, speedometers, circular progress, and radial charts.

mod types;
mod basic;
mod speed;
mod circular;
mod radial;
mod multi;

// Re-export types
pub use types::{GaugeStyle, GaugeSize, GaugeZone};

// Re-export components
pub use basic::Gauge;
pub use speed::SpeedGauge;
pub use circular::CircularProgress;
pub use radial::{RadialChart, RadialSegment};
pub use multi::{MultiGauge, GaugeRing};

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::*;

    #[test]
    fn test_gauge_creation() {
        let gauge = Gauge::new("g-1")
            .value(75.0)
            .range(0.0, 100.0)
            .style(GaugeStyle::Semicircle);
        assert_eq!(gauge.value, 75.0);
        assert_eq!(gauge.normalized(), 0.75);
    }

    #[test]
    fn test_gauge_zones() {
        let gauge = Gauge::new("g-2")
            .value(85.0)
            .add_zone(GaugeZone::new(0.0, 50.0, rgb(0x22c55e).into()))
            .add_zone(GaugeZone::new(50.0, 80.0, rgb(0xeab308).into()))
            .add_zone(GaugeZone::new(80.0, 100.0, rgb(0xef4444).into()));
        assert_eq!(gauge.zones.len(), 3);
    }

    #[test]
    fn test_speed_gauge() {
        let sg = SpeedGauge::new("sg-1")
            .value(120.0)
            .max_value(200.0)
            .unit("mph");
        assert_eq!(sg.value, 120.0);
        assert_eq!(sg.unit.as_ref(), "mph");
    }

    #[test]
    fn test_circular_progress() {
        let cp = CircularProgress::new("cp-1")
            .percentage(65.0)
            .label("Complete");
        assert_eq!(cp.normalized(), 0.65);
        assert!(cp.show_percentage);
    }

    #[test]
    fn test_radial_chart() {
        let rc = RadialChart::new("rc-1")
            .add_segment(RadialSegment::new(30.0, rgb(0x3b82f6).into()))
            .add_segment(RadialSegment::new(50.0, rgb(0x22c55e).into()))
            .add_segment(RadialSegment::new(20.0, rgb(0xef4444).into()));
        assert_eq!(rc.total(), 100.0);
    }

    #[test]
    fn test_multi_gauge() {
        let mg = MultiGauge::new("mg-1")
            .add_ring(GaugeRing::new(80.0, 100.0, rgb(0x3b82f6).into(), "CPU"))
            .add_ring(GaugeRing::new(4.0, 8.0, rgb(0x22c55e).into(), "RAM"))
            .add_ring(GaugeRing::new(256.0, 512.0, rgb(0xa855f7).into(), "Disk"));
        assert_eq!(mg.rings.len(), 3);
        assert_eq!(mg.rings[0].percentage(), 80.0);
        assert_eq!(mg.rings[1].percentage(), 50.0);
    }

    #[test]
    fn test_gauge_size() {
        assert!(GaugeSize::Xs.diameter() < GaugeSize::Md.diameter());
        assert!(GaugeSize::Md.diameter() < GaugeSize::Xl.diameter());
    }
}
