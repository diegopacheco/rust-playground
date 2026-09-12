//! Colours, independent of any terminal library.

use crate::piece::Tetromino;

/// A colour a glyph can be drawn in.
///
/// Deliberately a small closed set rather than the terminal library's own type,
/// so views can be written and tested without a terminal anywhere in sight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color {
    /// The terminal's own foreground colour.
    #[default]
    Default,
    /// Dimmed, for scaffolding such as the ghost piece.
    Dim,
    /// Cyan, the line piece.
    Cyan,
    /// Yellow, the square.
    Yellow,
    /// Magenta, the T piece.
    Magenta,
    /// Green, the S piece.
    Green,
    /// Red, the Z piece.
    Red,
    /// Blue, the J piece.
    Blue,
    /// White, the L piece.
    White,
}

impl Color {
    /// The colour conventionally used for `piece`.
    #[must_use]
    pub const fn of(piece: Tetromino) -> Self {
        match piece {
            Tetromino::I => Self::Cyan,
            Tetromino::O => Self::Yellow,
            Tetromino::T => Self::Magenta,
            Tetromino::S => Self::Green,
            Tetromino::Z => Self::Red,
            Tetromino::J => Self::Blue,
            Tetromino::L => Self::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_tetromino_has_its_own_colour_so_the_stack_stays_readable() {
        let colors: HashSet<Color> = Tetromino::ALL.iter().map(|&p| Color::of(p)).collect();
        assert_eq!(colors.len(), Tetromino::ALL.len());
    }

    #[test]
    fn no_piece_is_drawn_in_the_dim_colour_reserved_for_the_ghost() {
        assert!(Tetromino::ALL.iter().all(|&p| Color::of(p) != Color::Dim));
    }

    #[test]
    fn no_piece_borrows_the_plain_default_colour_used_for_chrome() {
        assert!(
            Tetromino::ALL
                .iter()
                .all(|&p| Color::of(p) != Color::Default)
        );
    }

    #[test]
    fn the_mirror_pieces_are_told_apart_by_colour() {
        assert_ne!(Color::of(Tetromino::S), Color::of(Tetromino::Z));
    }
}
