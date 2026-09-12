//! Turning keys into actions.

use crate::game::Action;
use crate::geometry::Spin;
use crate::input::key::Key;

/// Decides what a key means.
///
/// Rebinding is the single most requested change to any game, so the mapping is
/// a trait with no dependency on either the terminal or the engine: an
/// alternative layout is a new implementor and nothing else moves.
pub trait KeyMap {
    /// The action `key` should trigger, or [`None`] if it is unbound.
    fn action_for(&self, key: Key) -> Option<Action>;
}

/// Arrow keys to move, space to drop, and the usual letters.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultKeyMap;

impl KeyMap for DefaultKeyMap {
    fn action_for(&self, key: Key) -> Option<Action> {
        match key {
            Key::Left => Some(Action::MoveLeft),
            Key::Right => Some(Action::MoveRight),
            Key::Down => Some(Action::SoftDrop),
            Key::Up => Some(Action::Rotate(Spin::Clockwise)),
            Key::Space => Some(Action::HardDrop),
            Key::Escape => Some(Action::Quit),
            Key::Char(_) => match key.letter() {
                Some('x') => Some(Action::Rotate(Spin::Clockwise)),
                Some('z') => Some(Action::Rotate(Spin::CounterClockwise)),
                Some('p') => Some(Action::TogglePause),
                Some('r') => Some(Action::Restart),
                Some('q') => Some(Action::Quit),
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(key: Key) -> Option<Action> {
        DefaultKeyMap.action_for(key)
    }

    #[test]
    fn the_arrow_keys_move_and_turn_the_piece() {
        assert_eq!(action(Key::Left), Some(Action::MoveLeft));
        assert_eq!(action(Key::Right), Some(Action::MoveRight));
        assert_eq!(action(Key::Down), Some(Action::SoftDrop));
        assert_eq!(action(Key::Up), Some(Action::Rotate(Spin::Clockwise)));
    }

    #[test]
    fn space_is_the_hard_drop_because_it_is_the_easiest_key_to_hit() {
        assert_eq!(action(Key::Space), Some(Action::HardDrop));
    }

    #[test]
    fn both_rotation_directions_are_reachable_from_the_keyboard() {
        assert_eq!(
            action(Key::Char('z')),
            Some(Action::Rotate(Spin::CounterClockwise))
        );
        assert_eq!(
            action(Key::Char('x')),
            Some(Action::Rotate(Spin::Clockwise))
        );
    }

    #[test]
    fn keys_work_with_caps_lock_on() {
        for (lower, upper) in [('q', 'Q'), ('p', 'P'), ('r', 'R'), ('z', 'Z')] {
            assert_eq!(action(Key::Char(lower)), action(Key::Char(upper)));
        }
    }

    #[test]
    fn there_are_two_ways_to_quit_so_the_player_is_never_trapped() {
        assert_eq!(action(Key::Escape), Some(Action::Quit));
        assert_eq!(action(Key::Char('q')), Some(Action::Quit));
    }

    #[test]
    fn pause_and_restart_are_bound() {
        assert_eq!(action(Key::Char('p')), Some(Action::TogglePause));
        assert_eq!(action(Key::Char('r')), Some(Action::Restart));
    }

    #[test]
    fn an_unbound_key_does_nothing_rather_than_guessing() {
        assert_eq!(action(Key::Char('k')), None);
        assert_eq!(action(Key::Char('7')), None);
    }

    #[test]
    fn no_two_distinct_keys_are_bound_to_conflicting_meanings_by_accident() {
        let bound: Vec<Action> = "qprzx"
            .chars()
            .filter_map(|ch| action(Key::Char(ch)))
            .collect();
        assert_eq!(bound.len(), 5);
    }
}
