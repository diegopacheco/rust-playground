//! Whole games played through the public API, with no terminal involved.

use std::time::Duration;
use tetris::app::GameLoop;
use tetris::board::{Board, Cell, Playfield};
use tetris::game::{Action, Engine, ManualClock, Phase};
use tetris::geometry::{Point, Spin};
use tetris::input::ScriptedInput;
use tetris::piece::Tetromino;
use tetris::render::{Screen, TextCanvas};
use tetris::rules::{ClassicGravity, GravityCurve, StandardRules, XorShift64};

type Game = Engine<StandardRules<XorShift64>>;

fn game(seed: u64) -> Game {
    Engine::new(Board::standard(), StandardRules::seeded(seed))
}

fn occupied(engine: &Game) -> usize {
    let board = engine.board();
    (0..board.height())
        .flat_map(|y| (0..board.width()).map(move |x| Point::new(x, y)))
        .filter(|&at| board.cell(at).is_some_and(Cell::is_filled))
        .count()
}

#[test]
fn the_same_seed_replays_an_identical_game() {
    let script = [
        Action::MoveLeft,
        Action::Rotate(Spin::Clockwise),
        Action::HardDrop,
        Action::MoveRight,
        Action::HardDrop,
        Action::SoftDrop,
    ];
    let play = |seed| {
        let mut engine = game(seed);
        for action in script {
            engine.apply(action);
        }
        engine.snapshot()
    };
    assert_eq!(play(7), play(7));
    assert_ne!(play(7), play(8));
}

#[test]
fn every_locked_piece_adds_exactly_four_cells_unless_a_row_cleared() {
    let mut engine = game(3);
    for _ in 0..8 {
        let before = occupied(&engine);
        let lines_before = engine.stats().lines;
        engine.apply(Action::HardDrop);
        if engine.stats().lines == lines_before {
            assert_eq!(occupied(&engine), before + 4, "a locked piece lost cells");
        }
    }
}

#[test]
fn pieces_never_overlap_the_stack_however_they_are_moved() {
    let mut engine = game(11);
    let moves = [
        Action::MoveLeft,
        Action::MoveRight,
        Action::Rotate(Spin::Clockwise),
        Action::Rotate(Spin::CounterClockwise),
        Action::SoftDrop,
    ];
    for round in 0..200 {
        engine.apply(moves[round % moves.len()]);
        if engine.phase().is_over() {
            break;
        }
        for cell in engine.current().cells() {
            assert!(
                engine.board().cell(cell).is_some_and(Cell::is_empty),
                "the falling piece overlapped the stack at {cell:?}"
            );
        }
    }
}

#[test]
fn a_piece_always_stays_inside_the_board() {
    let mut engine = game(5);
    for round in 0..300 {
        let action = match round % 4 {
            0 => Action::MoveLeft,
            1 => Action::Rotate(Spin::Clockwise),
            2 => Action::MoveRight,
            _ => Action::SoftDrop,
        };
        engine.apply(action);
        for cell in engine.current().cells() {
            assert!(engine.board().in_bounds(cell), "piece escaped at {cell:?}");
        }
    }
}

#[test]
fn a_game_left_running_eventually_ends_rather_than_looping_forever() {
    let mut engine = game(13);
    let step = ClassicGravity::default().interval(0);
    for _ in 0..20_000 {
        engine.tick(step);
        if engine.phase().is_over() {
            break;
        }
    }
    assert!(engine.phase().is_over(), "the stack never topped out");
    assert!(engine.stats().pieces > 0);
}

#[test]
fn a_finished_game_can_always_be_restarted_into_a_playable_state() {
    let mut engine = game(17);
    for _ in 0..300 {
        engine.apply(Action::HardDrop);
        if engine.phase().is_over() {
            break;
        }
    }
    assert!(engine.phase().is_over());
    engine.apply(Action::Restart);
    assert_eq!(engine.phase(), Phase::Playing);
    assert_eq!(occupied(&engine), 0);
    assert_eq!(engine.stats().score, 0);
}

#[test]
fn clearing_a_row_removes_exactly_one_row_worth_of_cells() {
    let mut engine = Engine::new(Board::new(4, 10), StandardRules::seeded(1));
    let mut cleared_once = false;
    for round in 0..120 {
        if engine.phase().is_over() {
            engine.apply(Action::Restart);
            continue;
        }
        let toward = if round % 2 == 0 {
            Action::MoveLeft
        } else {
            Action::MoveRight
        };
        for _ in 0..4 {
            engine.apply(toward);
        }
        let before = occupied(&engine);
        let lines_before = engine.stats().lines;
        engine.apply(Action::HardDrop);
        let rows = engine.stats().lines - lines_before;
        if rows > 0 {
            cleared_once = true;
            let expected = before + 4 - (rows as usize) * 4;
            assert_eq!(
                occupied(&engine),
                expected,
                "row collapse lost or kept cells"
            );
        }
    }
    assert!(cleared_once, "no row ever cleared while packing both walls");
}

#[test]
fn the_score_only_ever_goes_up() {
    let mut engine = game(23);
    let mut last = 0;
    for round in 0..400 {
        engine.apply(if round % 3 == 0 {
            Action::HardDrop
        } else {
            Action::SoftDrop
        });
        assert!(engine.stats().score >= last, "the score went backwards");
        last = engine.stats().score;
        if engine.phase().is_over() {
            break;
        }
    }
}

#[test]
fn gravity_alone_lands_a_piece_where_the_ghost_said_it_would() {
    let mut engine = game(29);
    let promised = engine.ghost().cells();
    let step = ClassicGravity::default().interval(0);
    for _ in 0..40 {
        if engine.stats().pieces > 0 {
            break;
        }
        engine.tick(step);
    }
    for cell in promised {
        assert!(
            engine.board().cell(cell).is_some_and(Cell::is_filled),
            "the piece did not land where the ghost promised"
        );
    }
}

#[test]
fn a_hard_drop_is_never_worse_than_letting_the_piece_fall() {
    let mut dropped = game(31);
    let mut fallen = game(31);
    dropped.apply(Action::HardDrop);
    let step = ClassicGravity::default().interval(0);
    while fallen.stats().pieces == 0 {
        fallen.tick(step);
    }
    assert!(dropped.stats().score >= fallen.stats().score);
}

#[test]
fn a_whole_game_runs_through_the_loop_without_a_terminal() {
    let (width, height) = Screen::required_size(10, 20);
    let mut script = vec![Action::HardDrop; 500];
    script.push(Action::Quit);
    let mut loop_ = GameLoop::new(
        game(37),
        ScriptedInput::new(script),
        TextCanvas::new(width, height),
        ManualClock::new(),
    );
    loop_.run().expect("the loop should not fail");
    assert!(loop_.engine().stats().pieces > 0);
    assert!(loop_.canvas().to_text().contains("TETRIS"));
}

#[test]
fn every_tetromino_shows_up_over_a_long_game() {
    let mut engine = game(41);
    let mut seen = std::collections::HashSet::new();
    for _ in 0..200 {
        seen.insert(engine.current().kind);
        engine.apply(Action::HardDrop);
        if engine.phase().is_over() {
            engine.apply(Action::Restart);
        }
    }
    assert_eq!(seen.len(), Tetromino::ALL.len());
}

#[test]
fn a_paused_game_survives_a_long_wait_untouched() {
    let mut engine = game(43);
    engine.apply(Action::TogglePause);
    let frozen = engine.snapshot();
    engine.tick(Duration::from_secs(600));
    assert_eq!(engine.snapshot(), frozen);
}
