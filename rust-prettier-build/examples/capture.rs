//! Renders a scripted game to ANSI text, used to capture the screenshots in the
//! README. Run with `cargo run --example capture`.

use std::time::Duration;
use tetris::board::Board;
use tetris::game::{Action, Engine, Phase};
use tetris::render::{Canvas, Color, Screen, TextCanvas, View};
use tetris::rules::StandardRules;

fn ansi(color: Color) -> &'static str {
    match color {
        Color::Default => "\x1b[39m",
        Color::Dim => "\x1b[90m",
        Color::Cyan => "\x1b[36m",
        Color::Yellow => "\x1b[33m",
        Color::Magenta => "\x1b[35m",
        Color::Green => "\x1b[32m",
        Color::Red => "\x1b[31m",
        Color::Blue => "\x1b[34m",
        Color::White => "\x1b[37m",
    }
}

fn emit(name: &str, canvas: &TextCanvas) {
    println!("=== {name} ===");
    for y in 0..canvas.height() {
        let mut current = None;
        let mut line = String::new();
        for x in 0..canvas.width() {
            let glyph = canvas.glyph(x, y).unwrap();
            if current != Some(glyph.color) {
                line.push_str(ansi(glyph.color));
                current = Some(glyph.color);
            }
            line.push(glyph.ch);
        }
        line.push_str("\x1b[0m");
        println!("{line}");
    }
}

fn main() {
    let (width, height) = Screen::required_size(10, 20);
    let screen = Screen::for_board(10, 20);
    let mut engine = Engine::new(Board::standard(), StandardRules::seeded(2026));

    let shot = |engine: &Engine<_>, name: &str| {
        let mut canvas = TextCanvas::new(width, height);
        screen.paint(&engine.snapshot(), &mut canvas);
        emit(name, &canvas);
    };

    engine.tick(Duration::from_millis(2400));
    shot(&engine, "playing");

    for round in 0..28 {
        for _ in 0..6 {
            engine.apply(Action::MoveLeft);
        }
        for _ in 0..(round % 7) {
            engine.apply(Action::MoveRight);
        }
        engine.apply(Action::HardDrop);
    }
    shot(&engine, "stacked");

    engine.apply(Action::TogglePause);
    shot(&engine, "paused");
    engine.apply(Action::TogglePause);

    for _ in 0..400 {
        engine.apply(Action::HardDrop);
        if engine.phase() == Phase::Over {
            break;
        }
    }
    shot(&engine, "gameover");
}
