//! How the next piece is chosen.

use crate::piece::Tetromino;
use crate::rules::rng::Rng;
use std::collections::VecDeque;

/// A source of upcoming tetrominoes.
///
/// The engine only ever asks for the next piece, so the policy behind it — a
/// shuffle bag, uniform noise, or a fixed script in a test — is interchangeable.
pub trait Randomizer {
    /// Produces the next tetromino to spawn.
    fn next_piece(&mut self) -> Tetromino;

    /// The next `count` pieces without consuming them.
    fn peek(&self, count: usize) -> Vec<Tetromino>;
}

/// The seven-bag randomizer: every tetromino appears once before any repeats.
///
/// This is what keeps a game fair. Uniform random choice can withhold the line
/// piece for dozens of spawns; a bag bounds the worst-case drought at twelve
/// pieces, so a player can plan.
#[derive(Debug, Clone)]
pub struct SevenBag<R: Rng> {
    rng: R,
    queue: VecDeque<Tetromino>,
}

impl<R: Rng> SevenBag<R> {
    /// The number of upcoming pieces kept available for display.
    pub const LOOKAHEAD: usize = 5;

    /// Creates a bag drawing from `rng`.
    pub fn new(rng: R) -> Self {
        let mut bag = Self {
            rng,
            queue: VecDeque::new(),
        };
        bag.top_up();
        bag
    }

    fn top_up(&mut self) {
        while self.queue.len() <= Self::LOOKAHEAD {
            let mut batch = Tetromino::ALL;
            for index in (1..batch.len()).rev() {
                batch.swap(index, self.rng.next_below(index + 1));
            }
            self.queue.extend(batch);
        }
    }
}

impl<R: Rng> Randomizer for SevenBag<R> {
    fn next_piece(&mut self) -> Tetromino {
        self.top_up();
        let piece = self.queue.pop_front().unwrap_or(Tetromino::I);
        self.top_up();
        piece
    }

    fn peek(&self, count: usize) -> Vec<Tetromino> {
        self.queue.iter().copied().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::rng::XorShift64;
    use std::collections::HashSet;

    fn bag(seed: u64) -> SevenBag<XorShift64> {
        SevenBag::new(XorShift64::new(seed))
    }

    fn draw(seed: u64, count: usize) -> Vec<Tetromino> {
        let mut bag = bag(seed);
        (0..count).map(|_| bag.next_piece()).collect()
    }

    #[test]
    fn each_group_of_seven_contains_every_piece_exactly_once() {
        let drawn = draw(11, 70);
        for group in drawn.chunks(7) {
            let unique: HashSet<Tetromino> = group.iter().copied().collect();
            assert_eq!(unique.len(), 7, "a bag repeated a piece: {group:?}");
        }
    }

    #[test]
    fn a_player_never_waits_more_than_twelve_pieces_for_a_given_tetromino() {
        let drawn = draw(2026, 700);
        for target in Tetromino::ALL {
            let mut gap = 0;
            let mut worst = 0;
            for piece in &drawn {
                if *piece == target {
                    worst = worst.max(gap);
                    gap = 0;
                } else {
                    gap += 1;
                }
            }
            assert!(worst <= 12, "{target:?} went missing for {worst} pieces");
        }
    }

    #[test]
    fn the_same_seed_produces_the_same_game() {
        assert_eq!(draw(5, 40), draw(5, 40));
    }

    #[test]
    fn different_seeds_produce_different_games() {
        assert_ne!(draw(5, 40), draw(6, 40));
    }

    #[test]
    fn shuffling_actually_reorders_rather_than_returning_the_canonical_order() {
        let orders: HashSet<Vec<Tetromino>> = (0..20).map(|seed| draw(seed, 7)).collect();
        assert!(orders.len() > 1, "every seed produced the same bag order");
    }

    #[test]
    fn peek_shows_what_is_coming_without_consuming_it() {
        let mut bag = bag(3);
        let previewed = bag.peek(SevenBag::<XorShift64>::LOOKAHEAD);
        let drawn: Vec<Tetromino> = (0..previewed.len()).map(|_| bag.next_piece()).collect();
        assert_eq!(previewed, drawn);
    }

    #[test]
    fn the_preview_stays_populated_no_matter_how_many_pieces_are_drawn() {
        let mut bag = bag(8);
        for _ in 0..50 {
            bag.next_piece();
            assert_eq!(
                bag.peek(SevenBag::<XorShift64>::LOOKAHEAD).len(),
                SevenBag::<XorShift64>::LOOKAHEAD
            );
        }
    }
}
