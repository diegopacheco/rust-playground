//! What to try when a rotation does not fit where it stands.

use crate::geometry::Offset;

/// Supplies the translations to attempt after a rotation is blocked.
///
/// Without kicks a piece cannot turn while resting against a wall or the stack,
/// which players read as the controls being broken. The candidates are tried in
/// order and the first that fits wins, so ordering is the rule: nearer offsets
/// must come first or pieces teleport.
pub trait KickTable {
    /// Translations to try, in priority order. The first is conventionally
    /// [`Offset::ZERO`], meaning "rotate in place".
    fn candidates(&self) -> &[Offset];
}

/// A compact symmetric kick table: in place, then sideways, then up.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleKicks;

impl SimpleKicks {
    const CANDIDATES: [Offset; 7] = [
        Offset::new(0, 0),
        Offset::new(-1, 0),
        Offset::new(1, 0),
        Offset::new(0, -1),
        Offset::new(-2, 0),
        Offset::new(2, 0),
        Offset::new(-1, -1),
    ];
}

impl KickTable for SimpleKicks {
    fn candidates(&self) -> &[Offset] {
        &Self::CANDIDATES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotating_in_place_is_always_tried_first() {
        assert_eq!(SimpleKicks.candidates().first(), Some(&Offset::ZERO));
    }

    #[test]
    fn the_table_offers_a_way_off_both_walls() {
        let candidates = SimpleKicks.candidates();
        assert!(candidates.contains(&Offset::new(-1, 0)));
        assert!(candidates.contains(&Offset::new(1, 0)));
    }

    #[test]
    fn candidates_are_ordered_by_distance_so_pieces_never_teleport() {
        let distances: Vec<i16> = SimpleKicks
            .candidates()
            .iter()
            .map(|o| o.dx.abs() + o.dy.abs())
            .collect();
        let mut sorted = distances.clone();
        sorted.sort_unstable();
        assert_eq!(distances, sorted);
    }

    #[test]
    fn no_candidate_pushes_a_piece_downward_which_would_steal_a_free_drop() {
        assert!(SimpleKicks.candidates().iter().all(|o| o.dy <= 0));
    }

    #[test]
    fn every_candidate_is_distinct_so_no_position_is_probed_twice() {
        let candidates = SimpleKicks.candidates();
        for (index, offset) in candidates.iter().enumerate() {
            assert!(
                !candidates[index + 1..].contains(offset),
                "{offset:?} repeated"
            );
        }
    }
}
