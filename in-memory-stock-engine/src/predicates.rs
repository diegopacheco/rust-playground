use crate::event::Event;

pub trait Predicate {
    fn matches(&self, event: &dyn Event) -> bool;
}

pub struct Equal {
    value: f64,
    symbol: String,
}

impl Equal {
    pub fn new(symbol: String, value: f64) -> Self {
        Equal { value, symbol }
    }
}

impl Predicate for Equal {
    fn matches(&self, event: &dyn Event) -> bool {
        if self.symbol != event.symbol() {
            return false;
        }
        self.value == event.value()
    }
}

pub struct GreaterThan {
    symbol: String,
    value: f64,
}

impl GreaterThan {
    pub fn new(symbol: String, value: f64) -> Self {
        GreaterThan { symbol, value }
    }
}

impl Predicate for GreaterThan {
    fn matches(&self, event: &dyn Event) -> bool {
        if self.symbol != event.symbol() {
            return false;
        }
        event.value() >= self.value
    }
}

pub struct LessThan {
    symbol: String,
    value: f64,
}

impl LessThan {
    pub fn new(symbol: String, value: f64) -> Self {
        LessThan { symbol, value }
    }
}

impl Predicate for LessThan {
    fn matches(&self, event: &dyn Event) -> bool {
        if self.symbol != event.symbol() {
            return false;
        }
        event.value() <= self.value
    }
}
