//! Integer lattice points and translations on the playfield.

use std::ops::{Add, Neg, Sub};

/// A signed cell coordinate on the playfield.
///
/// `x` grows to the right, `y` grows downward, matching the way a terminal
/// addresses its rows. Coordinates are deliberately signed: a piece may sit
/// partly above the visible field while it spawns, and rotation kicks probe
/// positions that are briefly out of bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Point {
    /// Column index, increasing rightward.
    pub x: i16,
    /// Row index, increasing downward.
    pub y: i16,
}

impl Point {
    /// The origin, `(0, 0)`.
    pub const ORIGIN: Self = Self::new(0, 0);

    /// Builds a point from a column and a row.
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Returns this point shifted by `offset`.
    #[must_use]
    pub const fn shifted(self, offset: Offset) -> Self {
        Self::new(self.x + offset.dx, self.y + offset.dy)
    }
}

impl Add<Offset> for Point {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self {
        self.shifted(rhs)
    }
}

impl Sub for Point {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Offset {
        Offset::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// A relative displacement between two [`Point`]s.
///
/// Kept distinct from `Point` so that the type system rejects adding two
/// absolute positions together, which is never meaningful here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Offset {
    /// Horizontal displacement, positive rightward.
    pub dx: i16,
    /// Vertical displacement, positive downward.
    pub dy: i16,
}

impl Offset {
    /// No displacement at all.
    pub const ZERO: Self = Self::new(0, 0);
    /// One cell to the left.
    pub const LEFT: Self = Self::new(-1, 0);
    /// One cell to the right.
    pub const RIGHT: Self = Self::new(1, 0);
    /// One cell downward.
    pub const DOWN: Self = Self::new(0, 1);

    /// Builds an offset from its two components.
    #[must_use]
    pub const fn new(dx: i16, dy: i16) -> Self {
        Self { dx, dy }
    }
}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.dx, -self.dy)
    }
}

impl Add for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifting_moves_by_the_offset_components() {
        assert_eq!(
            Point::new(3, 4).shifted(Offset::new(-1, 2)),
            Point::new(2, 6)
        );
    }

    #[test]
    fn shifting_by_zero_is_identity_so_a_no_op_command_cannot_move_a_piece() {
        let p = Point::new(7, 2);
        assert_eq!(p.shifted(Offset::ZERO), p);
    }

    #[test]
    fn subtracting_points_yields_the_offset_that_reconstructs_the_first() {
        let (a, b) = (Point::new(9, 1), Point::new(4, 6));
        assert_eq!(b + (a - b), a);
    }

    #[test]
    fn left_and_right_cancel_so_a_rejected_shift_leaves_no_drift() {
        assert_eq!(Offset::LEFT + Offset::RIGHT, Offset::ZERO);
    }

    #[test]
    fn negating_an_offset_undoes_it() {
        let p = Point::new(5, 5);
        assert_eq!(p.shifted(Offset::DOWN).shifted(-Offset::DOWN), p);
    }

    #[test]
    fn coordinates_are_signed_so_pieces_may_sit_above_the_field_while_spawning() {
        let above = Point::new(4, 0).shifted(Offset::new(0, -2));
        assert_eq!(above.y, -2);
    }
}
