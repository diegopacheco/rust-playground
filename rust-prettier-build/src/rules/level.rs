//! How clearing rows advances the level.

/// Maps total rows cleared to a level.
///
/// Progression and speed are separate concerns: this decides *when* the game
/// gets harder, [`crate::rules::gravity::GravityCurve`] decides *how much*.
pub trait LevelCurve {
    /// The level reached after clearing `lines` rows in total.
    fn level_for(&self, lines: u32) -> u32;
}

/// Advances one level for every fixed number of rows cleared.
#[derive(Debug, Clone, Copy)]
pub struct LinesPerLevel {
    per_level: u32,
    cap: u32,
}

impl LinesPerLevel {
    /// Creates a curve advancing every `per_level` rows, stopping at `cap`.
    ///
    /// # Panics
    ///
    /// Panics if `per_level` is zero, which would make every clear infinite.
    #[must_use]
    pub const fn new(per_level: u32, cap: u32) -> Self {
        assert!(per_level > 0, "per_level must be positive");
        Self { per_level, cap }
    }
}

impl Default for LinesPerLevel {
    fn default() -> Self {
        Self::new(10, 29)
    }
}

impl LevelCurve for LinesPerLevel {
    fn level_for(&self, lines: u32) -> u32 {
        (lines / self.per_level).min(self.cap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_game_starts_at_level_zero() {
        assert_eq!(LinesPerLevel::default().level_for(0), 0);
    }

    #[test]
    fn the_level_never_drops_as_more_rows_are_cleared() {
        let curve = LinesPerLevel::default();
        for lines in 0..500 {
            assert!(curve.level_for(lines + 1) >= curve.level_for(lines));
        }
    }

    #[test]
    fn ten_rows_earn_exactly_one_level_with_the_default_curve() {
        let curve = LinesPerLevel::default();
        assert_eq!(curve.level_for(9), 0);
        assert_eq!(curve.level_for(10), 1);
        assert_eq!(curve.level_for(20), 2);
    }

    #[test]
    fn the_level_stops_climbing_at_the_cap_so_speed_cannot_run_away() {
        let curve = LinesPerLevel::new(10, 29);
        assert_eq!(curve.level_for(10_000), 29);
        assert_eq!(curve.level_for(u32::MAX), 29);
    }

    #[test]
    fn a_steeper_curve_reaches_a_given_level_sooner() {
        let steep = LinesPerLevel::new(2, 29);
        let gentle = LinesPerLevel::new(20, 29);
        assert!(steep.level_for(20) > gentle.level_for(20));
    }
}
