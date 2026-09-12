//! The [`Playfield`] trait and the [`Board`] that implements it.

use crate::board::cell::Cell;
use crate::geometry::{Offset, Point};
use crate::piece::ActivePiece;

/// Read-only queries every collision check needs.
///
/// Movement, hard drop and the ghost piece are written against this trait, so
/// they can be exercised against a hand-built fixture board without a running
/// game, and an alternative field (wider, taller, pre-filled) needs no changes
/// to the rules that use it.
pub trait Playfield {
    /// Number of columns.
    fn width(&self) -> i16;

    /// Number of rows.
    fn height(&self) -> i16;

    /// The contents of `at`, or [`None`] when it lies outside the field.
    fn cell(&self, at: Point) -> Option<Cell>;

    /// Whether `at` lies inside the field.
    fn in_bounds(&self, at: Point) -> bool {
        (0..self.width()).contains(&at.x) && (0..self.height()).contains(&at.y)
    }

    /// Whether `at` is inside the field and unoccupied.
    fn is_free(&self, at: Point) -> bool {
        self.cell(at).is_some_and(Cell::is_empty)
    }

    /// Whether `piece` may legally occupy its current position.
    fn accepts(&self, piece: &ActivePiece) -> bool {
        piece.cells().iter().all(|&cell| self.is_free(cell))
    }

    /// The same piece dropped as far as it will go without colliding.
    fn landed(&self, piece: &ActivePiece) -> ActivePiece {
        let mut resting = *piece;
        loop {
            let next = resting.shifted(Offset::DOWN);
            if self.accepts(&next) {
                resting = next;
            } else {
                return resting;
            }
        }
    }
}

/// A rectangular playfield holding locked cells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: i16,
    height: i16,
    cells: Vec<Cell>,
}

impl Board {
    /// The column count of a standard game.
    pub const STANDARD_WIDTH: i16 = 10;
    /// The row count of a standard game.
    pub const STANDARD_HEIGHT: i16 = 20;

    /// Creates an empty board.
    ///
    /// # Panics
    ///
    /// Panics if either dimension is not positive; a board with no cells has no
    /// meaningful behaviour and every caller passes a constant.
    #[must_use]
    pub fn new(width: i16, height: i16) -> Self {
        assert!(
            width > 0 && height > 0,
            "board must have positive dimensions"
        );
        Self {
            width,
            height,
            cells: vec![Cell::Empty; Self::area(width, height)],
        }
    }

    /// Creates a board with the standard ten-by-twenty dimensions.
    #[must_use]
    pub fn standard() -> Self {
        Self::new(Self::STANDARD_WIDTH, Self::STANDARD_HEIGHT)
    }

    fn area(width: i16, height: i16) -> usize {
        usize::from(width.unsigned_abs()) * usize::from(height.unsigned_abs())
    }

    fn index(&self, at: Point) -> Option<usize> {
        self.in_bounds(at).then(|| {
            usize::from(at.y.unsigned_abs()) * usize::from(self.width.unsigned_abs())
                + usize::from(at.x.unsigned_abs())
        })
    }

    /// Writes `cell` at `at`, ignoring positions outside the field.
    pub fn set(&mut self, at: Point, cell: Cell) {
        if let Some(index) = self.index(at) {
            self.cells[index] = cell;
        }
    }

    /// Locks `piece` into the stack.
    pub fn lock(&mut self, piece: &ActivePiece) {
        for cell in piece.cells() {
            self.set(cell, Cell::Filled(piece.kind));
        }
    }

    /// Whether every square in `row` is occupied.
    #[must_use]
    pub fn is_row_full(&self, row: i16) -> bool {
        (0..self.width).all(|x| !self.is_free(Point::new(x, row)))
            && (0..self.width).all(|x| self.in_bounds(Point::new(x, row)))
    }

    /// Whether `row` holds nothing at all.
    #[must_use]
    pub fn is_row_empty(&self, row: i16) -> bool {
        (0..self.width).all(|x| self.is_free(Point::new(x, row)))
    }

    /// Removes every full row, collapsing the rows above downward.
    ///
    /// Returns the rows that were removed, topmost first.
    pub fn clear_full_rows(&mut self) -> Vec<i16> {
        let cleared: Vec<i16> = (0..self.height)
            .filter(|&row| self.is_row_full(row))
            .collect();
        if cleared.is_empty() {
            return cleared;
        }
        let survivors: Vec<Cell> = (0..self.height)
            .filter(|row| !cleared.contains(row))
            .flat_map(|row| self.row_cells(row))
            .collect();
        let blank = vec![Cell::Empty; cleared.len() * usize::from(self.width.unsigned_abs())];
        self.cells = blank.into_iter().chain(survivors).collect();
        cleared
    }

    fn row_cells(&self, row: i16) -> Vec<Cell> {
        (0..self.width)
            .map(|x| self.cell(Point::new(x, row)).unwrap_or_default())
            .collect()
    }

    /// The highest occupied row, or [`None`] when the board is empty.
    #[must_use]
    pub fn stack_top(&self) -> Option<i16> {
        (0..self.height).find(|&row| !self.is_row_empty(row))
    }
}

impl Playfield for Board {
    fn width(&self) -> i16 {
        self.width
    }

    fn height(&self) -> i16 {
        self.height
    }

    fn cell(&self, at: Point) -> Option<Cell> {
        self.index(at).map(|index| self.cells[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::piece::Tetromino;

    fn fill_row(board: &mut Board, row: i16, kind: Tetromino) {
        for x in 0..board.width() {
            board.set(Point::new(x, row), Cell::Filled(kind));
        }
    }

    #[test]
    fn a_new_board_is_entirely_empty_so_the_first_piece_can_always_spawn() {
        let board = Board::standard();
        assert!((0..board.height()).all(|row| board.is_row_empty(row)));
    }

    #[test]
    fn cells_outside_the_field_read_as_none_rather_than_wrapping_around() {
        let board = Board::new(4, 4);
        assert_eq!(board.cell(Point::new(-1, 0)), None);
        assert_eq!(board.cell(Point::new(4, 0)), None);
        assert_eq!(board.cell(Point::new(0, 4)), None);
    }

    #[test]
    fn out_of_bounds_is_never_free_so_walls_and_floor_block_movement() {
        let board = Board::new(4, 4);
        assert!(!board.is_free(Point::new(-1, 2)));
        assert!(!board.is_free(Point::new(0, 4)));
    }

    #[test]
    fn locking_a_piece_records_its_kind_in_every_cell_it_covered() {
        let mut board = Board::standard();
        let piece = ActivePiece::new(Tetromino::T, Point::new(3, 0));
        board.lock(&piece);
        for cell in piece.cells() {
            assert_eq!(board.cell(cell), Some(Cell::Filled(Tetromino::T)));
        }
    }

    #[test]
    fn a_locked_piece_blocks_a_later_one_from_overlapping_it() {
        let mut board = Board::standard();
        let piece = ActivePiece::new(Tetromino::O, Point::new(3, 0));
        board.lock(&piece);
        assert!(!board.accepts(&piece));
    }

    #[test]
    fn a_row_is_full_only_when_every_column_is_occupied() {
        let mut board = Board::new(4, 4);
        fill_row(&mut board, 3, Tetromino::I);
        assert!(board.is_row_full(3));
        board.set(Point::new(2, 3), Cell::Empty);
        assert!(!board.is_row_full(3));
    }

    #[test]
    fn clearing_removes_the_full_row_and_reports_it() {
        let mut board = Board::new(4, 4);
        fill_row(&mut board, 3, Tetromino::I);
        assert_eq!(board.clear_full_rows(), vec![3]);
        assert!(board.is_row_empty(3));
    }

    #[test]
    fn clearing_collapses_the_stack_downward_instead_of_leaving_a_hole() {
        let mut board = Board::new(4, 4);
        board.set(Point::new(1, 2), Cell::Filled(Tetromino::S));
        fill_row(&mut board, 3, Tetromino::I);
        board.clear_full_rows();
        assert_eq!(
            board.cell(Point::new(1, 3)),
            Some(Cell::Filled(Tetromino::S))
        );
        assert!(board.is_row_empty(2));
    }

    #[test]
    fn four_simultaneous_rows_all_clear_in_one_pass() {
        let mut board = Board::new(4, 8);
        for row in 4..8 {
            fill_row(&mut board, row, Tetromino::I);
        }
        assert_eq!(board.clear_full_rows(), vec![4, 5, 6, 7]);
        assert!((0..8).all(|row| board.is_row_empty(row)));
    }

    #[test]
    fn non_adjacent_rows_clear_together_without_disturbing_the_gap_between() {
        let mut board = Board::new(4, 6);
        fill_row(&mut board, 3, Tetromino::I);
        fill_row(&mut board, 5, Tetromino::I);
        board.set(Point::new(0, 4), Cell::Filled(Tetromino::L));
        assert_eq!(board.clear_full_rows(), vec![3, 5]);
        assert_eq!(
            board.cell(Point::new(0, 5)),
            Some(Cell::Filled(Tetromino::L))
        );
    }

    #[test]
    fn clearing_nothing_leaves_the_board_byte_for_byte_identical() {
        let mut board = Board::new(4, 4);
        board.set(Point::new(0, 0), Cell::Filled(Tetromino::J));
        let before = board.clone();
        assert!(board.clear_full_rows().is_empty());
        assert_eq!(board, before);
    }

    #[test]
    fn landed_drops_a_piece_to_the_floor_when_the_column_is_clear() {
        let board = Board::new(4, 6);
        let piece = ActivePiece::new(Tetromino::O, Point::new(0, 0));
        assert_eq!(board.landed(&piece).lowest_row(), board.height() - 1);
    }

    #[test]
    fn landed_stops_on_top_of_the_stack_rather_than_passing_through_it() {
        let mut board = Board::new(4, 6);
        fill_row(&mut board, 5, Tetromino::I);
        let piece = ActivePiece::new(Tetromino::O, Point::new(0, 0));
        assert_eq!(board.landed(&piece).lowest_row(), 4);
    }

    #[test]
    fn landed_is_idempotent_so_a_hard_drop_cannot_be_repeated_for_extra_ground() {
        let board = Board::new(4, 6);
        let piece = ActivePiece::new(Tetromino::T, Point::new(0, 0));
        let once = board.landed(&piece);
        assert_eq!(board.landed(&once), once);
    }

    #[test]
    fn stack_top_reports_the_highest_occupied_row() {
        let mut board = Board::new(4, 6);
        assert_eq!(board.stack_top(), None);
        board.set(Point::new(2, 4), Cell::Filled(Tetromino::Z));
        assert_eq!(board.stack_top(), Some(4));
    }

    #[test]
    #[should_panic(expected = "positive dimensions")]
    fn a_board_with_no_cells_is_rejected_loudly() {
        let _ = Board::new(0, 10);
    }
}
