//! Where elapsed time comes from.

use std::time::{Duration, Instant};

/// A source of elapsed time.
///
/// The engine measures gravity in durations, never by calling the system clock
/// itself. A test can therefore advance time by an exact amount and assert on
/// the frame that follows, instead of sleeping and hoping.
pub trait Clock {
    /// The time elapsed since the previous call, zero on the first.
    fn tick(&mut self) -> Duration;
}

/// A clock reading the operating system's monotonic timer.
#[derive(Debug)]
pub struct SystemClock {
    last: Instant,
}

impl SystemClock {
    /// Starts a clock measuring from now.
    #[must_use]
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last);
        self.last = now;
        elapsed
    }
}

/// A clock that only moves when a test tells it to.
#[derive(Debug, Default, Clone)]
pub struct ManualClock {
    pending: Duration,
}

impl ManualClock {
    /// Creates a clock sitting at zero.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pending: Duration::ZERO,
        }
    }

    /// Queues `amount` to be reported by the next [`Clock::tick`].
    pub const fn advance(&mut self, amount: Duration) {
        self.pending = self.pending.saturating_add(amount);
    }
}

impl Clock for ManualClock {
    fn tick(&mut self) -> Duration {
        std::mem::replace(&mut self.pending, Duration::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manual_clock_reports_exactly_what_was_queued() {
        let mut clock = ManualClock::new();
        clock.advance(Duration::from_millis(250));
        assert_eq!(clock.tick(), Duration::from_millis(250));
    }

    #[test]
    fn time_is_consumed_once_so_a_single_advance_cannot_drop_a_piece_twice() {
        let mut clock = ManualClock::new();
        clock.advance(Duration::from_millis(100));
        clock.tick();
        assert_eq!(clock.tick(), Duration::ZERO);
    }

    #[test]
    fn advances_accumulate_until_they_are_read() {
        let mut clock = ManualClock::new();
        clock.advance(Duration::from_millis(30));
        clock.advance(Duration::from_millis(70));
        assert_eq!(clock.tick(), Duration::from_millis(100));
    }

    #[test]
    fn an_untouched_manual_clock_never_advances_the_game() {
        assert_eq!(ManualClock::new().tick(), Duration::ZERO);
    }

    #[test]
    fn the_system_clock_moves_forward_on_its_own() {
        let mut clock = SystemClock::new();
        clock.tick();
        std::thread::sleep(Duration::from_millis(5));
        assert!(clock.tick() >= Duration::from_millis(4));
    }
}
