## Related POCS

- [1. Java Pure Memory - Stock Engine](https://github.com/diegopacheco/java-pocs/tree/master/pocs/stock-matcher-engine)
- [2. Java 25, Kafka, Kafka-Streams](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-25-kafka-streams-windoning-eo-purchases)
- [3. Java 25, Kafka, KsqlDB](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-25-kafka-ksqldb-windoning-eo-purchases)
- [4. Java 25, RocksDB](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-25-rocksdb-windoning-eo-purchases)
- [5. Java 25, Redis and Redis Streams](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-25-redis-windoning-eo-purchases)
- [6. Java 21, Kafka, Flink](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-21-kafka-flink-windoning-eo-purchases)
- [7. Java 21, Kafka, Spark](https://github.com/diegopacheco/java-pocs/tree/master/pocs/java-21-kafka-spark-windoning-eo-purchases)
- [8. Rust 2024, Pure Memory - Stock Engine](https://github.com/diegopacheco/rust-playground/tree/master/in-memory-stock-engine)

## Rationale

- Build an intentionally naive in-memory matcher in Rust to explore baseline costs for stock alerting.
- Keep a quadratic events x rules pass to measure worst-case latency before adding filtering or indexes.
- Use synthetic Nasdaq-like data to benchmark CPU and memory without external dependencies.
- Provide a reference point for future optimizations: rule caps, parallelism, better data structures or languages.
- Serve as a teaching toy for discussing complexity and scaling trade-offs in streaming/trading workloads.

### Build
```bash
cargo build --release
```

### About it

Stock exchange application written in Rust.
Imagine you want to be notified when something happens, some GOOGLE(GOOG) stock went up or down.
There are some simple rules(Equal, GreaterThan,LessThan) when the stock price change. Fake Data generation techniques are used to generate a lot of data for benchmarks.
Such solution would be used for Day Trading applications.

### Design

```mermaid
classDiagram
    class Event {
        <<interface>>
        +symbol() String
        +value() f64
    }

    class StockUp {
        -symbol: String
        -value: f64
        +new(String, f64) StockUp
    }

    class StockDown {
        -symbol: String
        -value: f64
        +new(String, f64) StockDown
    }

    class Predicate {
        <<interface>>
        +matches(Event) bool
    }

    class Equal {
        -symbol: String
        -value: f64
        +new(String, f64) Equal
    }

    class LessThan {
        -symbol: String
        -value: f64
        +new(String, f64) LessThan
    }

    class GreaterThan {
        -symbol: String
        -value: f64
        +new(String, f64) GreaterThan
    }

    class Matcher {
        <<interface>>
        +run(Vec~Event~) Vec~MaterializedMatch~
    }

    class InMemoryMatcher {
        -predicates: Vec~Predicate~
        +new(Vec~Predicate~) InMemoryMatcher
    }

    class MaterializedMatch {
        -match_time: SystemTime
        +new() MaterializedMatch
    }

    class EventGenerator {
        <<interface>>
        +generate(usize) Vec~Event~
    }

    class NasdaqEventGenerator {
        +generate(usize) Vec~Event~
        -create() Event
    }

    class PredicateGenerator {
        <<interface>>
        +generate(usize) Vec~Predicate~
    }

    class UserPredicatesGenerator {
        +generate(usize) Vec~Predicate~
        -create() Predicate
    }

    class Randomizer {
        +value() f64
        +symbol() String
    }

    class Main {
        +main()
        -benchmark(usize)
        -benchmark_cap(usize, usize)
    }

    Event <|.. StockUp
    Event <|.. StockDown
    Predicate <|.. Equal
    Predicate <|.. LessThan
    Predicate <|.. GreaterThan
    Matcher <|.. InMemoryMatcher
    EventGenerator <|.. NasdaqEventGenerator
    PredicateGenerator <|.. UserPredicatesGenerator

    InMemoryMatcher --> Predicate
    InMemoryMatcher --> Event
    InMemoryMatcher --> MaterializedMatch
    MaterializedMatch --> Event
    MaterializedMatch --> Predicate

    Main --> InMemoryMatcher
    Main --> NasdaqEventGenerator
    Main --> UserPredicatesGenerator

    NasdaqEventGenerator --> Randomizer
    NasdaqEventGenerator --> EventGenerator
    NasdaqEventGenerator --> Event

    UserPredicatesGenerator --> Randomizer
    UserPredicatesGenerator --> PredicateGenerator
    UserPredicatesGenerator --> Predicate
```

Color scheme:
* Green: Traits (Interfaces)
* Blue: Predicates (rules)
* Purple: Fake data generation
* Dark blue: Matching engine
* Yellow: Raw events
* Red: Main orchestration and benchmarks

### Benchmark

```bash
>> Benchmarks: CAP 100 rules
Matching 10 events / 100 predicates resulted in: [42] match in 0 ms
Matching 100 events / 100 predicates resulted in: [386] match in 0 ms
Matching 1000 events / 100 predicates resulted in: [3603] match in 0 ms
Matching 10000 events / 100 predicates resulted in: [41599] match in 7 ms
Matching 100000 events / 100 predicates resulted in: [401166] match in 53 ms
Matching 1000000 events / 100 predicates resulted in: [3608692] match in 376 ms
Matching 10000000 events / 100 predicates resulted in: [42284503] match in 3818 ms
>> Benchmarks: NO CAP (rules x events)
Matching 10 events / 10 predicates resulted in: [2] match in 0 ms
Matching 100 events / 100 predicates resulted in: [386] match in 0 ms
Matching 1000 events / 1000 predicates resulted in: [42150] match in 6 ms
Matching 10000 events / 10000 predicates resulted in: [4104344] match in 835 ms
Matching 100000 events / 100000 predicates resulted in: [416615936] match in 94165 ms
```

### How can we optimize and scale

1. Start filtering the rules by users(uuid, id, hash, or symbol, whatever).
2. CAP how many rules a user can have lets say 100 is max.
3. Use thread pool, process all in parallel rather a single thread, is all cpu bound (2 threads per core).
4. Simple things, integers instead of strings for symbols, it would speed up things and use less memory.
5. More machines with some seeding or light coordination(zookeeper like).
6. Instead of processing all at once, make continuous processing, as the events arrive you process.
7. Optimize the runtime with proper flags for allocation and pre-allocate memory
8. Further optimizations in Rust or other optimized languages like Go or Zig.

### CAP 100 rules

Now we can do:
 * 100k in 53 ms
 * 1M in 376m
 * 10M in 3818 ms

Again, single machine, still have 7 other optimizations to play.
