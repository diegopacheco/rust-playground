//! The interchangeable rules of the game: chance, reward and pace.
//!
//! Each rule is a trait with one standard implementor. The engine holds them by
//! generic parameter, so swapping a rule set never touches engine code.

pub mod gravity;
pub mod kick;
pub mod level;
pub mod randomizer;
pub mod rng;
pub mod ruleset;
pub mod scoring;

pub use gravity::{ClassicGravity, GravityCurve};
pub use kick::{KickTable, SimpleKicks};
pub use level::{LevelCurve, LinesPerLevel};
pub use randomizer::{Randomizer, SevenBag};
pub use rng::{Rng, XorShift64};
pub use ruleset::{RuleSet, StandardRules};
pub use scoring::{GuidelineScoring, LineClear, ScoringRule};
