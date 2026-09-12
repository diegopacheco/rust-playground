//! The visual layer, asserted character by character on an in-memory canvas.

use tetris::board::{Board, Cell, Playfield};
use tetris::game::{Action, Engine, Phase, Snapshot};
use tetris::geometry::Point;
use tetris::piece::Tetromino;
use tetris::render::{BoardPanel, Canvas, Color, Screen, TextCanvas, View};
use tetris::rules::StandardRules;

fn snapshot(seed: u64) -> Snapshot {
    Engine::new(Board::standard(), StandardRules::seeded(seed)).snapshot()
}

fn render(snapshot: &Snapshot) -> TextCanvas {
    let columns = u16::try_from(snapshot.board.width()).unwrap();
    let rows = u16::try_from(snapshot.board.height()).unwrap();
    let (width, height) = Screen::required_size(columns, rows);
    let mut canvas = TextCanvas::new(width, height);
    Screen::for_board(columns, rows).paint(snapshot, &mut canvas);
    canvas
}

#[test]
fn a_fresh_game_draws_every_panel() {
    let text = render(&snapshot(1)).to_text();
    for marker in ["TETRIS", "STATS", "NEXT", "SCORE", "LINES", "LEVEL", "quit"] {
        assert!(text.contains(marker), "{marker} is missing");
    }
}

#[test]
fn the_same_snapshot_always_draws_the_same_screen() {
    assert_eq!(
        render(&snapshot(9)).to_text(),
        render(&snapshot(9)).to_text()
    );
}

#[test]
fn the_screen_never_writes_outside_the_size_it_asked_for() {
    let canvas = render(&snapshot(4));
    for y in 0..canvas.height() {
        assert!(
            canvas.row(y).chars().count() <= usize::from(canvas.width()),
            "row {y} overflowed"
        );
    }
}

#[test]
fn a_standard_game_fits_an_eighty_by_twenty_four_terminal() {
    let (width, height) = Screen::required_size(10, 20);
    assert!(width <= 80 && height <= 24, "needs {width}x{height}");
}

#[test]
fn the_falling_piece_is_visible_on_the_board() {
    let shot = snapshot(6);
    let canvas = render(&shot);
    let drawn = shot.current.cells().iter().all(|cell| {
        let x = Screen::MARGIN + 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
        let y = Screen::MARGIN + 1 + cell.y as u16;
        canvas.glyph(x, y).is_some_and(|g| g.ch == '█')
    });
    assert!(drawn, "the falling piece was not drawn");
}

#[test]
fn each_tetromino_is_drawn_in_its_own_colour() {
    let mut shot = snapshot(2);
    for kind in Tetromino::ALL {
        shot.board.set(Point::new(0, 19), Cell::Filled(kind));
        let canvas = render(&shot);
        let x = Screen::MARGIN + 1;
        let y = Screen::MARGIN + 1 + 19;
        assert_eq!(canvas.glyph(x, y).map(|g| g.color), Some(Color::of(kind)));
    }
}

#[test]
fn a_locked_stack_shows_up_where_it_was_placed() {
    let mut engine = Engine::new(Board::standard(), StandardRules::seeded(8));
    engine.apply(Action::HardDrop);
    let shot = engine.snapshot();
    let canvas = render(&shot);
    let bottom_row = Screen::MARGIN + 1 + (shot.board.height() as u16) - 1;
    assert!(
        canvas.row(bottom_row).contains('█'),
        "the stack was not drawn"
    );
}

#[test]
fn the_score_on_screen_matches_the_score_in_the_game() {
    let mut engine = Engine::new(Board::standard(), StandardRules::seeded(12));
    for _ in 0..5 {
        engine.apply(Action::HardDrop);
    }
    let shot = engine.snapshot();
    assert!(shot.stats.score > 0);
    assert!(
        render(&shot)
            .to_text()
            .contains(&shot.stats.score.to_string())
    );
}

#[test]
fn pausing_puts_a_banner_over_the_board() {
    let mut shot = snapshot(3);
    assert!(!render(&shot).to_text().contains("PAUSED"));
    shot.phase = Phase::Paused;
    assert!(render(&shot).to_text().contains("PAUSED"));
}

#[test]
fn a_finished_game_says_so_on_screen() {
    let mut shot = snapshot(3);
    shot.phase = Phase::Over;
    let text = render(&shot).to_text();
    assert!(text.contains("GAME OVER"));
    assert!(text.contains("restart"));
}

#[test]
fn the_ghost_is_drawn_dimly_and_never_confused_with_the_piece() {
    let shot = snapshot(5);
    let canvas = render(&shot);
    let cell = shot.ghost.cells()[0];
    let x = Screen::MARGIN + 1 + (cell.x as u16) * BoardPanel::CELL_WIDTH;
    let y = Screen::MARGIN + 1 + cell.y as u16;
    let glyph = canvas.glyph(x, y).unwrap();
    assert_eq!(glyph.color, Color::Dim);
    assert_ne!(glyph.color, Color::of(shot.current.kind));
}

#[test]
fn a_non_standard_board_is_laid_out_without_clipping() {
    for (columns, rows) in [(6, 12), (14, 24), (4, 6)] {
        let engine = Engine::new(Board::new(columns, rows), StandardRules::seeded(1));
        let canvas = render(&engine.snapshot());
        assert!(
            canvas.to_text().contains("TETRIS"),
            "{columns}x{rows} clipped"
        );
    }
}

#[test]
fn the_upcoming_pieces_are_listed_in_the_order_they_arrive() {
    let shot = snapshot(15);
    let text = render(&shot).to_text();
    for kind in shot.next.iter().take(5) {
        assert!(
            text.contains(kind.letter()),
            "{kind:?} missing from the queue"
        );
    }
}
