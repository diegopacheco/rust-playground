//! Borders and labels shared by the panels.

use crate::render::canvas::{Canvas, Glyph};
use crate::render::color::Color;

/// Draws a single-line box whose interior is `width` by `height`.
///
/// The box is drawn *around* the interior, so a caller positions content at
/// `(x + 1, y + 1)` and never has to reason about where the border went.
pub fn draw_box(canvas: &mut dyn Canvas, x: u16, y: u16, width: u16, height: u16, title: &str) {
    let right = x.saturating_add(width).saturating_add(1);
    let bottom = y.saturating_add(height).saturating_add(1);
    let edge = |canvas: &mut dyn Canvas, x: u16, y: u16, ch: char| {
        canvas.put(x, y, Glyph::new(ch, Color::Dim));
    };

    edge(canvas, x, y, '┌');
    edge(canvas, right, y, '┐');
    edge(canvas, x, bottom, '└');
    edge(canvas, right, bottom, '┘');
    for column in (x + 1)..right {
        edge(canvas, column, y, '─');
        edge(canvas, column, bottom, '─');
    }
    for row in (y + 1)..bottom {
        edge(canvas, x, row, '│');
        edge(canvas, right, row, '│');
    }
    if !title.is_empty() {
        canvas.write(x + 2, y, title, Color::Default);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::canvas::TextCanvas;

    #[test]
    fn a_box_surrounds_an_interior_of_the_requested_size() {
        let mut canvas = TextCanvas::new(6, 4);
        draw_box(&mut canvas, 0, 0, 4, 2, "");
        assert_eq!(canvas.row(0), "┌────┐");
        assert_eq!(canvas.row(1), "│    │");
        assert_eq!(canvas.row(3), "└────┘");
    }

    #[test]
    fn the_interior_is_left_blank_for_the_caller_to_fill() {
        let mut canvas = TextCanvas::new(6, 4);
        draw_box(&mut canvas, 0, 0, 4, 2, "");
        for x in 1..5 {
            assert_eq!(canvas.glyph(x, 1).map(|g| g.ch), Some(' '));
        }
    }

    #[test]
    fn a_title_is_written_into_the_top_border() {
        let mut canvas = TextCanvas::new(10, 3);
        draw_box(&mut canvas, 0, 0, 8, 1, "NEXT");
        assert!(canvas.row(0).contains("NEXT"));
    }

    #[test]
    fn a_box_drawn_at_an_offset_leaves_the_space_before_it_untouched() {
        let mut canvas = TextCanvas::new(8, 4);
        draw_box(&mut canvas, 2, 1, 4, 1, "");
        assert_eq!(canvas.row(0), "        ");
        assert_eq!(canvas.row(1), "  ┌────┐");
    }

    #[test]
    fn drawing_past_the_edge_is_clipped_rather_than_panicking() {
        let mut canvas = TextCanvas::new(4, 2);
        draw_box(&mut canvas, 0, 0, 100, 100, "TITLE");
        assert_eq!(canvas.row(0).chars().next(), Some('┌'));
    }
}
