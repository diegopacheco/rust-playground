pub trait Event {
    fn symbol(&self) -> &str;
    fn value(&self) -> f64;
}

pub struct StockUp {
    value: f64,
    symbol: String,
}

impl StockUp {
    pub fn new(symbol: String, value: f64) -> Self {
        StockUp { value, symbol }
    }
}

impl Event for StockUp {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn value(&self) -> f64 {
        self.value
    }
}

pub struct StockDown {
    value: f64,
    symbol: String,
}

impl StockDown {
    pub fn new(symbol: String, value: f64) -> Self {
        StockDown { value, symbol }
    }
}

impl Event for StockDown {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn value(&self) -> f64 {
        self.value
    }
}
