//! A modular, trait-driven Tetris for the terminal.
//!
//! The crate is layered, and each layer depends only on the traits of the one
//! below it:
//!
//! - [`geometry`] — points, offsets and rotations.
//! - [`piece`] — the [`piece::Shape`] trait and the seven tetrominoes.
//! - [`board`] — the [`board::Playfield`] trait and the locked stack.
//! - [`rules`] — chance, reward and pace, bundled behind [`rules::RuleSet`].
//! - [`game`] — the engine, driven purely by actions and elapsed time.
//! - [`render`] — views painting onto the [`render::Canvas`] trait.
//! - [`input`] — key presses mapped to actions.
//! - [`app`] — the loop that connects them.
//!
//! Nothing below [`app`] reads a clock, a keyboard or a terminal, which is why
//! a complete game can be played inside a test.
//!
//! ```
//! use std::time::Duration;
//! use tetris::board::Board;
//! use tetris::game::{Action, Engine};
//! use tetris::rules::StandardRules;
//!
//! let mut engine = Engine::new(Board::standard(), StandardRules::seeded(42));
//! engine.apply(Action::MoveLeft);
//! engine.apply(Action::HardDrop);
//! engine.tick(Duration::from_millis(800));
//!
//! let snapshot = engine.snapshot();
//! assert_eq!(snapshot.stats.pieces, 1);
//! assert!(snapshot.stats.score > 0);
//! ```

pub mod app;
pub mod board;
pub mod game;
pub mod geometry;
pub mod input;
pub mod piece;
pub mod render;
pub mod rules;
pub mod test_support;
