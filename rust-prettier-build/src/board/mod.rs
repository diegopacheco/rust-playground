//! The playfield: what it holds, what fits in it and how rows collapse.

pub mod cell;
pub mod grid;

pub use cell::Cell;
pub use grid::{Board, Playfield};
