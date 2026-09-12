//! One bundle of every rule the engine consults.

use crate::rules::gravity::{ClassicGravity, GravityCurve};
use crate::rules::kick::{KickTable, SimpleKicks};
use crate::rules::level::{LevelCurve, LinesPerLevel};
use crate::rules::randomizer::{Randomizer, SevenBag};
use crate::rules::rng::{Rng, XorShift64};
use crate::rules::scoring::{GuidelineScoring, ScoringRule};

/// Every rule the engine needs, gathered behind one type.
///
/// The engine is generic over this single trait rather than over five separate
/// ones. That keeps its signature readable while leaving each rule individually
/// replaceable through the associated types.
pub trait RuleSet {
    /// Chooses the next piece.
    type Randomizer: Randomizer;
    /// Converts clears and drops into points.
    type Scoring: ScoringRule;
    /// Sets the falling speed.
    type Gravity: GravityCurve;
    /// Decides when the level advances.
    type Level: LevelCurve;
    /// Supplies rotation kick candidates.
    type Kicks: KickTable;

    /// Mutable access to the piece source, which is consumed as the game runs.
    fn randomizer(&mut self) -> &mut Self::Randomizer;
    /// The piece source, for previewing without consuming.
    fn peek_randomizer(&self) -> &Self::Randomizer;
    /// The scoring rule.
    fn scoring(&self) -> &Self::Scoring;
    /// The gravity curve.
    fn gravity(&self) -> &Self::Gravity;
    /// The level curve.
    fn level(&self) -> &Self::Level;
    /// The kick table.
    fn kicks(&self) -> &Self::Kicks;
}

/// The default rule set: seven-bag pieces, guideline scoring, classic gravity.
#[derive(Debug, Clone)]
pub struct StandardRules<R: Rng> {
    randomizer: SevenBag<R>,
    scoring: GuidelineScoring,
    gravity: ClassicGravity,
    level: LinesPerLevel,
    kicks: SimpleKicks,
}

impl<R: Rng> StandardRules<R> {
    /// Builds the standard rules drawing pieces from `rng`.
    pub fn new(rng: R) -> Self {
        Self {
            randomizer: SevenBag::new(rng),
            scoring: GuidelineScoring,
            gravity: ClassicGravity::default(),
            level: LinesPerLevel::default(),
            kicks: SimpleKicks,
        }
    }
}

impl StandardRules<XorShift64> {
    /// Builds the standard rules with a reproducible piece sequence.
    #[must_use]
    pub fn seeded(seed: u64) -> Self {
        Self::new(XorShift64::new(seed))
    }

    /// Builds the standard rules seeded from the system clock.
    #[must_use]
    pub fn from_clock() -> Self {
        Self::new(XorShift64::from_clock())
    }
}

impl<R: Rng> RuleSet for StandardRules<R> {
    type Randomizer = SevenBag<R>;
    type Scoring = GuidelineScoring;
    type Gravity = ClassicGravity;
    type Level = LinesPerLevel;
    type Kicks = SimpleKicks;

    fn randomizer(&mut self) -> &mut Self::Randomizer {
        &mut self.randomizer
    }

    fn peek_randomizer(&self) -> &Self::Randomizer {
        &self.randomizer
    }

    fn scoring(&self) -> &Self::Scoring {
        &self.scoring
    }

    fn gravity(&self) -> &Self::Gravity {
        &self.gravity
    }

    fn level(&self) -> &Self::Level {
        &self.level
    }

    fn kicks(&self) -> &Self::Kicks {
        &self.kicks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::scoring::LineClear;
    use std::time::Duration;

    #[test]
    fn the_standard_rules_expose_every_rule_the_engine_asks_for() {
        let mut rules = StandardRules::seeded(1);
        assert!(rules.gravity().interval(0) > Duration::ZERO);
        assert_eq!(rules.level().level_for(0), 0);
        assert!(rules.scoring().line_points(LineClear::Single, 0) > 0);
        assert!(!rules.kicks().candidates().is_empty());
        let _ = rules.randomizer().next_piece();
    }

    #[test]
    fn seeding_makes_a_whole_rule_set_reproducible() {
        let mut a = StandardRules::seeded(77);
        let mut b = StandardRules::seeded(77);
        let left: Vec<_> = (0..20).map(|_| a.randomizer().next_piece()).collect();
        let right: Vec<_> = (0..20).map(|_| b.randomizer().next_piece()).collect();
        assert_eq!(left, right);
    }

    #[test]
    fn peeking_does_not_consume_the_piece_the_engine_will_spawn_next() {
        let mut rules = StandardRules::seeded(3);
        let previewed = rules.peek_randomizer().peek(1);
        assert_eq!(
            previewed.first().copied(),
            Some(rules.randomizer().next_piece())
        );
    }
}
