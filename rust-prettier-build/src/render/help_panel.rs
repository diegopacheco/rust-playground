//! The key hints shown beneath the playfield.

use crate::game::Snapshot;
use crate::render::canvas::Canvas;
use crate::render::color::Color;
use crate::render::view::View;

/// Draws a one-line reminder of the controls.
#[derive(Debug, Clone, Copy)]
pub struct HelpPanel {
    /// Column of the hint line.
    pub x: u16,
    /// Row of the hint line.
    pub y: u16,
}

impl HelpPanel {
    /// The hint text, kept in one place so it cannot drift from the key map.
    pub const HINT: &'static str =
        "←→ move  ↑/z turn  ↓ soft  space drop  p pause  r restart  q quit";

    /// Creates a hint line at `(x, y)`.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// How many columns the hint needs.
    #[must_use]
    pub fn width() -> u16 {
        u16::try_from(Self::HINT.chars().count()).unwrap_or(u16::MAX)
    }
}

impl View for HelpPanel {
    fn paint(&self, _snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        canvas.write(self.x, self.y, Self::HINT, Color::Dim);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::canvas::TextCanvas;
    use crate::test_support::sample_snapshot;

    fn painted() -> TextCanvas {
        let mut canvas = TextCanvas::new(HelpPanel::width() + 4, 2);
        HelpPanel::new(0, 0).paint(&sample_snapshot(), &mut canvas);
        canvas
    }

    #[test]
    fn every_bound_key_is_mentioned_so_the_player_is_never_stuck() {
        let text = painted().to_text();
        for hint in ["move", "turn", "drop", "pause", "restart", "quit"] {
            assert!(text.contains(hint), "{hint} is not documented on screen");
        }
    }

    #[test]
    fn the_hints_are_dimmed_so_they_do_not_compete_with_the_board() {
        assert_eq!(painted().glyph(0, 0).map(|g| g.color), Some(Color::Dim));
    }

    #[test]
    fn the_declared_width_matches_what_is_actually_drawn() {
        let canvas = painted();
        let used = u16::try_from(canvas.row(0).trim_end().chars().count()).unwrap();
        assert_eq!(used, HelpPanel::width());
    }

    #[test]
    fn the_hints_do_not_depend_on_the_state_of_the_game() {
        let mut canvas = TextCanvas::new(HelpPanel::width() + 4, 2);
        let mut snapshot = sample_snapshot();
        snapshot.stats.score = 999;
        snapshot.phase = crate::game::Phase::Over;
        HelpPanel::new(0, 0).paint(&snapshot, &mut canvas);
        assert_eq!(canvas.to_text(), painted().to_text());
    }
}
