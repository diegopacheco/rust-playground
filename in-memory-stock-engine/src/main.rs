mod engine;
mod event;
mod generators;
mod predicates;

use engine::{InMemoryMatcher, Matcher};
use generators::{EventGenerator, NasdaqEventGenerator, PredicateGenerator, UserPredicatesGenerator};
use std::time::Instant;

fn main() {
    println!(">> Benchmarks: CAP 100 rules ");
    benchmark_cap(10, 100);
    benchmark_cap(100, 100);
    benchmark_cap(1_000, 100);
    benchmark_cap(10_000, 100);
    benchmark_cap(100_000, 100);
    benchmark_cap(1_000_000, 100);
    benchmark_cap(10_000_000, 100);

    println!(">> Benchmarks: NO CAP (rules x events) ");
    benchmark(10);
    benchmark(100);
    benchmark(1_000);
    benchmark(10_000);
    benchmark(100_000);
}

fn benchmark(amount: usize) {
    benchmark_cap(amount, amount);
}

fn benchmark_cap(amount_events: usize, amount_rules: usize) {
    let predicates_generator = UserPredicatesGenerator;
    let predicates = predicates_generator.generate(amount_rules);

    let event_generator = NasdaqEventGenerator;
    let events = event_generator.generate(amount_events);

    let matcher = InMemoryMatcher::new(predicates);

    let start = Instant::now();
    let matches = matcher.run(&events);
    let duration = start.elapsed();

    println!(
        "Matching {} events / {} predicates resulted in: [{}] match in {} ms",
        amount_events,
        amount_rules,
        matches.len(),
        duration.as_millis()
    );
}
