//! An immutable picture of the game, handed to the renderer.

use crate::board::Board;
use crate::game::state::{Phase, Stats};
use crate::piece::{ActivePiece, Tetromino};

/// Everything a renderer needs, and nothing it can change.
///
/// The engine is generic over its rule set; this is not. Taking a snapshot once
/// per frame keeps every view free of those generics, and guarantees a frame is
/// drawn from one consistent moment rather than from a game still moving.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    /// The locked stack.
    pub board: Board,
    /// The piece currently falling.
    pub current: ActivePiece,
    /// Where that piece would land if dropped now.
    pub ghost: ActivePiece,
    /// Upcoming pieces, soonest first.
    pub next: Vec<Tetromino>,
    /// Running totals.
    pub stats: Stats,
    /// What the game is doing.
    pub phase: Phase,
}

impl Snapshot {
    /// Whether the ghost sits somewhere the falling piece does not already cover.
    #[must_use]
    pub fn ghost_is_visible(&self) -> bool {
        self.ghost.origin != self.current.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Playfield;
    use crate::game::Engine;
    use crate::geometry::Offset;
    use crate::rules::StandardRules;

    fn snapshot() -> Snapshot {
        Engine::new(Board::standard(), StandardRules::seeded(4)).snapshot()
    }

    #[test]
    fn a_snapshot_reports_the_same_piece_the_engine_is_holding() {
        let engine = Engine::new(Board::standard(), StandardRules::seeded(9));
        assert_eq!(snapshot_of(&engine).current, *engine.current());
    }

    fn snapshot_of<R: crate::rules::RuleSet>(engine: &Engine<R>) -> Snapshot {
        engine.snapshot()
    }

    #[test]
    fn the_snapshot_carries_a_full_board_the_renderer_can_walk() {
        let shot = snapshot();
        assert_eq!(shot.board.width(), Board::STANDARD_WIDTH);
        assert_eq!(shot.board.height(), Board::STANDARD_HEIGHT);
    }

    #[test]
    fn a_fresh_snapshot_shows_a_playable_game() {
        let shot = snapshot();
        assert_eq!(shot.phase, Phase::Playing);
        assert_eq!(shot.stats, Stats::default());
    }

    #[test]
    fn the_preview_is_populated_so_the_sidebar_has_something_to_draw() {
        assert!(!snapshot().next.is_empty());
    }

    #[test]
    fn the_ghost_is_worth_drawing_when_it_sits_below_the_falling_piece() {
        let shot = snapshot();
        assert!(shot.ghost_is_visible());
        assert!(shot.ghost.origin.y > shot.current.origin.y);
    }

    #[test]
    fn the_ghost_is_hidden_once_the_piece_has_reached_its_landing_spot() {
        let mut shot = snapshot();
        shot.current = shot.ghost;
        assert!(!shot.ghost_is_visible());
    }

    #[test]
    fn a_snapshot_does_not_change_when_the_game_moves_on() {
        let mut engine = Engine::new(Board::standard(), StandardRules::seeded(1));
        let before = engine.snapshot();
        engine.apply(crate::game::Action::MoveLeft);
        assert_eq!(
            before.current.origin,
            engine.current().origin.shifted(Offset::RIGHT)
        );
    }
}
