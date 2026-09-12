//! The full game screen, composed from the individual panels.

use crate::game::Snapshot;
use crate::render::board_panel::BoardPanel;
use crate::render::canvas::Canvas;
use crate::render::help_panel::HelpPanel;
use crate::render::overlay_panel::OverlayPanel;
use crate::render::sidebar_panel::SidebarPanel;
use crate::render::view::View;

/// Lays the panels out and paints them in order.
///
/// Layout is computed once from the board dimensions rather than hard-coded, so
/// a wider or taller board needs no changes here; the panels themselves know
/// nothing about each other.
#[derive(Debug, Clone, Copy)]
pub struct Screen {
    board: BoardPanel,
    sidebar: SidebarPanel,
    overlay: OverlayPanel,
    help: HelpPanel,
}

impl Screen {
    /// Blank columns kept around the whole layout.
    pub const MARGIN: u16 = 1;

    /// Builds a layout for a board of `columns` by `rows`.
    #[must_use]
    pub const fn for_board(columns: u16, rows: u16) -> Self {
        let board_width = BoardPanel::width_for(columns);
        let board_height = BoardPanel::height_for(rows);
        let left = Self::MARGIN;
        let top = Self::MARGIN;
        Self {
            board: BoardPanel::new(left, top),
            sidebar: SidebarPanel::new(left + board_width + 1, top),
            overlay: OverlayPanel::new(left + 1, top + 1, columns * BoardPanel::CELL_WIDTH, rows),
            help: HelpPanel::new(left, top + board_height),
        }
    }

    /// The smallest terminal this layout fits in, as `(columns, rows)`.
    #[must_use]
    pub fn required_size(columns: u16, rows: u16) -> (u16, u16) {
        let layout = Self::for_board(columns, rows);
        let right = layout.sidebar.x + SidebarPanel::WIDTH + Self::MARGIN;
        let width = right.max(HelpPanel::width() + Self::MARGIN * 2);
        let height = layout.help.y + 1;
        (width, height)
    }
}

impl View for Screen {
    fn paint(&self, snapshot: &Snapshot, canvas: &mut dyn Canvas) {
        self.board.paint(snapshot, canvas);
        self.sidebar.paint(snapshot, canvas);
        self.help.paint(snapshot, canvas);
        self.overlay.paint(snapshot, canvas);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Playfield;
    use crate::game::Phase;
    use crate::render::canvas::TextCanvas;
    use crate::test_support::sample_snapshot;

    fn painted(snapshot: &Snapshot) -> TextCanvas {
        let columns = u16::try_from(snapshot.board.width()).unwrap();
        let rows = u16::try_from(snapshot.board.height()).unwrap();
        let (width, height) = Screen::required_size(columns, rows);
        let mut canvas = TextCanvas::new(width, height);
        Screen::for_board(columns, rows).paint(snapshot, &mut canvas);
        canvas
    }

    #[test]
    fn the_whole_screen_fits_in_the_size_it_asks_for() {
        let canvas = painted(&sample_snapshot());
        for y in 0..canvas.height() {
            let used = canvas.row(y).trim_end().chars().count();
            assert!(used <= usize::from(canvas.width()), "row {y} overflowed");
        }
    }

    #[test]
    fn a_standard_board_asks_for_a_terminal_most_people_already_have() {
        let (width, height) = Screen::required_size(10, 20);
        assert!(width <= 80, "needs {width} columns");
        assert!(height <= 24, "needs {height} rows");
    }

    #[test]
    fn every_panel_shows_up_on_the_composed_screen() {
        let text = painted(&sample_snapshot()).to_text();
        for marker in ["TETRIS", "STATS", "NEXT", "quit"] {
            assert!(text.contains(marker), "{marker} is missing from the screen");
        }
    }

    #[test]
    fn the_sidebar_sits_beside_the_board_rather_than_on_top_of_it() {
        let layout = Screen::for_board(10, 20);
        assert!(layout.sidebar.x >= layout.board.x + BoardPanel::width_for(10));
    }

    #[test]
    fn the_hints_sit_below_the_board_rather_than_over_it() {
        let layout = Screen::for_board(10, 20);
        assert!(layout.help.y >= layout.board.y + BoardPanel::height_for(20));
    }

    #[test]
    fn the_pause_banner_is_painted_over_the_board_not_hidden_behind_it() {
        let mut snapshot = sample_snapshot();
        snapshot.phase = Phase::Paused;
        assert!(painted(&snapshot).to_text().contains("PAUSED"));
    }

    #[test]
    fn a_larger_board_is_given_a_larger_screen() {
        let (small, _) = Screen::required_size(10, 20);
        let (large, _) = Screen::required_size(30, 20);
        assert!(large > small, "{small} did not grow to {large}");
    }

    #[test]
    fn the_layout_adapts_to_a_non_standard_board_without_clipping() {
        use crate::board::Board;
        use crate::game::Engine;
        use crate::rules::StandardRules;
        let engine = Engine::new(Board::new(6, 12), StandardRules::seeded(1));
        let canvas = painted(&engine.snapshot());
        assert!(canvas.to_text().contains("TETRIS"));
    }
}
