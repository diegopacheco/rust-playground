//! How a clear turns into points.

/// The number of rows removed by a single lock, named as players name them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LineClear {
    /// One row.
    Single,
    /// Two rows at once.
    Double,
    /// Three rows at once.
    Triple,
    /// Four rows at once, only possible with the line piece.
    Tetris,
}

impl LineClear {
    /// Interprets a row count, returning [`None`] when nothing was cleared.
    #[must_use]
    pub const fn from_rows(rows: usize) -> Option<Self> {
        match rows {
            1 => Some(Self::Single),
            2 => Some(Self::Double),
            3 => Some(Self::Triple),
            4 => Some(Self::Tetris),
            _ => None,
        }
    }

    /// How many rows this clear removed.
    #[must_use]
    pub const fn rows(self) -> u32 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
            Self::Triple => 3,
            Self::Tetris => 4,
        }
    }
}

/// Turns events into points.
///
/// Scoring is the rule players argue about most, so it lives behind a trait:
/// a different rule set is a new implementor, not an edit to the engine.
pub trait ScoringRule {
    /// Points awarded for `clear` at `level`.
    fn line_points(&self, clear: LineClear, level: u32) -> u32;

    /// Points awarded for holding the piece down over `cells` rows.
    fn soft_drop_points(&self, cells: u32) -> u32 {
        cells
    }

    /// Points awarded for slamming a piece down `cells` rows.
    fn hard_drop_points(&self, cells: u32) -> u32 {
        cells * 2
    }
}

/// The standard guideline scoring table.
///
/// Rewards are deliberately super-linear in rows cleared — a Tetris is worth
/// twice four singles — because stacking high enough to clear four at once is
/// the risk the game is built around.
#[derive(Debug, Clone, Copy, Default)]
pub struct GuidelineScoring;

impl ScoringRule for GuidelineScoring {
    fn line_points(&self, clear: LineClear, level: u32) -> u32 {
        let base = match clear {
            LineClear::Single => 100,
            LineClear::Double => 300,
            LineClear::Triple => 500,
            LineClear::Tetris => 800,
        };
        base * (level + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [LineClear; 4] = [
        LineClear::Single,
        LineClear::Double,
        LineClear::Triple,
        LineClear::Tetris,
    ];

    #[test]
    fn clearing_nothing_is_not_a_line_clear() {
        assert_eq!(LineClear::from_rows(0), None);
    }

    #[test]
    fn one_to_four_rows_map_to_the_four_named_clears() {
        for (rows, expected) in (1..=4).zip(ALL) {
            assert_eq!(LineClear::from_rows(rows), Some(expected));
            assert_eq!(expected.rows(), rows as u32);
        }
    }

    #[test]
    fn more_than_four_rows_is_impossible_and_reports_as_such() {
        assert_eq!(LineClear::from_rows(5), None);
    }

    #[test]
    fn clearing_more_rows_at_once_always_pays_better() {
        let rule = GuidelineScoring;
        for pair in ALL.windows(2) {
            assert!(
                rule.line_points(pair[0], 0) < rule.line_points(pair[1], 0),
                "{:?} should pay less than {:?}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn a_tetris_beats_four_singles_so_stacking_high_is_worth_the_risk() {
        let rule = GuidelineScoring;
        let four_singles = 4 * rule.line_points(LineClear::Single, 0);
        assert!(rule.line_points(LineClear::Tetris, 0) > four_singles);
    }

    #[test]
    fn the_same_clear_pays_more_at_a_higher_level() {
        let rule = GuidelineScoring;
        assert!(rule.line_points(LineClear::Single, 5) > rule.line_points(LineClear::Single, 0));
    }

    #[test]
    fn level_zero_pays_the_plain_base_value() {
        assert_eq!(GuidelineScoring.line_points(LineClear::Tetris, 0), 800);
    }

    #[test]
    fn a_hard_drop_pays_better_than_riding_the_piece_down_slowly() {
        let rule = GuidelineScoring;
        assert!(rule.hard_drop_points(10) > rule.soft_drop_points(10));
    }

    #[test]
    fn dropping_zero_rows_earns_nothing() {
        let rule = GuidelineScoring;
        assert_eq!(rule.soft_drop_points(0), 0);
        assert_eq!(rule.hard_drop_points(0), 0);
    }
}
