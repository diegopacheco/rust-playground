use crate::event::Event;
use crate::predicates::Predicate;
use std::time::SystemTime;

pub trait Matcher {
    fn run(&self, events: &[Box<dyn Event>]) -> Vec<MaterializedMatch>;
}

pub struct MaterializedMatch {
    _match_time: SystemTime,
}

impl MaterializedMatch {
    pub fn new() -> Self {
        MaterializedMatch {
            _match_time: SystemTime::now(),
        }
    }
}

pub struct InMemoryMatcher {
    predicates: Vec<Box<dyn Predicate>>,
}

impl InMemoryMatcher {
    pub fn new(predicates: Vec<Box<dyn Predicate>>) -> Self {
        InMemoryMatcher { predicates }
    }
}

impl Matcher for InMemoryMatcher {
    fn run(&self, events: &[Box<dyn Event>]) -> Vec<MaterializedMatch> {
        let mut materialized_matches = Vec::with_capacity(events.len());
        for event in events {
            for predicate in &self.predicates {
                if predicate.matches(event.as_ref()) {
                    materialized_matches.push(MaterializedMatch::new());
                }
            }
        }
        materialized_matches
    }
}
