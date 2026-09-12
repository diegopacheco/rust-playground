//! The game loop that ties input, time and rendering together.

use crate::board::Playfield;
use crate::game::{Action, Clock, Engine};
use crate::input::InputSource;
use crate::render::canvas::Canvas;
use crate::render::screen::Screen;
use crate::render::view::View;
use crate::rules::RuleSet;
use std::io;
use std::time::Duration;

/// Whether the loop should keep going.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    /// Run another frame.
    Continue,
    /// Leave the game.
    Quit,
}

/// Drives a game: read intent, advance time, draw.
///
/// Everything it touches is a trait — the input source, the canvas and the
/// clock — so the identical loop that runs in a terminal also runs in a test
/// with a scripted input, an in-memory canvas and a clock the test controls.
#[derive(Debug)]
pub struct GameLoop<R: RuleSet, I: InputSource, C: Canvas, K: Clock> {
    engine: Engine<R>,
    input: I,
    canvas: C,
    clock: K,
    screen: Screen,
    frame: Duration,
}

impl<R: RuleSet, I: InputSource, C: Canvas, K: Clock> GameLoop<R, I, C, K> {
    /// Roughly sixty frames a second.
    pub const FRAME: Duration = Duration::from_millis(16);

    /// Assembles a loop from its parts.
    pub fn new(engine: Engine<R>, input: I, canvas: C, clock: K) -> Self {
        let columns = u16::try_from(engine.board().width()).unwrap_or(0);
        let rows = u16::try_from(engine.board().height()).unwrap_or(0);
        Self {
            engine,
            input,
            canvas,
            clock,
            screen: Screen::for_board(columns, rows),
            frame: Self::FRAME,
        }
    }

    /// The game being driven.
    pub const fn engine(&self) -> &Engine<R> {
        &self.engine
    }

    /// The surface being drawn to.
    pub const fn canvas(&self) -> &C {
        &self.canvas
    }

    /// Runs one frame: input, then time, then a redraw.
    ///
    /// # Errors
    ///
    /// Returns any error from the input source or the canvas.
    pub fn step(&mut self) -> io::Result<Flow> {
        let action = self.input.poll(self.frame)?;
        if action == Some(Action::Quit) {
            return Ok(Flow::Quit);
        }
        if let Some(action) = action {
            self.engine.apply(action);
        }
        let elapsed = self.clock.tick();
        self.engine.tick(elapsed);
        self.draw()?;
        Ok(Flow::Continue)
    }

    /// Draws the current state.
    ///
    /// # Errors
    ///
    /// Returns any error from the canvas.
    pub fn draw(&mut self) -> io::Result<()> {
        self.canvas.clear();
        self.screen.paint(&self.engine.snapshot(), &mut self.canvas);
        self.canvas.present()
    }

    /// Runs frames until the player quits.
    ///
    /// # Errors
    ///
    /// Returns any error from the input source or the canvas.
    pub fn run(&mut self) -> io::Result<()> {
        self.draw()?;
        while self.step()? == Flow::Continue {}
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::game::{ManualClock, Phase};
    use crate::geometry::Offset;
    use crate::input::ScriptedInput;
    use crate::render::canvas::TextCanvas;
    use crate::rules::{ClassicGravity, GravityCurve, StandardRules, XorShift64};

    type Harness = GameLoop<StandardRules<XorShift64>, ScriptedInput, TextCanvas, ManualClock>;

    fn harness(actions: &[Action]) -> Harness {
        let engine = Engine::new(Board::standard(), StandardRules::seeded(2026));
        let (width, height) = Screen::required_size(10, 20);
        GameLoop::new(
            engine,
            ScriptedInput::new(actions.iter().copied()),
            TextCanvas::new(width, height),
            ManualClock::new(),
        )
    }

    #[test]
    fn a_quit_action_stops_the_loop() {
        let mut game = harness(&[Action::Quit]);
        assert_eq!(game.step().unwrap(), Flow::Quit);
    }

    #[test]
    fn running_returns_once_the_player_quits() {
        let mut game = harness(&[Action::MoveLeft, Action::MoveRight, Action::Quit]);
        game.run().unwrap();
        assert_eq!(game.engine().phase(), Phase::Playing);
    }

    #[test]
    fn an_action_reaches_the_engine() {
        let mut game = harness(&[Action::MoveLeft]);
        let before = game.engine().current().origin;
        game.step().unwrap();
        assert_eq!(game.engine().current().origin, before.shifted(Offset::LEFT));
    }

    #[test]
    fn a_frame_with_no_key_pressed_still_advances_gravity() {
        let mut game = harness(&[]);
        game.clock.advance(ClassicGravity::default().interval(0));
        let before = game.engine().current().origin.y;
        game.step().unwrap();
        assert_eq!(game.engine().current().origin.y, before + 1);
    }

    #[test]
    fn no_elapsed_time_means_the_piece_does_not_move_on_its_own() {
        let mut game = harness(&[]);
        let before = *game.engine().current();
        game.step().unwrap();
        assert_eq!(*game.engine().current(), before);
    }

    #[test]
    fn every_frame_leaves_something_drawn_on_the_canvas() {
        let mut game = harness(&[]);
        game.step().unwrap();
        assert!(game.canvas().to_text().contains("TETRIS"));
    }

    #[test]
    fn the_canvas_is_cleared_each_frame_so_old_pieces_do_not_smear() {
        let mut game = harness(&[Action::HardDrop]);
        game.step().unwrap();
        let filled_after_drop = game.canvas().to_text().matches('█').count();
        game.draw().unwrap();
        assert_eq!(
            game.canvas().to_text().matches('█').count(),
            filled_after_drop
        );
    }

    #[test]
    fn the_drawn_score_follows_the_game() {
        let mut game = harness(&[Action::HardDrop]);
        game.step().unwrap();
        assert!(game.engine().stats().score > 0);
        assert!(!game.canvas().to_text().contains("SCORE\n"));
    }

    #[test]
    fn a_full_game_can_be_played_to_its_end_without_a_terminal() {
        let mut script = vec![Action::HardDrop; 200];
        script.push(Action::Quit);
        let mut game = harness(&script);
        game.run().unwrap();
        assert!(game.engine().stats().pieces > 0);
        assert!(game.engine().phase().is_over() || game.engine().stats().score > 0);
    }

    #[test]
    fn quitting_is_not_passed_on_to_the_engine_as_a_move() {
        let mut game = harness(&[Action::Quit]);
        let before = *game.engine().current();
        game.step().unwrap();
        assert_eq!(*game.engine().current(), before);
    }

    #[test]
    fn pausing_through_the_loop_freezes_gravity() {
        let mut game = harness(&[Action::TogglePause]);
        game.step().unwrap();
        let frozen = *game.engine().current();
        game.clock.advance(Duration::from_secs(5));
        game.step().unwrap();
        assert_eq!(*game.engine().current(), frozen);
        assert!(game.canvas().to_text().contains("PAUSED"));
    }
}
