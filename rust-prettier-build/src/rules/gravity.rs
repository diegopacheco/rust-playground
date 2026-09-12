//! How fast pieces fall.

use std::time::Duration;

/// Maps a level to the delay between automatic downward steps.
///
/// Speed is what turns a solved puzzle into a game, so the curve is a trait:
/// a gentler ramp for practice or a brutal one for experts is a new type, not a
/// patched constant.
pub trait GravityCurve {
    /// The delay between gravity steps at `level`.
    fn interval(&self, level: u32) -> Duration;
}

/// A curve that shortens the interval on every level and then plateaus.
///
/// The floor matters: without it the interval reaches zero and the game stops
/// being playable rather than merely hard.
#[derive(Debug, Clone, Copy)]
pub struct ClassicGravity {
    start: Duration,
    step: Duration,
    floor: Duration,
}

impl ClassicGravity {
    /// Creates a curve from its starting delay, per-level decrement and floor.
    #[must_use]
    pub const fn new(start: Duration, step: Duration, floor: Duration) -> Self {
        Self { start, step, floor }
    }
}

impl Default for ClassicGravity {
    fn default() -> Self {
        Self::new(
            Duration::from_millis(800),
            Duration::from_millis(60),
            Duration::from_millis(60),
        )
    }
}

impl GravityCurve for ClassicGravity {
    fn interval(&self, level: u32) -> Duration {
        self.start
            .checked_sub(self.step.saturating_mul(level))
            .unwrap_or(self.floor)
            .max(self.floor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_level_falls_at_least_as_fast_as_the_one_before() {
        let curve = ClassicGravity::default();
        for level in 0..60 {
            assert!(
                curve.interval(level + 1) <= curve.interval(level),
                "level {level} got slower"
            );
        }
    }

    #[test]
    fn the_early_levels_actually_speed_up_rather_than_sitting_at_one_rate() {
        let curve = ClassicGravity::default();
        assert!(curve.interval(5) < curve.interval(0));
    }

    #[test]
    fn the_interval_never_reaches_zero_so_the_game_stays_playable() {
        let curve = ClassicGravity::default();
        for level in 0..10_000 {
            assert!(
                curve.interval(level) > Duration::ZERO,
                "level {level} froze"
            );
        }
    }

    #[test]
    fn extreme_levels_plateau_at_the_floor_instead_of_underflowing() {
        let curve = ClassicGravity::default();
        assert_eq!(curve.interval(u32::MAX), curve.interval(1_000));
    }

    #[test]
    fn level_zero_uses_the_configured_starting_delay() {
        let curve = ClassicGravity::new(
            Duration::from_millis(500),
            Duration::from_millis(10),
            Duration::from_millis(50),
        );
        assert_eq!(curve.interval(0), Duration::from_millis(500));
    }

    #[test]
    fn a_custom_curve_is_honoured_without_touching_the_default() {
        let gentle = ClassicGravity::new(
            Duration::from_secs(2),
            Duration::from_millis(1),
            Duration::from_millis(500),
        );
        assert!(gentle.interval(3) > ClassicGravity::default().interval(3));
    }
}
