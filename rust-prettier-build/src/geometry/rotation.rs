//! The four orientations a tetromino can occupy.

/// A tetromino orientation, named after the number of clockwise quarter turns
/// applied to its spawn state.
///
/// Rotation is a cyclic group of order four, so [`Rotation::clockwise`] and
/// [`Rotation::counter_clockwise`] are total: every orientation has a successor
/// and a predecessor, and four turns in either direction return to the start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub enum Rotation {
    /// The orientation a piece spawns in.
    #[default]
    Spawn,
    /// One quarter turn clockwise from spawn.
    Right,
    /// Two quarter turns from spawn, upside down.
    Half,
    /// One quarter turn counter-clockwise from spawn.
    Left,
}

impl Rotation {
    /// Every orientation, in clockwise order starting at [`Rotation::Spawn`].
    pub const ALL: [Self; 4] = [Self::Spawn, Self::Right, Self::Half, Self::Left];

    /// Returns the orientation one quarter turn clockwise.
    #[must_use]
    pub const fn clockwise(self) -> Self {
        match self {
            Self::Spawn => Self::Right,
            Self::Right => Self::Half,
            Self::Half => Self::Left,
            Self::Left => Self::Spawn,
        }
    }

    /// Returns the orientation one quarter turn counter-clockwise.
    #[must_use]
    pub const fn counter_clockwise(self) -> Self {
        match self {
            Self::Spawn => Self::Left,
            Self::Right => Self::Spawn,
            Self::Half => Self::Right,
            Self::Left => Self::Half,
        }
    }

    /// Returns how many clockwise quarter turns separate this orientation from
    /// [`Rotation::Spawn`].
    #[must_use]
    pub const fn quarter_turns(self) -> u8 {
        match self {
            Self::Spawn => 0,
            Self::Right => 1,
            Self::Half => 2,
            Self::Left => 3,
        }
    }
}

/// The direction of a quarter turn.
///
/// Input maps a key to a `Spin`; the piece layer applies it. Keeping the
/// direction a value rather than two near-identical methods means a rotation
/// can be stored, replayed and tested like any other command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spin {
    /// A quarter turn clockwise.
    Clockwise,
    /// A quarter turn counter-clockwise.
    CounterClockwise,
}

impl Spin {
    /// Applies this spin to `rotation`.
    #[must_use]
    pub const fn apply(self, rotation: Rotation) -> Rotation {
        match self {
            Self::Clockwise => rotation.clockwise(),
            Self::CounterClockwise => rotation.counter_clockwise(),
        }
    }

    /// The spin that undoes this one.
    #[must_use]
    pub const fn reversed(self) -> Self {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_clockwise_turns_return_to_the_start() {
        for start in Rotation::ALL {
            let looped = start.clockwise().clockwise().clockwise().clockwise();
            assert_eq!(looped, start, "{start:?} did not survive a full turn");
        }
    }

    #[test]
    fn counter_clockwise_undoes_clockwise_so_a_rejected_spin_restores_orientation() {
        for start in Rotation::ALL {
            assert_eq!(start.clockwise().counter_clockwise(), start);
        }
    }

    #[test]
    fn the_two_directions_disagree_so_the_player_has_a_real_choice() {
        for start in Rotation::ALL {
            assert_ne!(start.clockwise(), start.counter_clockwise());
        }
    }

    #[test]
    fn half_is_reached_by_two_turns_from_either_direction() {
        assert_eq!(Rotation::Spawn.clockwise().clockwise(), Rotation::Half);
        assert_eq!(
            Rotation::Spawn.counter_clockwise().counter_clockwise(),
            Rotation::Half
        );
    }

    #[test]
    fn quarter_turns_counts_clockwise_distance_from_spawn() {
        let mut current = Rotation::Spawn;
        for expected in 0..4 {
            assert_eq!(current.quarter_turns(), expected);
            current = current.clockwise();
        }
    }

    #[test]
    fn spawn_is_the_default_so_new_pieces_start_upright() {
        assert_eq!(Rotation::default(), Rotation::Spawn);
    }

    #[test]
    fn a_spin_and_its_reverse_cancel_so_a_rejected_rotation_is_recoverable() {
        for start in Rotation::ALL {
            for spin in [Spin::Clockwise, Spin::CounterClockwise] {
                assert_eq!(spin.reversed().apply(spin.apply(start)), start);
            }
        }
    }

    #[test]
    fn spin_apply_agrees_with_the_direct_rotation_methods() {
        for start in Rotation::ALL {
            assert_eq!(Spin::Clockwise.apply(start), start.clockwise());
            assert_eq!(
                Spin::CounterClockwise.apply(start),
                start.counter_clockwise()
            );
        }
    }
}
