//! Running totals and the queue of upcoming pieces.

use crate::game::Snapshot;
use crate::render::canvas::{Canvas, Glyph};
use crate::render::chrome::draw_box;
use crate::render::color::Color;
use crate::render::view::View;

/// Draws the score, the progress totals and the next pieces.
#[derive(Debug, Clone, Copy)]
pub struct SidebarPanel {
    /// Column of the panel's top-left corner.
    pub x: u16,
    /// Row of the panel's top-left corner.
    pub y: u16,
}

impl SidebarPanel {
    /// Interior width of the panel, in characters.
    pub const INNER_WIDTH: u16 = 12;
    /// Total width the panel occupies, borders included.
    pub const WIDTH: u16 = Self::INNER_WIDTH + 2;
    /// How many upcoming pieces are listed.
    pub const PREVIEW: usize = 5;

    /// Creates a panel anchored at `(x, y)`.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn stat(&self, canvas: &mut dyn Canvas, row: u16, label: &str, value: u32) {
        canvas.write(self.x + 2, self.y + 1 + row, label, Color::Dim);
        let text = value.to_string();
        let right = Self::INNER_WIDTH + 1;
        let column = self.x + right.saturating_sub(u16::try_from(text.len()).unwrap_or(right));
        canvas.write(column, self.y + 2 + row, &text, Color::Default);
    }
}

impl View for SidebarPanel {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        let stats_height = 8;
        draw_box(
            canvas,
            self.x,
            self.y,
            Self::INNER_WIDTH,
            stats_height,
            "STATS",
        );
        self.stat(canvas, 0, "SCORE", snapshot.stats.score);
        self.stat(canvas, 2, "LINES", snapshot.stats.lines);
        self.stat(canvas, 4, "LEVEL", snapshot.stats.level);
        self.stat(canvas, 6, "PIECES", snapshot.stats.pieces);

        let next_y = self.y + stats_height + 2;
        let preview = snapshot.next.len().min(Self::PREVIEW);
        let next_height = u16::try_from(preview).unwrap_or(0).max(1);
        draw_box(
            canvas,
            self.x,
            next_y,
            Self::INNER_WIDTH,
            next_height,
            "NEXT",
        );
        for (index, &kind) in snapshot.next.iter().take(Self::PREVIEW).enumerate() {
            let Ok(row) = u16::try_from(index) else {
                break;
            };
            let color = Color::of(kind);
            canvas.put(self.x + 3, next_y + 1 + row, Glyph::new('█', color));
            canvas.put(self.x + 4, next_y + 1 + row, Glyph::new('█', color));
            canvas.write(
                self.x + 6,
                next_y + 1 + row,
                &kind.letter().to_string(),
                color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Tetromino;
    use crate::render::canvas::TextCanvas;
    use crate::test_support::sample_snapshot;

    fn painted(snapshot: &Snapshot) -> TextCanvas {
        let mut canvas = TextCanvas::new(30, 30);
        SidebarPanel::new(0, 0).paint(snapshot, &mut canvas);
        canvas
    }

    #[test]
    fn every_running_total_is_labelled_on_screen() {
        let text = painted(&sample_snapshot()).to_text();
        for label in ["SCORE", "LINES", "LEVEL", "PIECES"] {
            assert!(text.contains(label), "{label} was not drawn");
        }
    }

    #[test]
    fn the_score_shown_is_the_score_in_the_snapshot() {
        let mut snapshot = sample_snapshot();
        snapshot.stats.score = 12_345;
        assert!(painted(&snapshot).to_text().contains("12345"));
    }

    #[test]
    fn a_changed_total_changes_what_is_drawn() {
        let before = painted(&sample_snapshot()).to_text();
        let mut snapshot = sample_snapshot();
        snapshot.stats.lines = 7;
        assert_ne!(painted(&snapshot).to_text(), before);
    }

    #[test]
    fn the_upcoming_pieces_are_listed_by_letter() {
        let snapshot = sample_snapshot();
        let text = painted(&snapshot).to_text();
        for kind in snapshot.next.iter().take(SidebarPanel::PREVIEW) {
            assert!(
                text.contains(kind.letter()),
                "{kind:?} missing from preview"
            );
        }
    }

    #[test]
    fn each_upcoming_piece_is_swatched_in_its_own_colour() {
        let mut snapshot = sample_snapshot();
        snapshot.next = vec![Tetromino::Z];
        let canvas = painted(&snapshot);
        assert_eq!(
            canvas.glyph(3, 11).map(|g| g.color),
            Some(Color::of(Tetromino::Z))
        );
    }

    #[test]
    fn the_preview_is_capped_so_a_long_queue_cannot_overflow_the_panel() {
        let mut snapshot = sample_snapshot();
        snapshot.next = vec![Tetromino::Z; 50];
        let canvas = painted(&snapshot);
        let drawn = (0..30).filter(|&y| canvas.row(y).contains('Z')).count();
        assert!(drawn <= SidebarPanel::PREVIEW);
    }

    #[test]
    fn an_empty_queue_still_draws_a_next_box_rather_than_collapsing() {
        let mut snapshot = sample_snapshot();
        snapshot.next.clear();
        assert!(painted(&snapshot).to_text().contains("NEXT"));
    }

    #[test]
    fn the_panel_fits_inside_its_declared_width() {
        let canvas = painted(&sample_snapshot());
        for y in 0..20 {
            let row = canvas.row(y);
            let used = row.trim_end().chars().count();
            assert!(
                used <= usize::from(SidebarPanel::WIDTH),
                "row {y} used {used} columns"
            );
        }
    }
}
