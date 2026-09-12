//! The [`Shape`] abstraction: anything with a four-cell, rotatable footprint.

use crate::geometry::{Offset, Point, Rotation};

/// A rotatable footprint made of exactly four cells.
///
/// Implementors describe their cells in local coordinates inside a square
/// bounding box, which is what makes rotation a pure function of the piece
/// rather than of the board it sits on. The rest of the engine depends on this
/// trait and never on a concrete tetromino, so alternative piece sets — or the
/// fixtures used in tests — drop in without touching collision or scoring.
pub trait Shape {
    /// The side length of the square this shape rotates inside.
    fn bounding_box(&self) -> i16;

    /// The four cells this shape occupies at `rotation`, as offsets from the
    /// top-left corner of its bounding box.
    fn cells(&self, rotation: Rotation) -> [Offset; 4];

    /// The four absolute cells this shape occupies when its bounding box is
    /// anchored at `origin`.
    fn cells_at(&self, origin: Point, rotation: Rotation) -> [Point; 4] {
        self.cells(rotation).map(|offset| origin.shifted(offset))
    }

    /// The number of columns actually covered at `rotation`, ignoring the empty
    /// margins of the bounding box.
    fn width(&self, rotation: Rotation) -> i16 {
        let cells = self.cells(rotation);
        let min = cells.iter().map(|c| c.dx).min().unwrap_or(0);
        let max = cells.iter().map(|c| c.dx).max().unwrap_or(0);
        max - min + 1
    }

    /// The offset of the leftmost occupied column at `rotation`.
    fn left_margin(&self, rotation: Rotation) -> i16 {
        self.cells(rotation).iter().map(|c| c.dx).min().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Domino;

    impl Shape for Domino {
        fn bounding_box(&self) -> i16 {
            4
        }

        fn cells(&self, _rotation: Rotation) -> [Offset; 4] {
            [
                Offset::new(1, 0),
                Offset::new(2, 0),
                Offset::new(1, 1),
                Offset::new(2, 1),
            ]
        }
    }

    #[test]
    fn cells_at_anchors_the_footprint_to_an_absolute_origin() {
        let placed = Domino.cells_at(Point::new(10, 5), Rotation::Spawn);
        assert!(placed.contains(&Point::new(11, 5)));
        assert!(placed.contains(&Point::new(12, 6)));
    }

    #[test]
    fn cells_at_preserves_the_four_cell_count() {
        assert_eq!(Domino.cells_at(Point::new(-3, -3), Rotation::Half).len(), 4);
    }

    #[test]
    fn width_ignores_empty_margins_so_wall_checks_are_not_too_strict() {
        assert_eq!(Domino.width(Rotation::Spawn), 2);
        assert_ne!(Domino.width(Rotation::Spawn), Domino.bounding_box());
    }

    #[test]
    fn left_margin_reports_the_first_occupied_column() {
        assert_eq!(Domino.left_margin(Rotation::Spawn), 1);
    }

    #[test]
    fn a_shape_implementor_needs_only_two_methods_for_the_engine_to_use_it() {
        let origin = Point::new(0, 0);
        assert_eq!(Domino.cells_at(origin, Rotation::Left).len(), 4);
        assert_eq!(Domino.width(Rotation::Left), 2);
    }
}
