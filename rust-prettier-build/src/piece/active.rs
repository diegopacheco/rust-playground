//! A tetromino placed on the playfield, with pure moves that return candidates.

use crate::geometry::{Offset, Point, Rotation, Spin};
use crate::piece::kind::Tetromino;
use crate::piece::shape::Shape;

/// A tetromino at a concrete position and orientation.
///
/// Every move returns a *new* piece instead of mutating this one. The engine
/// builds a candidate, asks the board whether it fits, and keeps it only if it
/// does — so an illegal move costs nothing to try and can never leave a piece
/// half-moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    /// Which tetromino this is.
    pub kind: Tetromino,
    /// Top-left corner of the piece's bounding box, in playfield coordinates.
    pub origin: Point,
    /// The current orientation.
    pub rotation: Rotation,
}

impl ActivePiece {
    /// Creates a piece at `origin` in its spawn orientation.
    #[must_use]
    pub const fn new(kind: Tetromino, origin: Point) -> Self {
        Self {
            kind,
            origin,
            rotation: Rotation::Spawn,
        }
    }

    /// The four playfield cells this piece currently occupies.
    #[must_use]
    pub fn cells(&self) -> [Point; 4] {
        self.kind.cells_at(self.origin, self.rotation)
    }

    /// The same piece translated by `offset`.
    #[must_use]
    pub const fn shifted(&self, offset: Offset) -> Self {
        Self {
            origin: self.origin.shifted(offset),
            ..*self
        }
    }

    /// The same piece turned one quarter in the given direction.
    #[must_use]
    pub const fn spun(&self, spin: Spin) -> Self {
        Self {
            rotation: spin.apply(self.rotation),
            ..*self
        }
    }

    /// The lowest row this piece occupies.
    #[must_use]
    pub fn lowest_row(&self) -> i16 {
        self.cells()
            .iter()
            .map(|c| c.y)
            .max()
            .unwrap_or(self.origin.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t_piece() -> ActivePiece {
        ActivePiece::new(Tetromino::T, Point::new(3, 0))
    }

    #[test]
    fn a_new_piece_starts_in_spawn_orientation() {
        assert_eq!(t_piece().rotation, Rotation::Spawn);
    }

    #[test]
    fn moving_leaves_the_original_untouched_so_a_rejected_move_costs_nothing() {
        let piece = t_piece();
        let candidate = piece.shifted(Offset::LEFT);
        assert_eq!(piece.origin, Point::new(3, 0));
        assert_ne!(candidate.origin, piece.origin);
    }

    #[test]
    fn shifting_translates_every_cell_by_the_same_amount() {
        let piece = t_piece();
        let moved = piece.shifted(Offset::new(2, 3));
        for (before, after) in piece.cells().into_iter().zip(moved.cells()) {
            assert_eq!(after - before, Offset::new(2, 3));
        }
    }

    #[test]
    fn spinning_changes_orientation_without_moving_the_bounding_box() {
        let piece = t_piece();
        let spun = piece.spun(Spin::Clockwise);
        assert_eq!(spun.origin, piece.origin);
        assert_ne!(spun.cells(), piece.cells());
    }

    #[test]
    fn a_spin_and_its_reverse_restore_the_exact_footprint() {
        let piece = t_piece();
        let round_trip = piece.spun(Spin::Clockwise).spun(Spin::CounterClockwise);
        assert_eq!(round_trip, piece);
    }

    #[test]
    fn lowest_row_tracks_the_bottom_of_the_footprint_not_the_origin() {
        let piece = ActivePiece::new(Tetromino::T, Point::new(0, 0));
        assert_eq!(piece.lowest_row(), 1);
    }

    #[test]
    fn a_piece_always_reports_four_cells() {
        for kind in Tetromino::ALL {
            for rotation in Rotation::ALL {
                let piece = ActivePiece {
                    kind,
                    origin: Point::new(4, 2),
                    rotation,
                };
                assert_eq!(piece.cells().len(), 4);
            }
        }
    }
}
