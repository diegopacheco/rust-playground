//! Keys, independent of any terminal library.

/// A key the player pressed.
///
/// A deliberately small set of our own, rather than the terminal library's
/// event type, so the key map is a pure function that tests can call directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// The left arrow.
    Left,
    /// The right arrow.
    Right,
    /// The up arrow.
    Up,
    /// The down arrow.
    Down,
    /// The space bar.
    Space,
    /// The escape key.
    Escape,
    /// A printable character.
    Char(char),
}

impl Key {
    /// The lowercase character this key carries, if it carries one.
    #[must_use]
    pub fn letter(self) -> Option<char> {
        match self {
            Self::Char(ch) => Some(ch.to_ascii_lowercase()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_letter_key_reports_its_character_in_lowercase() {
        assert_eq!(Key::Char('Q').letter(), Some('q'));
        assert_eq!(Key::Char('q').letter(), Some('q'));
    }

    #[test]
    fn a_named_key_carries_no_letter() {
        assert_eq!(Key::Left.letter(), None);
        assert_eq!(Key::Space.letter(), None);
    }

    #[test]
    fn the_arrows_are_four_distinct_keys() {
        use std::collections::HashSet;
        let arrows: HashSet<Key> = [Key::Left, Key::Right, Key::Up, Key::Down].into();
        assert_eq!(arrows.len(), 4);
    }
}
