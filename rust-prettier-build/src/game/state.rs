//! The observable condition of a game in progress.

/// What the game is currently doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Phase {
    /// Pieces are falling and input is accepted.
    #[default]
    Playing,
    /// Play is suspended; gravity is frozen.
    Paused,
    /// A piece could not spawn; only restart and quit remain.
    Over,
}

impl Phase {
    /// Whether gravity should advance in this phase.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Playing)
    }

    /// Whether the game has finished.
    #[must_use]
    pub const fn is_over(self) -> bool {
        matches!(self, Self::Over)
    }

    /// Toggles between playing and paused, leaving a finished game finished.
    #[must_use]
    pub const fn toggled_pause(self) -> Self {
        match self {
            Self::Playing => Self::Paused,
            Self::Paused => Self::Playing,
            Self::Over => Self::Over,
        }
    }
}

/// Running totals shown to the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stats {
    /// Points earned so far.
    pub score: u32,
    /// Rows cleared so far.
    pub lines: u32,
    /// The current level.
    pub level: u32,
    /// How many pieces have locked.
    pub pieces: u32,
}

impl Stats {
    /// Adds `points` to the score, saturating rather than wrapping.
    pub const fn add_score(&mut self, points: u32) {
        self.score = self.score.saturating_add(points);
    }

    /// Records `rows` cleared.
    pub const fn add_lines(&mut self, rows: u32) {
        self.lines = self.lines.saturating_add(rows);
    }

    /// Records one more locked piece.
    pub const fn add_piece(&mut self) {
        self.pieces = self.pieces.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_game_is_playing_with_everything_at_zero() {
        assert_eq!(Phase::default(), Phase::Playing);
        assert_eq!(
            Stats::default(),
            Stats {
                score: 0,
                lines: 0,
                level: 0,
                pieces: 0
            }
        );
    }

    #[test]
    fn only_the_playing_phase_lets_gravity_advance() {
        assert!(Phase::Playing.is_running());
        assert!(!Phase::Paused.is_running());
        assert!(!Phase::Over.is_running());
    }

    #[test]
    fn pausing_and_unpausing_returns_to_play() {
        assert_eq!(
            Phase::Playing.toggled_pause().toggled_pause(),
            Phase::Playing
        );
    }

    #[test]
    fn a_finished_game_cannot_be_unpaused_back_into_play() {
        assert_eq!(Phase::Over.toggled_pause(), Phase::Over);
    }

    #[test]
    fn the_score_saturates_rather_than_wrapping_to_zero_on_a_long_game() {
        let mut stats = Stats {
            score: u32::MAX,
            ..Stats::default()
        };
        stats.add_score(500);
        assert_eq!(stats.score, u32::MAX);
    }

    #[test]
    fn totals_accumulate_across_several_clears() {
        let mut stats = Stats::default();
        stats.add_lines(4);
        stats.add_lines(2);
        stats.add_piece();
        assert_eq!((stats.lines, stats.pieces), (6, 1));
    }
}
