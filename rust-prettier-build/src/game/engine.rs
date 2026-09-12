//! The rules of play, wired together and driven by actions and elapsed time.

use crate::board::{Board, Playfield};
use crate::game::command::Action;
use crate::game::snapshot::Snapshot;
use crate::game::state::{Phase, Stats};
use crate::geometry::{Offset, Point, Spin};
use crate::piece::{ActivePiece, Shape, Tetromino};
use crate::rules::kick::KickTable;
use crate::rules::randomizer::Randomizer;
use crate::rules::ruleset::RuleSet;
use crate::rules::scoring::{LineClear, ScoringRule};
use crate::rules::{GravityCurve, LevelCurve};
use std::time::Duration;

/// A game of Tetris.
///
/// The engine owns the board, the falling piece and the running totals, and it
/// consults [`RuleSet`] for every decision that could reasonably differ between
/// variants. It never reads a clock or a keyboard: time arrives through
/// [`Engine::tick`] and intent through [`Engine::apply`], which is what makes a
/// whole game reproducible in a test.
#[derive(Debug, Clone)]
pub struct Engine<R: RuleSet> {
    board: Board,
    rules: R,
    current: ActivePiece,
    phase: Phase,
    stats: Stats,
    elapsed: Duration,
}

impl<R: RuleSet> Engine<R> {
    /// How many upcoming pieces a snapshot carries.
    pub const PREVIEW: usize = 5;

    /// Starts a game on `board` under `rules`, spawning the first piece.
    pub fn new(board: Board, mut rules: R) -> Self {
        let kind = rules.randomizer().next_piece();
        let origin = Self::spawn_origin(&board, kind);
        Self {
            current: ActivePiece::new(kind, origin),
            board,
            rules,
            phase: Phase::Playing,
            stats: Stats::default(),
            elapsed: Duration::ZERO,
        }
    }

    fn spawn_origin(board: &Board, kind: Tetromino) -> Point {
        Point::new((board.width() - kind.bounding_box()) / 2, 0)
    }

    /// The locked stack.
    pub const fn board(&self) -> &Board {
        &self.board
    }

    /// The piece currently falling.
    pub const fn current(&self) -> &ActivePiece {
        &self.current
    }

    /// Where the falling piece would land if dropped now.
    pub fn ghost(&self) -> ActivePiece {
        self.board.landed(&self.current)
    }

    /// The upcoming pieces, soonest first.
    pub fn next_pieces(&self, count: usize) -> Vec<Tetromino> {
        self.rules.peek_randomizer().peek(count)
    }

    /// The running totals.
    pub const fn stats(&self) -> Stats {
        self.stats
    }

    /// What the game is currently doing.
    pub const fn phase(&self) -> Phase {
        self.phase
    }

    /// Takes an immutable picture of the game for rendering.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            board: self.board.clone(),
            current: self.current,
            ghost: self.ghost(),
            next: self.next_pieces(Self::PREVIEW),
            stats: self.stats,
            phase: self.phase,
        }
    }

    /// Applies a player action.
    ///
    /// Actions that need a falling piece are ignored unless the game is running,
    /// so a paused or finished game cannot be played by accident.
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => {}
            Action::Restart => self.restart(),
            Action::TogglePause => self.phase = self.phase.toggled_pause(),
            _ if !self.phase.is_running() => {}
            Action::MoveLeft => {
                self.try_shift(Offset::LEFT);
            }
            Action::MoveRight => {
                self.try_shift(Offset::RIGHT);
            }
            Action::Rotate(spin) => {
                self.try_rotate(spin);
            }
            Action::SoftDrop => self.soft_drop(),
            Action::HardDrop => self.hard_drop(),
        }
    }

    /// Advances the game by `delta` of elapsed time.
    ///
    /// Time is accumulated rather than truncated, so a slow frame still applies
    /// every gravity step it was owed instead of silently losing them.
    pub fn tick(&mut self, delta: Duration) {
        if !self.phase.is_running() {
            return;
        }
        self.elapsed = self.elapsed.saturating_add(delta);
        while self.phase.is_running() {
            let interval = self.rules.gravity().interval(self.stats.level);
            if self.elapsed < interval {
                break;
            }
            self.elapsed -= interval;
            if !self.try_shift(Offset::DOWN) {
                self.lock();
            }
        }
    }

    /// Abandons the current game and starts a fresh one on an empty board.
    pub fn restart(&mut self) {
        self.board = Board::new(self.board.width(), self.board.height());
        self.stats = Stats::default();
        self.phase = Phase::Playing;
        self.elapsed = Duration::ZERO;
        self.spawn();
    }

    fn try_shift(&mut self, offset: Offset) -> bool {
        let candidate = self.current.shifted(offset);
        let fits = self.board.accepts(&candidate);
        if fits {
            self.current = candidate;
        }
        fits
    }

    fn try_rotate(&mut self, spin: Spin) -> bool {
        let spun = self.current.spun(spin);
        for &kick in self.rules.kicks().candidates() {
            let candidate = spun.shifted(kick);
            if self.board.accepts(&candidate) {
                self.current = candidate;
                return true;
            }
        }
        false
    }

    fn soft_drop(&mut self) {
        if self.try_shift(Offset::DOWN) {
            let points = self.rules.scoring().soft_drop_points(1);
            self.stats.add_score(points);
        } else {
            self.lock();
        }
    }

    fn hard_drop(&mut self) {
        let landed = self.board.landed(&self.current);
        let distance = u32::from(
            (landed.origin.y - self.current.origin.y)
                .max(0)
                .unsigned_abs(),
        );
        self.current = landed;
        let points = self.rules.scoring().hard_drop_points(distance);
        self.stats.add_score(points);
        self.lock();
    }

    fn lock(&mut self) {
        self.board.lock(&self.current);
        self.stats.add_piece();
        let cleared = self.board.clear_full_rows();
        if let Some(clear) = LineClear::from_rows(cleared.len()) {
            let points = self.rules.scoring().line_points(clear, self.stats.level);
            self.stats.add_score(points);
        }
        self.stats
            .add_lines(u32::try_from(cleared.len()).unwrap_or(0));
        self.stats.level = self.rules.level().level_for(self.stats.lines);
        self.spawn();
    }

    fn spawn(&mut self) {
        let kind = self.rules.randomizer().next_piece();
        let piece = ActivePiece::new(kind, Self::spawn_origin(&self.board, kind));
        self.current = piece;
        if !self.board.accepts(&piece) {
            self.phase = Phase::Over;
        }
        self.elapsed = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Cell;
    use crate::rules::gravity::ClassicGravity;
    use crate::rules::kick::SimpleKicks;
    use crate::rules::level::LinesPerLevel;
    use crate::rules::scoring::GuidelineScoring;
    use std::collections::VecDeque;

    #[derive(Debug, Clone)]
    struct ScriptedBag {
        script: VecDeque<Tetromino>,
        fallback: Tetromino,
    }

    impl Randomizer for ScriptedBag {
        fn next_piece(&mut self) -> Tetromino {
            self.script.pop_front().unwrap_or(self.fallback)
        }

        fn peek(&self, count: usize) -> Vec<Tetromino> {
            self.script.iter().copied().take(count).collect()
        }
    }

    #[derive(Debug, Clone)]
    struct ScriptedRules {
        randomizer: ScriptedBag,
        scoring: GuidelineScoring,
        gravity: ClassicGravity,
        level: LinesPerLevel,
        kicks: SimpleKicks,
    }

    impl RuleSet for ScriptedRules {
        type Randomizer = ScriptedBag;
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

    fn engine_of(pieces: &[Tetromino], width: i16, height: i16) -> Engine<ScriptedRules> {
        let rules = ScriptedRules {
            randomizer: ScriptedBag {
                script: pieces.iter().copied().collect(),
                fallback: Tetromino::O,
            },
            scoring: GuidelineScoring,
            gravity: ClassicGravity::default(),
            level: LinesPerLevel::default(),
            kicks: SimpleKicks,
        };
        Engine::new(Board::new(width, height), rules)
    }

    fn standard(pieces: &[Tetromino]) -> Engine<ScriptedRules> {
        engine_of(pieces, Board::STANDARD_WIDTH, Board::STANDARD_HEIGHT)
    }

    fn gravity_step(engine: &Engine<ScriptedRules>) -> Duration {
        ClassicGravity::default().interval(engine.stats().level)
    }

    #[test]
    fn a_new_game_is_playing_with_a_piece_already_falling() {
        let engine = standard(&[Tetromino::T]);
        assert_eq!(engine.phase(), Phase::Playing);
        assert_eq!(engine.current().kind, Tetromino::T);
        assert_eq!(engine.stats(), Stats::default());
    }

    #[test]
    fn a_piece_spawns_horizontally_centred_so_both_sides_stay_reachable() {
        let engine = standard(&[Tetromino::O]);
        let columns: Vec<i16> = engine.current().cells().iter().map(|c| c.x).collect();
        let left = columns.iter().min().copied().unwrap();
        let right = columns.iter().max().copied().unwrap();
        assert_eq!(left, Board::STANDARD_WIDTH - 1 - right);
    }

    #[test]
    fn moving_left_and_right_shifts_the_piece_by_one_column() {
        let mut engine = standard(&[Tetromino::O]);
        let start = engine.current().origin;
        engine.apply(Action::MoveLeft);
        assert_eq!(engine.current().origin, start.shifted(Offset::LEFT));
        engine.apply(Action::MoveRight);
        assert_eq!(engine.current().origin, start);
    }

    #[test]
    fn the_wall_stops_a_piece_instead_of_letting_it_leave_the_board() {
        let mut engine = standard(&[Tetromino::O]);
        for _ in 0..20 {
            engine.apply(Action::MoveLeft);
        }
        assert!(engine.current().cells().iter().all(|c| c.x >= 0));
        let pinned = engine.current().origin;
        engine.apply(Action::MoveLeft);
        assert_eq!(engine.current().origin, pinned);
    }

    #[test]
    fn rotating_changes_the_footprint_of_an_asymmetric_piece() {
        let mut engine = standard(&[Tetromino::T]);
        let before = engine.current().cells();
        engine.apply(Action::Rotate(Spin::Clockwise));
        assert_ne!(engine.current().cells(), before);
    }

    #[test]
    fn a_piece_can_still_rotate_while_flush_against_the_wall() {
        let mut engine = standard(&[Tetromino::I]);
        for _ in 0..20 {
            engine.apply(Action::MoveLeft);
        }
        engine.apply(Action::Rotate(Spin::Clockwise));
        assert_eq!(engine.current().rotation.quarter_turns(), 1);
        assert!(engine.current().cells().iter().all(|c| c.x >= 0));
    }

    #[test]
    fn a_rotation_with_nowhere_to_go_is_refused_rather_than_overlapping_the_stack() {
        let mut engine = engine_of(&[Tetromino::I], 4, 4);
        for row in 0..4 {
            for x in 0..4 {
                if !engine.current().cells().contains(&Point::new(x, row)) {
                    engine
                        .board
                        .set(Point::new(x, row), Cell::Filled(Tetromino::Z));
                }
            }
        }
        let before = *engine.current();
        engine.apply(Action::Rotate(Spin::Clockwise));
        assert_eq!(*engine.current(), before);
    }

    #[test]
    fn gravity_pulls_the_piece_down_one_row_per_interval() {
        let mut engine = standard(&[Tetromino::O]);
        let start = engine.current().origin.y;
        engine.tick(gravity_step(&engine));
        assert_eq!(engine.current().origin.y, start + 1);
    }

    #[test]
    fn a_long_frame_applies_every_gravity_step_it_was_owed() {
        let mut engine = standard(&[Tetromino::O]);
        let start = engine.current().origin.y;
        engine.tick(gravity_step(&engine) * 3);
        assert_eq!(engine.current().origin.y, start + 3);
    }

    #[test]
    fn partial_time_accumulates_instead_of_being_thrown_away() {
        let mut engine = standard(&[Tetromino::O]);
        let start = engine.current().origin.y;
        let half = gravity_step(&engine) / 2;
        engine.tick(half);
        assert_eq!(engine.current().origin.y, start);
        engine.tick(half);
        assert_eq!(engine.current().origin.y, start + 1);
    }

    #[test]
    fn a_paused_game_ignores_both_gravity_and_movement() {
        let mut engine = standard(&[Tetromino::O]);
        engine.apply(Action::TogglePause);
        let frozen = *engine.current();
        engine.tick(gravity_step(&engine) * 5);
        engine.apply(Action::MoveLeft);
        assert_eq!(*engine.current(), frozen);
        assert_eq!(engine.phase(), Phase::Paused);
    }

    #[test]
    fn unpausing_lets_the_game_continue_from_where_it_stopped() {
        let mut engine = standard(&[Tetromino::O]);
        engine.apply(Action::TogglePause);
        engine.apply(Action::TogglePause);
        let before = engine.current().origin.y;
        engine.tick(gravity_step(&engine));
        assert_eq!(engine.current().origin.y, before + 1);
    }

    #[test]
    fn a_hard_drop_lands_the_piece_on_the_floor_and_locks_it() {
        let mut engine = standard(&[Tetromino::O, Tetromino::T]);
        engine.apply(Action::HardDrop);
        assert_eq!(engine.stats().pieces, 1);
        assert_eq!(engine.current().kind, Tetromino::T);
        assert!(!engine.board().is_row_empty(Board::STANDARD_HEIGHT - 1));
    }

    #[test]
    fn a_hard_drop_pays_for_the_distance_actually_fallen() {
        let mut engine = standard(&[Tetromino::O, Tetromino::O]);
        engine.apply(Action::HardDrop);
        assert!(engine.stats().score > 0);
    }

    #[test]
    fn a_soft_drop_moves_one_row_and_earns_a_smaller_reward() {
        let mut engine = standard(&[Tetromino::O, Tetromino::O]);
        let start = engine.current().origin.y;
        engine.apply(Action::SoftDrop);
        assert_eq!(engine.current().origin.y, start + 1);
        assert_eq!(engine.stats().score, 1);
    }

    #[test]
    fn a_soft_drop_against_the_floor_locks_the_piece() {
        let mut engine = engine_of(&[Tetromino::O, Tetromino::O], 4, 3);
        for _ in 0..10 {
            engine.apply(Action::SoftDrop);
        }
        assert!(engine.stats().pieces >= 1);
    }

    #[test]
    fn pieces_stack_on_each_other_rather_than_passing_through() {
        let mut engine = engine_of(&[Tetromino::O, Tetromino::O, Tetromino::O], 4, 10);
        engine.apply(Action::HardDrop);
        engine.apply(Action::HardDrop);
        assert!(!engine.board().is_row_empty(8));
        assert!(!engine.board().is_row_empty(6));
    }

    #[test]
    fn filling_a_row_clears_it_and_awards_points() {
        let mut engine = engine_of(&[Tetromino::O, Tetromino::O, Tetromino::T], 4, 6);
        engine.apply(Action::MoveLeft);
        engine.apply(Action::MoveLeft);
        engine.apply(Action::HardDrop);
        engine.apply(Action::MoveRight);
        engine.apply(Action::MoveRight);
        engine.apply(Action::HardDrop);
        assert_eq!(engine.stats().lines, 2);
        assert!(engine.stats().score >= 300);
    }

    #[test]
    fn clearing_enough_rows_advances_the_level_and_speeds_the_game_up() {
        let mut engine = standard(&[Tetromino::O]);
        let fast = ClassicGravity::default();
        engine.stats.lines = 30;
        engine.stats.level = LinesPerLevel::default().level_for(30);
        assert_eq!(engine.stats().level, 3);
        assert!(fast.interval(engine.stats().level) < fast.interval(0));
    }

    #[test]
    fn the_game_ends_when_a_new_piece_cannot_fit() {
        let mut engine = engine_of(&[Tetromino::O; 12], 4, 4);
        for _ in 0..6 {
            engine.apply(Action::HardDrop);
        }
        assert!(engine.phase().is_over());
    }

    #[test]
    fn a_finished_game_ignores_movement_but_still_accepts_a_restart() {
        let mut engine = engine_of(&[Tetromino::O; 12], 4, 4);
        for _ in 0..6 {
            engine.apply(Action::HardDrop);
        }
        let frozen = *engine.current();
        engine.apply(Action::MoveLeft);
        assert_eq!(*engine.current(), frozen);
        engine.apply(Action::Restart);
        assert_eq!(engine.phase(), Phase::Playing);
    }

    #[test]
    fn restarting_wipes_the_board_and_the_score() {
        let mut engine = standard(&[Tetromino::O, Tetromino::O, Tetromino::O]);
        engine.apply(Action::HardDrop);
        engine.apply(Action::Restart);
        assert_eq!(engine.stats(), Stats::default());
        assert!((0..Board::STANDARD_HEIGHT).all(|row| engine.board().is_row_empty(row)));
    }

    #[test]
    fn the_ghost_shows_the_landing_spot_without_moving_the_real_piece() {
        let engine = standard(&[Tetromino::O]);
        let ghost = engine.ghost();
        assert_eq!(ghost.kind, engine.current().kind);
        assert!(ghost.origin.y > engine.current().origin.y);
        assert_eq!(engine.current().origin.y, 0);
    }

    #[test]
    fn a_hard_drop_puts_the_piece_exactly_where_the_ghost_promised() {
        let mut engine = standard(&[Tetromino::T, Tetromino::T]);
        engine.apply(Action::MoveLeft);
        let promised = engine.ghost().cells();
        engine.apply(Action::HardDrop);
        for cell in promised {
            assert!(engine.board().cell(cell).is_some_and(Cell::is_filled));
        }
    }

    #[test]
    fn the_preview_lists_the_pieces_that_actually_arrive_next() {
        let script = [Tetromino::I, Tetromino::S, Tetromino::Z, Tetromino::L];
        let mut engine = standard(&script);
        assert_eq!(engine.next_pieces(3), script[1..4].to_vec());
        engine.apply(Action::HardDrop);
        assert_eq!(engine.current().kind, Tetromino::S);
    }

    #[test]
    fn quitting_is_not_the_engines_business_and_changes_nothing() {
        let mut engine = standard(&[Tetromino::T]);
        let before = engine.clone();
        engine.apply(Action::Quit);
        assert_eq!(engine.phase(), before.phase());
        assert_eq!(*engine.current(), *before.current());
    }

    #[test]
    fn the_piece_count_rises_by_exactly_one_per_lock() {
        let mut engine = standard(&[Tetromino::O; 6]);
        for expected in 1..=4 {
            engine.apply(Action::HardDrop);
            assert_eq!(engine.stats().pieces, expected);
        }
    }
}
