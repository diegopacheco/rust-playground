use crate::event::{Event, StockDown, StockUp};
use crate::predicates::{Equal, GreaterThan, LessThan, Predicate};
use rand::Rng;

pub trait EventGenerator {
    fn generate(&self, amount: usize) -> Vec<Box<dyn Event>>;
}

pub trait PredicateGenerator {
    fn generate(&self, amount: usize) -> Vec<Box<dyn Predicate>>;
}

pub struct Randomizer;

impl Randomizer {
    pub fn value() -> f64 {
        let mut rng = rand::rng();
        rng.random_range(10.0..9000.0)
    }

    pub fn symbol() -> String {
        let symbols = vec![
            "XOM", "GE", "TM", "PG", "GOOG", "ING", "AAPL", "META", "NFLX", "AMZN", "XOM",
        ];
        let mut rng = rand::rng();
        let result = rng.random_range(1..9);
        symbols[result].to_string()
    }
}

pub struct NasdaqEventGenerator;

impl EventGenerator for NasdaqEventGenerator {
    fn generate(&self, amount: usize) -> Vec<Box<dyn Event>> {
        (0..amount)
            .map(|_| self.create())
            .collect()
    }
}

impl NasdaqEventGenerator {
    fn create(&self) -> Box<dyn Event> {
        let mut rng = rand::rng();
        let result = rng.random_range(1..=2);
        if result == 1 {
            Box::new(StockUp::new(Randomizer::symbol(), Randomizer::value()))
        } else {
            Box::new(StockDown::new(Randomizer::symbol(), Randomizer::value()))
        }
    }
}

pub struct UserPredicatesGenerator;

impl PredicateGenerator for UserPredicatesGenerator {
    fn generate(&self, amount: usize) -> Vec<Box<dyn Predicate>> {
        (0..amount)
            .map(|_| self.create())
            .collect()
    }
}

impl UserPredicatesGenerator {
    fn create(&self) -> Box<dyn Predicate> {
        let mut rng = rand::rng();
        let result = rng.random_range(1..=3);
        match result {
            1 => Box::new(Equal::new(Randomizer::symbol(), Randomizer::value())),
            2 => Box::new(LessThan::new(Randomizer::symbol(), Randomizer::value())),
            _ => Box::new(GreaterThan::new(Randomizer::symbol(), Randomizer::value())),
        }
    }
}
