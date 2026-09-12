//! The playfield itself: stack, ghost and falling piece.

use crate::board::Playfield;
use crate::game::Snapshot;
use crate::geometry::Point;
use crate::render::canvas::{Canvas, Glyph};
use crate::render::chrome::draw_box;
use crate::render::color::Color;
use crate::render::view::View;

/// Draws the playfield inside a titled box.
///
/// Each board square is drawn two characters wide, because terminal cells are
/// roughly twice as tall as they are wide and a one-character square renders a
/// visibly squashed board.
#[derive(Debug, Clone, Copy)]
pub struct BoardPanel {
    /// Column of the panel's top-left corner.
    pub x: u16,
    /// Row of the panel's top-left corner.
    pub y: u16,
}

impl BoardPanel {
    /// How many characters wide one board square is drawn.
    pub const CELL_WIDTH: u16 = 2;
    const FILLED: char = '█';
    const GHOST: char = '░';

    /// Creates a panel anchored at `(x, y)`.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// The total width this panel occupies for a board `columns` wide.
    #[must_use]
    pub const fn width_for(columns: u16) -> u16 {
        columns * Self::CELL_WIDTH + 2
    }

    /// The total height this panel occupies for a board `rows` tall.
    #[must_use]
    pub const fn height_for(rows: u16) -> u16 {
        rows + 2
    }

    fn put_cell(&self, canvas: &mut dyn Canvas, at: Point, glyph: Glyph) {
        let Ok(column) = u16::try_from(at.x) else {
            return;
        };
        let Ok(row) = u16::try_from(at.y) else {
            return;
        };
        let left = self.x + 1 + column * Self::CELL_WIDTH;
        let top = self.y + 1 + row;
        for offset in 0..Self::CELL_WIDTH {
            canvas.put(left + offset, top, glyph);
        }
    }
}

impl View for BoardPanel {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        let board = &snapshot.board;
        let columns = u16::try_from(board.width()).unwrap_or(0);
        let rows = u16::try_from(board.height()).unwrap_or(0);
        draw_box(
            canvas,
            self.x,
            self.y,
            columns * Self::CELL_WIDTH,
            rows,
            "TETRIS",
        );

        for y in 0..board.height() {
            for x in 0..board.width() {
                let at = Point::new(x, y);
                let glyph = match board
                    .cell(at)
                    .and_then(super::super::board::cell::Cell::tetromino)
                {
                    Some(kind) => Glyph::new(Self::FILLED, Color::of(kind)),
                    None => Glyph::BLANK,
                };
                self.put_cell(canvas, at, glyph);
            }
        }

        if snapshot.ghost_is_visible() {
            for at in snapshot.ghost.cells() {
                self.put_cell(canvas, at, Glyph::new(Self::GHOST, Color::Dim));
            }
        }

        let color = Color::of(snapshot.current.kind);
        for at in snapshot.current.cells() {
            self.put_cell(canvas, at, Glyph::new(Self::FILLED, color));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Cell;
    use crate::piece::Tetromino;
    use crate::render::canvas::TextCanvas;
    use crate::test_support::sample_snapshot;

    fn painted(snapshot: &Snapshot) -> TextCanvas {
        let mut canvas = TextCanvas::new(40, 30);
        BoardPanel::new(0, 0).paint(snapshot, &mut canvas);
        canvas
    }

    #[test]
    fn the_panel_is_wide_enough_for_two_characters_per_board_column() {
        assert_eq!(BoardPanel::width_for(10), 22);
        assert_eq!(BoardPanel::height_for(20), 22);
    }

    #[test]
    fn the_playfield_is_drawn_inside_a_titled_border() {
        let canvas = painted(&sample_snapshot());
        assert!(canvas.row(0).contains("TETRIS"));
        assert_eq!(canvas.glyph(0, 0).map(|g| g.ch), Some('┌'));
    }

    #[test]
    fn the_falling_piece_appears_on_the_board() {
        let snapshot = sample_snapshot();
        let canvas = painted(&snapshot);
        for cell in snapshot.current.cells() {
            let x = 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
            let y = 1 + cell.y as u16;
            assert_eq!(canvas.glyph(x, y).map(|g| g.ch), Some(BoardPanel::FILLED));
        }
    }

    #[test]
    fn the_falling_piece_is_drawn_in_its_own_colour() {
        let snapshot = sample_snapshot();
        let canvas = painted(&snapshot);
        let cell = snapshot.current.cells()[0];
        let x = 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
        assert_eq!(
            canvas.glyph(x, 1 + cell.y as u16).map(|g| g.color),
            Some(Color::of(snapshot.current.kind))
        );
    }

    #[test]
    fn the_ghost_is_drawn_dimly_so_it_is_never_mistaken_for_the_piece() {
        let snapshot = sample_snapshot();
        let canvas = painted(&snapshot);
        let cell = snapshot.ghost.cells()[0];
        let x = 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
        let glyph = canvas.glyph(x, 1 + cell.y as u16).unwrap();
        assert_eq!(glyph.ch, BoardPanel::GHOST);
        assert_eq!(glyph.color, Color::Dim);
    }

    #[test]
    fn the_falling_piece_is_drawn_over_the_ghost_where_they_overlap() {
        let mut snapshot = sample_snapshot();
        snapshot.ghost = snapshot.current;
        let canvas = painted(&snapshot);
        let cell = snapshot.current.cells()[0];
        let x = 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
        assert_eq!(
            canvas.glyph(x, 1 + cell.y as u16).map(|g| g.ch),
            Some(BoardPanel::FILLED)
        );
    }

    #[test]
    fn locked_cells_keep_the_colour_of_the_piece_that_made_them() {
        let mut snapshot = sample_snapshot();
        snapshot
            .board
            .set(Point::new(0, 19), Cell::Filled(Tetromino::Z));
        let canvas = painted(&snapshot);
        assert_eq!(
            canvas.glyph(1, 20).map(|g| g.color),
            Some(Color::of(Tetromino::Z))
        );
    }

    #[test]
    fn empty_squares_are_left_blank_so_the_stack_stands_out() {
        let snapshot = sample_snapshot();
        let canvas = painted(&snapshot);
        assert_eq!(canvas.glyph(1, 20).map(|g| g.ch), Some(' '));
    }

    #[test]
    fn the_panel_honours_its_offset_and_draws_nothing_above_it() {
        let mut canvas = TextCanvas::new(40, 30);
        BoardPanel::new(3, 2).paint(&sample_snapshot(), &mut canvas);
        assert_eq!(canvas.row(0).trim(), "");
        assert_eq!(canvas.glyph(3, 2).map(|g| g.ch), Some('┌'));
    }
}
