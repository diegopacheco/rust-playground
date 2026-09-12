//! A surface views draw onto.

use crate::render::color::Color;
use std::io;

/// One character cell with a colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Glyph {
    /// The character to show.
    pub ch: char,
    /// The colour to show it in.
    pub color: Color,
}

impl Glyph {
    /// Builds a glyph.
    #[must_use]
    pub const fn new(ch: char, color: Color) -> Self {
        Self { ch, color }
    }

    /// A blank cell.
    pub const BLANK: Self = Self::new(' ', Color::Default);
}

impl Default for Glyph {
    fn default() -> Self {
        Self::BLANK
    }
}

/// Somewhere glyphs can be written.
///
/// Every view targets this trait, never a terminal. That is what allows a view
/// to be rendered into a plain string in a test and asserted on exactly, which
/// is otherwise the least testable part of a TUI.
pub trait Canvas {
    /// Total columns available.
    fn width(&self) -> u16;

    /// Total rows available.
    fn height(&self) -> u16;

    /// Writes one glyph, ignoring positions outside the surface.
    fn put(&mut self, x: u16, y: u16, glyph: Glyph);

    /// Resets every cell to blank.
    fn clear(&mut self);

    /// Shows everything drawn since the last call.
    ///
    /// An in-memory canvas has nothing to do here; a terminal flushes. Keeping
    /// it on the trait means the game loop never learns which it is holding.
    ///
    /// # Errors
    ///
    /// Returns any error produced while writing to the underlying surface.
    fn present(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Writes `text` running rightward from `(x, y)`.
    fn write(&mut self, x: u16, y: u16, text: &str, color: Color) {
        for (index, ch) in text.chars().enumerate() {
            let Ok(offset) = u16::try_from(index) else {
                return;
            };
            let Some(column) = x.checked_add(offset) else {
                return;
            };
            self.put(column, y, Glyph::new(ch, color));
        }
    }
}

/// A canvas that keeps its glyphs in memory.
///
/// Used by the tests to assert on exactly what a view drew, and by nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCanvas {
    width: u16,
    height: u16,
    cells: Vec<Glyph>,
}

impl TextCanvas {
    /// Creates a blank canvas.
    ///
    /// # Panics
    ///
    /// Panics if either dimension is zero.
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        assert!(
            width > 0 && height > 0,
            "canvas must have positive dimensions"
        );
        Self {
            width,
            height,
            cells: vec![Glyph::BLANK; usize::from(width) * usize::from(height)],
        }
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        (x < self.width && y < self.height)
            .then(|| usize::from(y) * usize::from(self.width) + usize::from(x))
    }

    /// The glyph at `(x, y)`, if it lies on the canvas.
    #[must_use]
    pub fn glyph(&self, x: u16, y: u16) -> Option<Glyph> {
        self.index(x, y).map(|index| self.cells[index])
    }

    /// Row `y` as a string.
    #[must_use]
    pub fn row(&self, y: u16) -> String {
        (0..self.width)
            .filter_map(|x| self.glyph(x, y))
            .map(|glyph| glyph.ch)
            .collect()
    }

    /// The whole canvas as newline-separated rows.
    #[must_use]
    pub fn to_text(&self) -> String {
        (0..self.height)
            .map(|y| self.row(y))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Canvas for TextCanvas {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn put(&mut self, x: u16, y: u16, glyph: Glyph) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = glyph;
        }
    }

    fn clear(&mut self) {
        self.cells.fill(Glyph::BLANK);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_canvas_is_entirely_blank() {
        let canvas = TextCanvas::new(4, 2);
        assert_eq!(canvas.to_text(), "    \n    ");
    }

    #[test]
    fn a_glyph_lands_exactly_where_it_was_put() {
        let mut canvas = TextCanvas::new(4, 2);
        canvas.put(2, 1, Glyph::new('#', Color::Red));
        assert_eq!(canvas.glyph(2, 1), Some(Glyph::new('#', Color::Red)));
        assert_eq!(canvas.row(1), "  # ");
    }

    #[test]
    fn writing_off_the_edge_is_ignored_rather_than_wrapping_to_the_next_row() {
        let mut canvas = TextCanvas::new(4, 2);
        canvas.write(2, 0, "abcdef", Color::Default);
        assert_eq!(canvas.row(0), "  ab");
        assert_eq!(canvas.row(1), "    ");
    }

    #[test]
    fn writing_out_of_bounds_never_panics() {
        let mut canvas = TextCanvas::new(4, 2);
        canvas.put(99, 99, Glyph::new('x', Color::Default));
        canvas.write(u16::MAX - 1, 0, "overflow", Color::Default);
        assert_eq!(canvas.to_text(), "    \n    ");
    }

    #[test]
    fn text_is_written_left_to_right_in_one_colour() {
        let mut canvas = TextCanvas::new(6, 1);
        canvas.write(0, 0, "SCORE", Color::Cyan);
        assert_eq!(canvas.row(0), "SCORE ");
        assert_eq!(canvas.glyph(4, 0).map(|g| g.color), Some(Color::Cyan));
    }

    #[test]
    fn presenting_an_in_memory_canvas_succeeds_and_keeps_its_contents() {
        let mut canvas = TextCanvas::new(3, 1);
        canvas.write(0, 0, "abc", Color::Default);
        canvas.present().unwrap();
        assert_eq!(canvas.row(0), "abc");
    }

    #[test]
    fn clearing_returns_the_canvas_to_blank_so_frames_do_not_smear() {
        let mut canvas = TextCanvas::new(3, 1);
        canvas.write(0, 0, "abc", Color::Default);
        canvas.clear();
        assert_eq!(canvas.to_text(), "   ");
    }

    #[test]
    #[should_panic(expected = "positive dimensions")]
    fn a_canvas_with_no_cells_is_rejected_loudly() {
        let _ = TextCanvas::new(0, 5);
    }
}
