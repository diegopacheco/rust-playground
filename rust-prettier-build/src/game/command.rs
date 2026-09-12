//! What a player can ask the game to do.

use crate::geometry::Spin;

/// A single player intent, already decoded from whatever key produced it.
///
/// Input and engine meet here and nowhere else: the engine never sees a key
/// code, and the key map never sees the board. That is what lets the whole
/// engine be driven from a test by pushing actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Move the falling piece one column left.
    MoveLeft,
    /// Move the falling piece one column right.
    MoveRight,
    /// Turn the falling piece a quarter turn.
    Rotate(Spin),
    /// Step the falling piece down one row by hand.
    SoftDrop,
    /// Slam the falling piece to its landing position and lock it.
    HardDrop,
    /// Suspend or resume play.
    TogglePause,
    /// Abandon the current game and start a fresh one.
    Restart,
    /// Leave the game.
    Quit,
}

impl Action {
    /// Whether this action only makes sense while a piece is falling.
    ///
    /// Pause, restart and quit must keep working when play is suspended or the
    /// game is over, otherwise a finished game would trap the player.
    #[must_use]
    pub const fn needs_active_piece(self) -> bool {
        matches!(
            self,
            Self::MoveLeft | Self::MoveRight | Self::Rotate(_) | Self::SoftDrop | Self::HardDrop
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_actions_require_a_falling_piece() {
        for action in [
            Action::MoveLeft,
            Action::MoveRight,
            Action::SoftDrop,
            Action::HardDrop,
            Action::Rotate(Spin::Clockwise),
        ] {
            assert!(action.needs_active_piece(), "{action:?}");
        }
    }

    #[test]
    fn quitting_and_restarting_work_even_when_no_piece_is_falling() {
        for action in [Action::Quit, Action::Restart, Action::TogglePause] {
            assert!(!action.needs_active_piece(), "{action:?}");
        }
    }

    #[test]
    fn the_two_rotation_directions_are_distinct_actions() {
        assert_ne!(
            Action::Rotate(Spin::Clockwise),
            Action::Rotate(Spin::CounterClockwise)
        );
    }
}
