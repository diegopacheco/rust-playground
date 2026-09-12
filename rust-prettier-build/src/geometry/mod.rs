//! Coordinate primitives shared by every other layer.
//!
//! Nothing here knows about tetrominoes, boards or terminals; it is the common
//! vocabulary those layers speak.

pub mod point;
pub mod rotation;

pub use point::{Offset, Point};
pub use rotation::{Rotation, Spin};
