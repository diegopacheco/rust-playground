//! The seven tetrominoes and the geometry that distinguishes them.

use crate::geometry::{Offset, Rotation};
use crate::piece::shape::Shape;

/// One of the seven standard tetrominoes, named after the letter it resembles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tetromino {
    /// The four-in-a-row piece, the only one that clears four rows at once.
    I,
    /// The square, the only piece rotation does not move.
    O,
    /// The three-wide piece with a centre nub.
    T,
    /// The right-handed skew piece.
    S,
    /// The left-handed skew piece.
    Z,
    /// The three-wide piece with a left shoulder.
    J,
    /// The three-wide piece with a right shoulder.
    L,
}

impl Tetromino {
    /// Every tetromino, in the canonical order used to seed a shuffle bag.
    pub const ALL: [Self; 7] = [
        Self::I,
        Self::O,
        Self::T,
        Self::S,
        Self::Z,
        Self::J,
        Self::L,
    ];

    /// The single letter conventionally used to name this piece.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Self::I => 'I',
            Self::O => 'O',
            Self::T => 'T',
            Self::S => 'S',
            Self::Z => 'Z',
            Self::J => 'J',
            Self::L => 'L',
        }
    }

    /// The cells this piece occupies in its spawn orientation, expressed as
    /// offsets inside its own [`Shape::bounding_box`].
    const fn spawn_cells(self) -> [Offset; 4] {
        match self {
            Self::I => [
                Offset::new(0, 1),
                Offset::new(1, 1),
                Offset::new(2, 1),
                Offset::new(3, 1),
            ],
            Self::O => [
                Offset::new(0, 0),
                Offset::new(1, 0),
                Offset::new(0, 1),
                Offset::new(1, 1),
            ],
            Self::T => [
                Offset::new(1, 0),
                Offset::new(0, 1),
                Offset::new(1, 1),
                Offset::new(2, 1),
            ],
            Self::S => [
                Offset::new(1, 0),
                Offset::new(2, 0),
                Offset::new(0, 1),
                Offset::new(1, 1),
            ],
            Self::Z => [
                Offset::new(0, 0),
                Offset::new(1, 0),
                Offset::new(1, 1),
                Offset::new(2, 1),
            ],
            Self::J => [
                Offset::new(0, 0),
                Offset::new(0, 1),
                Offset::new(1, 1),
                Offset::new(2, 1),
            ],
            Self::L => [
                Offset::new(2, 0),
                Offset::new(0, 1),
                Offset::new(1, 1),
                Offset::new(2, 1),
            ],
        }
    }
}

impl Shape for Tetromino {
    fn bounding_box(&self) -> i16 {
        match self {
            Self::I => 4,
            Self::O => 2,
            _ => 3,
        }
    }

    fn cells(&self, rotation: Rotation) -> [Offset; 4] {
        let side = self.bounding_box();
        let mut cells = self.spawn_cells();
        for _ in 0..rotation.quarter_turns() {
            for cell in &mut cells {
                *cell = Offset::new(side - 1 - cell.dy, cell.dx);
            }
        }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn cell_set(piece: Tetromino, rotation: Rotation) -> HashSet<Offset> {
        piece.cells(rotation).into_iter().collect()
    }

    #[test]
    fn every_piece_always_occupies_exactly_four_distinct_cells() {
        for piece in Tetromino::ALL {
            for rotation in Rotation::ALL {
                assert_eq!(
                    cell_set(piece, rotation).len(),
                    4,
                    "{piece:?} at {rotation:?} lost or doubled a cell"
                );
            }
        }
    }

    #[test]
    fn rotation_never_pushes_a_piece_outside_its_own_bounding_box() {
        for piece in Tetromino::ALL {
            let side = piece.bounding_box();
            for rotation in Rotation::ALL {
                for cell in piece.cells(rotation) {
                    assert!(
                        (0..side).contains(&cell.dx) && (0..side).contains(&cell.dy),
                        "{piece:?} at {rotation:?} escaped its {side}x{side} box at {cell:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn four_quarter_turns_restore_the_original_footprint() {
        for piece in Tetromino::ALL {
            assert_eq!(
                cell_set(piece, Rotation::Spawn),
                cell_set(
                    piece,
                    Rotation::Spawn
                        .clockwise()
                        .clockwise()
                        .clockwise()
                        .clockwise()
                ),
            );
        }
    }

    #[test]
    fn the_square_is_the_only_piece_rotation_leaves_untouched() {
        for piece in Tetromino::ALL {
            let unchanged = Rotation::ALL
                .iter()
                .all(|&r| cell_set(piece, r) == cell_set(piece, Rotation::Spawn));
            assert_eq!(
                unchanged,
                piece == Tetromino::O,
                "{piece:?} rotational symmetry is wrong"
            );
        }
    }

    #[test]
    fn the_line_piece_is_the_only_one_spanning_four_columns() {
        for piece in Tetromino::ALL {
            let width = piece
                .cells(Rotation::Spawn)
                .iter()
                .map(|c| c.dx)
                .collect::<HashSet<_>>()
                .len();
            assert_eq!(
                width == 4,
                piece == Tetromino::I,
                "{piece:?} width is wrong"
            );
        }
    }

    #[test]
    fn s_and_z_are_mirror_images_rather_than_the_same_piece() {
        assert_ne!(
            cell_set(Tetromino::S, Rotation::Spawn),
            cell_set(Tetromino::Z, Rotation::Spawn)
        );
    }

    #[test]
    fn every_piece_has_a_distinct_letter_so_the_next_queue_is_readable() {
        let letters: HashSet<char> = Tetromino::ALL.iter().map(|p| p.letter()).collect();
        assert_eq!(letters.len(), Tetromino::ALL.len());
    }
}
