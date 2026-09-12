//! The banner shown when play is suspended or finished.

use crate::game::{Phase, Snapshot};
use crate::render::canvas::Canvas;
use crate::render::color::Color;
use crate::render::view::View;

/// Draws a centred banner over the playfield when the game is not running.
///
/// Nothing is drawn while play continues, so this view can always be composed
/// last without having to ask whether it applies.
#[derive(Debug, Clone, Copy)]
pub struct OverlayPanel {
    /// Column of the region the banner is centred in.
    pub x: u16,
    /// Row of the region the banner is centred in.
    pub y: u16,
    /// Width of that region.
    pub width: u16,
    /// Height of that region.
    pub height: u16,
}

impl OverlayPanel {
    /// Creates a banner centred in the given region.
    #[must_use]
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn message(phase: Phase) -> Option<(&'static str, &'static str, Color)> {
        match phase {
            Phase::Playing => None,
            Phase::Paused => Some(("PAUSED", "p to resume", Color::Yellow)),
            Phase::Over => Some(("GAME OVER", "r to restart", Color::Red)),
        }
    }

    fn centred(&self, text: &str) -> u16 {
        let len = u16::try_from(text.chars().count()).unwrap_or(self.width);
        self.x + self.width.saturating_sub(len) / 2
    }
}

impl View for OverlayPanel {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        let Some((title, hint, color)) = Self::message(snapshot.phase) else {
            return;
        };
        let row = self.y + self.height / 2;
        canvas.write(self.centred(title), row, title, color);
        canvas.write(self.centred(hint), row + 1, hint, Color::Dim);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::canvas::TextCanvas;
    use crate::test_support::sample_snapshot;

    fn painted(phase: Phase) -> TextCanvas {
        let mut snapshot = sample_snapshot();
        snapshot.phase = phase;
        let mut canvas = TextCanvas::new(30, 22);
        OverlayPanel::new(0, 0, 22, 20).paint(&snapshot, &mut canvas);
        canvas
    }

    #[test]
    fn nothing_is_drawn_while_the_game_is_running() {
        assert_eq!(painted(Phase::Playing).to_text().trim(), "");
    }

    #[test]
    fn pausing_says_so_and_tells_the_player_how_to_resume() {
        let text = painted(Phase::Paused).to_text();
        assert!(text.contains("PAUSED"));
        assert!(text.contains("resume"));
    }

    #[test]
    fn a_finished_game_says_so_and_tells_the_player_how_to_start_again() {
        let text = painted(Phase::Over).to_text();
        assert!(text.contains("GAME OVER"));
        assert!(text.contains("restart"));
    }

    #[test]
    fn the_two_banners_are_coloured_differently_so_they_are_not_confused() {
        assert_ne!(
            OverlayPanel::message(Phase::Paused).map(|m| m.2),
            OverlayPanel::message(Phase::Over).map(|m| m.2)
        );
    }

    #[test]
    fn the_banner_is_centred_rather_than_pinned_to_the_left_edge() {
        let canvas = painted(Phase::Paused);
        let row = (0..22).find(|&y| canvas.row(y).contains("PAUSED")).unwrap();
        let text = canvas.row(row);
        let leading = text.len() - text.trim_start().len();
        assert!(leading > 0, "banner was flush left");
    }

    #[test]
    fn the_banner_sits_within_the_region_it_was_given() {
        let canvas = painted(Phase::Over);
        for y in 0..22 {
            let row = canvas.row(y);
            assert!(row.trim_end().chars().count() <= 22, "row {y} overflowed");
        }
    }
}
