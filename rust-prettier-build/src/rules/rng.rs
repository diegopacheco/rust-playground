//! A tiny deterministic random source.

/// A source of pseudo-random numbers.
///
/// The engine depends on this trait rather than on a concrete generator, which
/// is what lets a test pin the exact piece sequence by supplying a scripted
/// source instead of a real one.
pub trait Rng {
    /// Returns the next 64-bit value.
    fn next_u64(&mut self) -> u64;

    /// Returns a value in `0..bound`.
    ///
    /// # Panics
    ///
    /// Panics when `bound` is zero, which is always a caller bug.
    fn next_below(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "bound must be positive");
        usize::try_from(self.next_u64() % bound as u64).unwrap_or(0)
    }
}

/// A xorshift64\* generator.
///
/// Not cryptographic and not meant to be: it is small, has no dependencies and
/// produces a well-distributed piece sequence, which is the whole requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Creates a generator from `seed`.
    ///
    /// A zero seed is replaced, because xorshift is stuck at zero forever.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x2545_F491_4F6C_DD1D
            } else {
                seed
            },
        }
    }

    /// Creates a generator seeded from the system clock.
    #[must_use]
    pub fn from_clock() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0x9E37_79B9_7F4A_7C15, |d| d.as_nanos() as u64);
        Self::new(nanos)
    }
}

impl Rng for XorShift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn the_same_seed_replays_the_same_sequence_so_a_failing_game_is_reproducible() {
        let mut a = XorShift64::new(42);
        let mut b = XorShift64::new(42);
        let left: Vec<u64> = (0..32).map(|_| a.next_u64()).collect();
        let right: Vec<u64> = (0..32).map(|_| b.next_u64()).collect();
        assert_eq!(left, right);
    }

    #[test]
    fn different_seeds_diverge_so_two_games_do_not_play_identically() {
        let mut a = XorShift64::new(1);
        let mut b = XorShift64::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn a_zero_seed_still_produces_varied_output_instead_of_locking_up() {
        let mut rng = XorShift64::new(0);
        let values: HashSet<u64> = (0..16).map(|_| rng.next_u64()).collect();
        assert!(values.len() > 1);
        assert!(!values.contains(&0));
    }

    #[test]
    fn next_below_always_lands_inside_the_requested_range() {
        let mut rng = XorShift64::new(7);
        for _ in 0..500 {
            assert!(rng.next_below(7) < 7);
        }
    }

    #[test]
    fn next_below_eventually_reaches_every_value_so_no_piece_is_unreachable() {
        let mut rng = XorShift64::new(99);
        let seen: HashSet<usize> = (0..500).map(|_| rng.next_below(7)).collect();
        assert_eq!(seen.len(), 7);
    }

    #[test]
    #[should_panic(expected = "bound must be positive")]
    fn a_zero_bound_is_rejected_loudly_rather_than_dividing_by_zero() {
        XorShift64::new(1).next_below(0);
    }
}
