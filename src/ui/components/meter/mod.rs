//! Meter components - Progress meters, gauges, and indicators
//!
//! Provides visual meter and gauge components for displaying values.

mod battery;
mod circular_gauge;
mod meter;
mod signal;
mod speedometer;
mod types;

pub use battery::BatteryIndicator;
pub use circular_gauge::CircularGauge;
pub use meter::Meter;
pub use signal::SignalStrength;
pub use speedometer::Speedometer;
pub use types::{MeterOrientation, MeterSize, MeterVariant};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meter_percentage() {
        let meter = Meter::new("m", 50.0).min(0.0).max(100.0);
        assert_eq!(meter.get_percentage(), 0.5);

        let meter2 = Meter::new("m2", 75.0).min(50.0).max(100.0);
        assert_eq!(meter2.get_percentage(), 0.5);
    }

    #[test]
    fn test_meter_sizes() {
        let small = Meter::new("s", 50.0).size(MeterSize::Small);
        let medium = Meter::new("m", 50.0).size(MeterSize::Medium);
        let large = Meter::new("l", 50.0).size(MeterSize::Large);

        assert_eq!(small.size, MeterSize::Small);
        assert_eq!(medium.size, MeterSize::Medium);
        assert_eq!(large.size, MeterSize::Large);
    }

    #[test]
    fn test_meter_variants() {
        let default = Meter::new("d", 50.0).variant(MeterVariant::Default);
        let success = Meter::new("s", 50.0).variant(MeterVariant::Success);
        let danger = Meter::new("dn", 50.0).variant(MeterVariant::Danger);

        assert_eq!(default.variant, MeterVariant::Default);
        assert_eq!(success.variant, MeterVariant::Success);
        assert_eq!(danger.variant, MeterVariant::Danger);
    }

    #[test]
    fn test_battery_level_clamping() {
        let low = BatteryIndicator::new("l", -10.0);
        let high = BatteryIndicator::new("h", 150.0);

        assert_eq!(low.level, 0.0);
        assert_eq!(high.level, 100.0);
    }

    #[test]
    fn test_signal_strength() {
        let signal = SignalStrength::new("s", 3).max_bars(4);

        assert_eq!(signal.bars, 3);
        assert_eq!(signal.max_bars, 4);
    }
}
