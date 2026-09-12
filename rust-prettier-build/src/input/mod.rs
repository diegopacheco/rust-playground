//! Player intent: from a key press to an [`crate::game::Action`].
//!
//! The chain is deliberately three small steps — terminal event, [`key::Key`],
//! action — so that only the last step touches crossterm and the other two are
//! plain values the tests can build by hand.

pub mod key;
pub mod keymap;
pub mod source;
pub mod terminal;

pub use key::Key;
pub use keymap::{DefaultKeyMap, KeyMap};
pub use source::{InputSource, ScriptedInput};
pub use terminal::TerminalInput;
