//! Reading keys from a real terminal.

use crate::game::Action;
use crate::input::key::Key;
use crate::input::keymap::{DefaultKeyMap, KeyMap};
use crate::input::source::InputSource;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;
use std::time::Duration;

/// Reads key presses from the terminal and maps them to actions.
#[derive(Debug, Clone)]
pub struct TerminalInput<K: KeyMap> {
    keymap: K,
}

impl<K: KeyMap> TerminalInput<K> {
    /// Creates an input source using `keymap`.
    pub const fn new(keymap: K) -> Self {
        Self { keymap }
    }
}

impl Default for TerminalInput<DefaultKeyMap> {
    fn default() -> Self {
        Self::new(DefaultKeyMap)
    }
}

impl<K: KeyMap> InputSource for TerminalInput<K> {
    fn poll(&mut self, timeout: Duration) -> io::Result<Option<Action>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        let Event::Key(key) = event::read()? else {
            return Ok(None);
        };
        if key.kind == KeyEventKind::Release {
            return Ok(None);
        }
        if is_interrupt(key) {
            return Ok(Some(Action::Quit));
        }
        Ok(translate(key.code).and_then(|key| self.keymap.action_for(key)))
    }
}

fn is_interrupt(key: KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c' | 'C'))
}

const fn translate(code: KeyCode) -> Option<Key> {
    match code {
        KeyCode::Left => Some(Key::Left),
        KeyCode::Right => Some(Key::Right),
        KeyCode::Up => Some(Key::Up),
        KeyCode::Down => Some(Key::Down),
        KeyCode::Char(' ') => Some(Key::Space),
        KeyCode::Esc => Some(Key::Escape),
        KeyCode::Char(ch) => Some(Key::Char(ch)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Spin;

    #[test]
    fn the_arrow_codes_become_our_own_arrow_keys() {
        assert_eq!(translate(KeyCode::Left), Some(Key::Left));
        assert_eq!(translate(KeyCode::Right), Some(Key::Right));
        assert_eq!(translate(KeyCode::Up), Some(Key::Up));
        assert_eq!(translate(KeyCode::Down), Some(Key::Down));
    }

    #[test]
    fn a_space_is_recognised_as_the_space_bar_not_as_a_character() {
        assert_eq!(translate(KeyCode::Char(' ')), Some(Key::Space));
    }

    #[test]
    fn a_letter_survives_translation_so_the_key_map_can_see_it() {
        assert_eq!(translate(KeyCode::Char('p')), Some(Key::Char('p')));
    }

    #[test]
    fn keys_the_game_does_not_use_are_dropped_early() {
        assert_eq!(translate(KeyCode::F(5)), None);
        assert_eq!(translate(KeyCode::Insert), None);
    }

    #[test]
    fn control_c_quits_even_though_it_is_not_in_the_key_map() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(is_interrupt(key));
        assert_eq!(DefaultKeyMap.action_for(Key::Char('c')), None);
    }

    #[test]
    fn a_plain_c_is_not_an_interrupt() {
        assert!(!is_interrupt(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::NONE
        )));
    }

    #[test]
    fn translation_feeds_the_key_map_which_produces_the_action() {
        let keymap = DefaultKeyMap;
        let key = translate(KeyCode::Char('z')).unwrap();
        assert_eq!(
            keymap.action_for(key),
            Some(Action::Rotate(Spin::CounterClockwise))
        );
    }
}
