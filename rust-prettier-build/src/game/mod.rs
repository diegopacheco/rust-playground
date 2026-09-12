//! The game itself: intent in, state out.

pub mod clock;
pub mod command;
pub mod engine;
pub mod snapshot;
pub mod state;

pub use clock::{Clock, ManualClock, SystemClock};
pub use command::Action;
pub use engine::Engine;
pub use snapshot::Snapshot;
pub use state::{Phase, Stats};
