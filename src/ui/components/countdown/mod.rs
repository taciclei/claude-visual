//! Countdown components - Timers and countdowns
//!
//! Provides countdown timer and clock display components.

mod types;
mod countdown;
mod timer;
mod clock;
mod pomodoro;

pub use types::{CountdownSize, CountdownVariant, TimeRemaining};
pub use countdown::Countdown;
pub use timer::Timer;
pub use clock::Clock;
pub use pomodoro::PomodoroTimer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_remaining() {
        let time = TimeRemaining::new(1, 2, 30, 45);
        assert_eq!(time.days, 1);
        assert_eq!(time.hours, 2);
        assert_eq!(time.minutes, 30);
        assert_eq!(time.seconds, 45);
    }

    #[test]
    fn test_time_remaining_from_seconds() {
        let time = TimeRemaining::from_seconds(90061); // 1 day, 1 hour, 1 minute, 1 second
        assert_eq!(time.days, 1);
        assert_eq!(time.hours, 1);
        assert_eq!(time.minutes, 1);
        assert_eq!(time.seconds, 1);
    }

    #[test]
    fn test_time_remaining_is_zero() {
        let zero = TimeRemaining::new(0, 0, 0, 0);
        let not_zero = TimeRemaining::new(0, 0, 0, 1);

        assert!(zero.is_zero());
        assert!(!not_zero.is_zero());
    }

    #[test]
    fn test_countdown_sizes() {
        let small = Countdown::from_seconds("s", 3600).size(CountdownSize::Small);
        let medium = Countdown::from_seconds("m", 3600).size(CountdownSize::Medium);
        let large = Countdown::from_seconds("l", 3600).size(CountdownSize::Large);

        assert_eq!(small.size, CountdownSize::Small);
        assert_eq!(medium.size, CountdownSize::Medium);
        assert_eq!(large.size, CountdownSize::Large);
    }

    #[test]
    fn test_countdown_variants() {
        let default = Countdown::from_seconds("d", 3600).variant(CountdownVariant::Default);
        let boxed = Countdown::from_seconds("b", 3600).variant(CountdownVariant::Boxed);
        let circular = Countdown::from_seconds("c", 3600).variant(CountdownVariant::Circular);

        assert_eq!(default.variant, CountdownVariant::Default);
        assert_eq!(boxed.variant, CountdownVariant::Boxed);
        assert_eq!(circular.variant, CountdownVariant::Circular);
    }

    #[test]
    fn test_clock_12_hour() {
        let clock = Clock::new("c", 14, 30).use_12_hour(true);
        assert!(clock.use_12_hour);
    }

    #[test]
    fn test_pomodoro_session() {
        let pomodoro = PomodoroTimer::new("p", 25, 0)
            .session(2, 4)
            .is_break(false);

        assert_eq!(pomodoro.session_number, 2);
        assert_eq!(pomodoro.total_sessions, 4);
        assert!(!pomodoro.is_break);
    }
}
