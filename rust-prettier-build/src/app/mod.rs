//! Wiring the pieces together into a runnable program.

pub mod config;
pub mod runner;

pub use config::{Config, USAGE};
pub use runner::{Flow, GameLoop};

use crate::game::{Engine, SystemClock};
use crate::input::TerminalInput;
use crate::render::screen::Screen;
use crate::render::terminal::{TerminalCanvas, TerminalGuard};
use crate::rules::StandardRules;
use std::io::{self, Write};

/// Runs a game in the terminal.
///
/// The terminal guard is created first and dropped last, so raw mode is undone
/// even if the loop fails part way through.
///
/// # Errors
///
/// Returns any error from the terminal, the input source or the renderer.
pub fn run(config: Config) -> io::Result<()> {
    let board = config.board();
    let rules = match config.seed {
        Some(seed) => StandardRules::seeded(seed),
        None => StandardRules::from_clock(),
    };

    let columns = u16::try_from(config.columns).unwrap_or(0);
    let rows = u16::try_from(config.rows).unwrap_or(0);
    let (needed_width, needed_height) = Screen::required_size(columns, rows);

    let guard = TerminalGuard::enter()?;
    let (width, height) = TerminalGuard::size()?;
    if width < needed_width || height < needed_height {
        drop(guard);
        return Err(io::Error::other(format!(
            "terminal is {width}x{height}; this board needs at least {needed_width}x{needed_height}"
        )));
    }

    let canvas = TerminalCanvas::new(io::stdout(), width, height);
    let mut game = GameLoop::new(
        Engine::new(board, rules),
        TerminalInput::default(),
        canvas,
        SystemClock::new(),
    );
    game.run()
}

/// Parses arguments and runs, printing usage on a bad argument.
///
/// Returns the process exit code.
pub fn main_with<I: IntoIterator<Item = String>>(args: I) -> i32 {
    let args: Vec<String> = args.into_iter().collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{USAGE}");
        return 0;
    }
    let config = match Config::from_args(args) {
        Ok(config) => config,
        Err(message) => {
            let _ = writeln!(io::stderr(), "error: {message}\n\n{USAGE}");
            return 2;
        }
    };
    match run(config) {
        Ok(()) => 0,
        Err(error) => {
            let _ = writeln!(io::stderr(), "error: {error}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_exits_cleanly_without_starting_a_game() {
        assert_eq!(main_with(["--help".to_owned()]), 0);
    }

    #[test]
    fn a_bad_argument_exits_with_a_usage_error_rather_than_a_panic() {
        assert_eq!(main_with(["--nope".to_owned()]), 2);
    }

    #[test]
    fn the_usage_text_is_what_help_would_print() {
        assert!(USAGE.contains("tetris-tui"));
    }
}
