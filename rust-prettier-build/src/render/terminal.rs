//! A canvas backed by a real terminal.

use crate::render::canvas::{Canvas, Glyph};
use crate::render::color::Color;
use crossterm::style::Color as TermColor;
use crossterm::{cursor, queue, style, terminal};
use std::io::{self, Write};

/// Draws onto a terminal, writing only the cells that actually changed.
///
/// A double buffer is not an optimisation detail here: repainting every cell of
/// the screen each frame makes the board flicker and the cursor crawl, which is
/// exactly the artefact a TUI is supposed to avoid.
#[derive(Debug)]
pub struct TerminalCanvas<W: Write> {
    out: W,
    width: u16,
    height: u16,
    back: Vec<Glyph>,
    front: Vec<Glyph>,
}

impl<W: Write> TerminalCanvas<W> {
    /// Creates a canvas of the given size writing to `out`.
    ///
    /// # Panics
    ///
    /// Panics if either dimension is zero.
    pub fn new(out: W, width: u16, height: u16) -> Self {
        assert!(
            width > 0 && height > 0,
            "terminal must have positive dimensions"
        );
        let cells = usize::from(width) * usize::from(height);
        Self {
            out,
            width,
            height,
            back: vec![Glyph::BLANK; cells],
            front: vec![Glyph::BLANK; cells],
        }
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        (x < self.width && y < self.height)
            .then(|| usize::from(y) * usize::from(self.width) + usize::from(x))
    }

    /// Writes every changed cell to the terminal.
    ///
    /// # Errors
    ///
    /// Returns any error produced by the underlying writer.
    pub fn flush(&mut self) -> io::Result<()> {
        let mut last_color = None;
        for index in 0..self.back.len() {
            if self.back[index] == self.front[index] {
                continue;
            }
            let glyph = self.back[index];
            let width = usize::from(self.width);
            let x = u16::try_from(index % width).unwrap_or(0);
            let y = u16::try_from(index / width).unwrap_or(0);
            queue!(self.out, cursor::MoveTo(x, y))?;
            if last_color != Some(glyph.color) {
                queue!(self.out, style::SetForegroundColor(term_color(glyph.color)))?;
                last_color = Some(glyph.color);
            }
            queue!(self.out, style::Print(glyph.ch))?;
            self.front[index] = glyph;
        }
        queue!(self.out, style::SetForegroundColor(TermColor::Reset))?;
        self.out.flush()
    }

    /// Forgets what is on screen, so the next flush repaints everything.
    pub fn invalidate(&mut self) {
        self.front.fill(Glyph::new('\u{0}', Color::Default));
    }

    /// Resizes the canvas, discarding its contents.
    pub fn resize(&mut self, width: u16, height: u16) {
        if width == 0 || height == 0 || (width == self.width && height == self.height) {
            return;
        }
        self.width = width;
        self.height = height;
        let cells = usize::from(width) * usize::from(height);
        self.back = vec![Glyph::BLANK; cells];
        self.front = vec![Glyph::new('\u{0}', Color::Default); cells];
    }
}

impl<W: Write> Canvas for TerminalCanvas<W> {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn put(&mut self, x: u16, y: u16, glyph: Glyph) {
        if let Some(index) = self.index(x, y) {
            self.back[index] = glyph;
        }
    }

    fn clear(&mut self) {
        self.back.fill(Glyph::BLANK);
    }

    fn present(&mut self) -> io::Result<()> {
        self.flush()
    }
}

const fn term_color(color: Color) -> TermColor {
    match color {
        Color::Default => TermColor::Reset,
        Color::Dim => TermColor::DarkGrey,
        Color::Cyan => TermColor::Cyan,
        Color::Yellow => TermColor::Yellow,
        Color::Magenta => TermColor::Magenta,
        Color::Green => TermColor::Green,
        Color::Red => TermColor::Red,
        Color::Blue => TermColor::Blue,
        Color::White => TermColor::White,
    }
}

/// Puts the terminal into raw, full-screen mode and restores it on drop.
///
/// Restoration happens in [`Drop`] rather than at the end of the game loop
/// because a panic must not leave the user with an invisible cursor and a
/// terminal that no longer echoes what they type.
#[derive(Debug)]
pub struct TerminalGuard;

impl TerminalGuard {
    /// Enters raw mode and the alternate screen.
    ///
    /// # Errors
    ///
    /// Returns any error reported by the terminal.
    pub fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut out = io::stdout();
        queue!(
            out,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide
        )?;
        out.flush()?;
        Ok(Self)
    }

    /// The terminal's current size, as `(columns, rows)`.
    ///
    /// # Errors
    ///
    /// Returns any error reported by the terminal.
    pub fn size() -> io::Result<(u16, u16)> {
        terminal::size()
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        let _ = queue!(
            out,
            style::SetForegroundColor(TermColor::Reset),
            cursor::Show,
            terminal::LeaveAlternateScreen
        );
        let _ = out.flush();
        let _ = terminal::disable_raw_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canvas() -> TerminalCanvas<Vec<u8>> {
        TerminalCanvas::new(Vec::new(), 10, 4)
    }

    #[test]
    fn a_fresh_canvas_matches_the_blank_screen_and_writes_almost_nothing() {
        let mut canvas = canvas();
        canvas.flush().unwrap();
        assert!(canvas.out.len() < 32);
    }

    #[test]
    fn a_changed_cell_is_written_out() {
        let mut canvas = canvas();
        canvas.write(0, 0, "HI", Color::Cyan);
        canvas.flush().unwrap();
        let written = String::from_utf8_lossy(&canvas.out).to_string();
        assert!(written.contains('H') && written.contains('I'));
    }

    #[test]
    fn flushing_twice_without_changes_sends_no_new_cells() {
        let mut canvas = canvas();
        canvas.write(0, 0, "HI", Color::Cyan);
        canvas.flush().unwrap();
        let after_first = canvas.out.len();
        canvas.flush().unwrap();
        assert!(canvas.out.len() - after_first < 32);
    }

    #[test]
    fn only_the_cells_that_changed_are_redrawn_so_the_board_does_not_flicker() {
        let mut canvas = canvas();
        canvas.write(0, 0, "HELLO", Color::Default);
        canvas.flush().unwrap();
        canvas.out.clear();
        canvas.put(0, 0, Glyph::new('J', Color::Default));
        canvas.flush().unwrap();
        let written = String::from_utf8_lossy(&canvas.out).to_string();
        assert!(written.contains('J'));
        assert!(!written.contains('E'), "an unchanged cell was repainted");
    }

    #[test]
    fn invalidating_forces_a_full_repaint_after_a_resize_or_redraw() {
        let mut canvas = canvas();
        canvas.write(0, 0, "HI", Color::Default);
        canvas.flush().unwrap();
        canvas.out.clear();
        canvas.invalidate();
        canvas.flush().unwrap();
        assert!(String::from_utf8_lossy(&canvas.out).contains('H'));
    }

    #[test]
    fn writing_outside_the_canvas_is_ignored_rather_than_panicking() {
        let mut canvas = canvas();
        canvas.put(500, 500, Glyph::new('x', Color::Red));
        canvas.flush().unwrap();
        assert!(!String::from_utf8_lossy(&canvas.out).contains('x'));
    }

    #[test]
    fn resizing_adopts_the_new_dimensions() {
        let mut canvas = canvas();
        canvas.resize(20, 8);
        assert_eq!((canvas.width(), canvas.height()), (20, 8));
    }

    #[test]
    fn a_degenerate_resize_is_ignored_so_the_canvas_stays_usable() {
        let mut canvas = canvas();
        canvas.resize(0, 0);
        assert_eq!((canvas.width(), canvas.height()), (10, 4));
    }

    #[test]
    fn every_colour_maps_to_a_distinct_terminal_colour() {
        use std::collections::HashSet;
        let colors = [
            Color::Cyan,
            Color::Yellow,
            Color::Magenta,
            Color::Green,
            Color::Red,
            Color::Blue,
            Color::White,
        ];
        let mapped: HashSet<String> = colors
            .iter()
            .map(|&c| format!("{:?}", term_color(c)))
            .collect();
        assert_eq!(mapped.len(), colors.len());
    }

    #[test]
    #[should_panic(expected = "positive dimensions")]
    fn a_zero_sized_terminal_is_rejected_loudly() {
        let _ = TerminalCanvas::new(Vec::new(), 0, 10);
    }
}
