//! What a single playfield square can hold.

use crate::piece::Tetromino;

/// The contents of one playfield square.
///
/// A filled cell remembers which tetromino locked there. That is not
/// decoration: it is the only thing the renderer needs to colour the stack, so
/// no parallel colour grid has to be kept in sync with the occupancy grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    /// Nothing here; a piece may move through it.
    #[default]
    Empty,
    /// Occupied by a locked piece of the given kind.
    Filled(Tetromino),
}

impl Cell {
    /// Whether a piece may occupy this square.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Whether a locked piece occupies this square.
    #[must_use]
    pub const fn is_filled(self) -> bool {
        !self.is_empty()
    }

    /// The tetromino locked here, if any.
    #[must_use]
    pub const fn tetromino(self) -> Option<Tetromino> {
        match self {
            Self::Empty => None,
            Self::Filled(kind) => Some(kind),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_cell_is_empty_so_a_new_board_is_playable() {
        assert!(Cell::default().is_empty());
    }

    #[test]
    fn empty_and_filled_are_exact_opposites() {
        for cell in [Cell::Empty, Cell::Filled(Tetromino::I)] {
            assert_ne!(cell.is_empty(), cell.is_filled());
        }
    }

    #[test]
    fn a_filled_cell_remembers_its_piece_so_the_stack_keeps_its_colour() {
        assert_eq!(Cell::Filled(Tetromino::Z).tetromino(), Some(Tetromino::Z));
    }

    #[test]
    fn an_empty_cell_has_no_colour_to_report() {
        assert_eq!(Cell::Empty.tetromino(), None);
    }
}
