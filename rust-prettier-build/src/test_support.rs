//! Fixtures shared by the unit tests, the integration tests and any consumer
//! writing its own [`crate::render::View`].

use crate::board::Board;
use crate::game::{Engine, Snapshot};
use crate::rules::StandardRules;

/// A game with a reproducible piece sequence.
#[must_use]
pub fn seeded_game(seed: u64) -> Engine<StandardRules<crate::rules::XorShift64>> {
    Engine::new(Board::standard(), StandardRules::seeded(seed))
}

/// A snapshot of a freshly started, reproducible game.
#[must_use]
pub fn sample_snapshot() -> Snapshot {
    seeded_game(2026).snapshot()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Phase;

    #[test]
    fn the_fixture_game_is_ready_to_play() {
        assert_eq!(seeded_game(1).phase(), Phase::Playing);
    }

    #[test]
    fn the_fixture_is_reproducible_so_tests_do_not_flake() {
        assert_eq!(sample_snapshot(), sample_snapshot());
    }
}
