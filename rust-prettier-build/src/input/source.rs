//! Where actions come from.

use crate::game::Action;
use std::io;
use std::time::Duration;

/// A source of player actions.
///
/// The game loop waits on this trait, never on a keyboard directly, so a test
/// can drive a complete game by handing it a scripted list of actions.
pub trait InputSource {
    /// Waits up to `timeout` for an action.
    ///
    /// Returns [`None`] when the timeout expires with nothing pressed, which is
    /// the normal case and is what lets the loop keep ticking gravity.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying source fails.
    fn poll(&mut self, timeout: Duration) -> io::Result<Option<Action>>;
}

/// An input source that replays a fixed list of actions and then goes quiet.
#[derive(Debug, Clone, Default)]
pub struct ScriptedInput {
    queued: Vec<Action>,
}

impl ScriptedInput {
    /// Creates a source that will yield `actions`, first one first.
    #[must_use]
    pub fn new(actions: impl IntoIterator<Item = Action>) -> Self {
        let mut queued: Vec<Action> = actions.into_iter().collect();
        queued.reverse();
        Self { queued }
    }

    /// Whether every scripted action has been consumed.
    #[must_use]
    pub fn is_exhausted(&self) -> bool {
        self.queued.is_empty()
    }
}

impl InputSource for ScriptedInput {
    fn poll(&mut self, _timeout: Duration) -> io::Result<Option<Action>> {
        Ok(self.queued.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain(source: &mut ScriptedInput) -> Vec<Action> {
        let mut seen = Vec::new();
        while let Ok(Some(action)) = source.poll(Duration::ZERO) {
            seen.push(action);
        }
        seen
    }

    #[test]
    fn scripted_actions_arrive_in_the_order_they_were_written() {
        let script = [Action::MoveLeft, Action::HardDrop, Action::Quit];
        let mut source = ScriptedInput::new(script);
        assert_eq!(drain(&mut source), script.to_vec());
    }

    #[test]
    fn an_exhausted_script_reports_nothing_pressed_rather_than_blocking() {
        let mut source = ScriptedInput::new([Action::Quit]);
        source.poll(Duration::ZERO).unwrap();
        assert_eq!(source.poll(Duration::ZERO).unwrap(), None);
        assert!(source.is_exhausted());
    }

    #[test]
    fn an_empty_script_is_immediately_exhausted() {
        let mut source = ScriptedInput::new([]);
        assert!(source.is_exhausted());
        assert_eq!(source.poll(Duration::ZERO).unwrap(), None);
    }

    #[test]
    fn the_timeout_does_not_change_what_a_script_yields() {
        let mut source = ScriptedInput::new([Action::MoveLeft]);
        assert_eq!(
            source.poll(Duration::from_secs(10)).unwrap(),
            Some(Action::MoveLeft)
        );
    }
}
