# rust-playground

Hands-on Rust POCs: the language itself, the std library, crates, frameworks, databases and tooling. Every project is a standalone cargo project and most ship with their own README.

## 🦀 Language Basics

Rust syntax from zero: variables, loops, tuples, arrays and pattern matching.
The smallest programs you can build and run.

* [hello_world](hello_world/) - The first cargo project
* [guessing_game](guessing_game/) - The Rust book guessing game with rand
* [lang_basics](lang_basics/) - Core syntax tour
* [basic_lang](basic_lang/) - Core syntax with tests
* [rust-fun](rust-fun/) - Plain rustc files, no cargo
* [rust-crash-course](rust-crash-course/) - Many small topics as modules called from main
* [repetition-loop-rust](repetition-loop-rust/) - loop, while and for
* [tuples](tuples/) - Tuples and destructuring
* [array-and-slices](array-and-slices/) - Arrays and slices
* [arrays-fun](arrays-fun/) - Arrays filled with random values
* [struct-2-dots-fun](struct-2-dots-fun/) - Struct update syntax with `..`
* [if_let_fun](if_let_fun/) - if let
* [pattern-matching-guards](pattern-matching-guards/) - match with guards
* [subslice_patterns_fun](subslice_patterns_fun/) - Subslice patterns in match
* [underscore_and_type_number](underscore_and_type_number/) - Underscores in numbers and typed literals
* [typealias-fun](typealias-fun/) - Type aliases
* [alias-with-type](alias-with-type/) - Aliases with the type keyword
* [conversions-fun](conversions-fun/) - Casting and conversions between types
* [rust-byte-string](rust-byte-string/) - Byte strings
* [bytes-string-back](bytes-string-back/) - Bytes to String and back
* [dbg_macro_fun](dbg_macro_fun/) - The dbg! macro
* [immutable-vs-muttable](immutable-vs-muttable/) - Immutable vs mutable bindings
* [mutability-freezing-scope](mutability-freezing-scope/) - Freezing a binding by shadowing it in a scope
* [crazy-stuff](crazy-stuff/) - Weird code that still compiles: `return return return`, nested closures
* [dynamic-fib](dynamic-fib/) - Fibonacci with dynamic programming
* [prime-generator](prime-generator/) - Prime number generator

## 🆕 Editions & Releases

New language features, one Rust release at a time.

* [rust-1.85-edition-2024](rust-1.85-edition-2024/) - Rust 1.85 and edition 2024
* [rust-1.85-edition-2024-asyncfn](rust-1.85-edition-2024-asyncfn/) - Async closures in edition 2024
* [rust-1.85-edition-2024-fanout-collections](rust-1.85-edition-2024-fanout-collections/) - Fan-out over collections in edition 2024
* [rust-1.88-let-chains](rust-1.88-let-chains/) - let chains from Rust 1.88
* [rust-1.90-fun](rust-1.90-fun/) - Rust 1.90 features

## 🔐 Ownership, Borrowing & Smart Pointers

Rust's memory model: who owns a value, who can borrow it and for how long.
Box, Rc, Arc and the Cell types cover the cases the borrow checker can't prove alone.

* [borrow_and_ownership](borrow_and_ownership/) - Ownership and borrowing rules
* [borrower-ownership-simple-samples](borrower-ownership-simple-samples/) - Small ownership and borrowing cases
* [annoying-borrow-checker](annoying-borrow-checker/) - Borrow checker errors and how to fix them
* [move](move/) - Move semantics
* [values-by-reference](values-by-reference/) - Passing values by reference
* [values-by-reference-mutable](values-by-reference-mutable/) - Passing values by mutable reference
* [box-rc-fun](box-rc-fun/) - Box and Rc
* [dynamic-data-structure-box](dynamic-data-structure-box/) - Recursive data structures with Box
* [rc-fun](rc-fun/) - Rc reference counting
* [rc-rust](rc-rust/) - Rc shared ownership
* [rc-cell-fun](rc-cell-fun/) - Rc with RefCell
* [cell-rust](cell-rust/) - Cell interior mutability
* [ref-cell-rust](ref-cell-rust/) - RefCell interior mutability
* [refcell-fun](refcell-fun/) - RefCell borrow rules at runtime
* [Arc-fun](Arc-fun/) - Arc across threads
* [arc-rust](arc-rust/) - Arc shared ownership
* [linked-lists-oh-boy](linked-lists-oh-boy/) - Linked lists, the hard way in Rust

## 🧬 Traits, Generics & Types

Traits define shared behavior, generics make code work for many types.
Plus the standard conversion traits and some OOP-style patterns.

* [traits-fun](traits-fun/) - Traits and impls
* [Add-trait-fun](Add-trait-fun/) - Operator overloading with Add
* [from-trait-fun](from-trait-fun/) - The From trait
* [into-trait-fun](into-trait-fun/) - The Into trait
* [try_from_fun](try_from_fun/) - TryFrom
* [try-from-trait-fun](try-from-trait-fun/) - TryFrom and TryInto
* [default-default-fun](default-default-fun/) - Default and derive-new
* [smart_default_fun](smart_default_fun/) - smart-default derive
* [getset_fun](getset_fun/) - Generated getters and setters with getset
* [closures-in-traits-rust](closures-in-traits-rust/) - Closures stored in traits
* [rust-cli-traits](rust-cli-traits/) - Traits in a CLI app
* [generic-functions](generic-functions/) - Generic functions
* [function-generics-multiple-traits](function-generics-multiple-traits/) - Generics bound by several traits
* [function-generics-where-traits](function-generics-where-traits/) - where clauses
* [generic-constraints-assoc-types](generic-constraints-assoc-types/) - Associated types as constraints
* [generic-const](generic-const/) - Const generics
* [generic-array_fun](generic-array_fun/) - generic-array crate
* [const-crazy](const-crazy/) - const items and const expressions
* [const-function-and-eval](const-function-and-eval/) - const fn and compile-time evaluation
* [trait_eval_fun](trait_eval_fun/) - Computing at compile time with traits
* [impls-fun](impls-fun/) - Checking if a type implements a trait with impls
* [query_interface_fun](query_interface_fun/) - Dynamic trait queries with query_interface
* [static-annotations](static-annotations/) - 'static lifetime annotations
* [oop-ish-rust](oop-ish-rust/) - OOP patterns in Rust
* [builder-pattern-rust](builder-pattern-rust/) - The builder pattern
* [state-machine-enum-fun](state-machine-enum-fun/) - A state machine with enums
* [memory-layout-packed](memory-layout-packed/) - Struct memory layout and repr(packed)

## 🪄 Macros

Code that writes code, at compile time.

* [macros](macros/) - Declarative macros
* [macros_fun](macros_fun/) - macro_rules!
* [macro-multi-trait-impl-fun](macro-multi-trait-impl-fun/) - One macro implementing many traits
* [maplit-fun](maplit-fun/) - Map literals with maplit
* [lazy_static_fun](lazy_static_fun/) - Lazy statics with lazy_static
* [once_cell_fun](once_cell_fun/) - Lazy values with once_cell

## 🔁 Closures, Iterators & Functional

Closures capture their environment, iterators chain lazy transformations.

* [closures-in-rust](closures-in-rust/) - Closures
* [little-high-order-funcs](little-high-order-funcs/) - Higher-order functions
* [collect-fun](collect-fun/) - collect into many collection types
* [fibonacci-iterator](fibonacci-iterator/) - A custom Fibonacci iterator
* [iter-extends-fun](iter-extends-fun/) - Extending iterators
* [iterate-fun](iterate-fun/) - Iterators with iterate
* [slice-window-and-chunks](slice-window-and-chunks/) - windows and chunks over slices

## 📚 Collections & Data Structures

Vectors, maps, sets and queues from std and from crates.
Some structures built by hand.

* [vec-fun](vec-fun/) - Vec basics
* [vec-adventures](vec-adventures/) - More Vec operations
* [searching-in-vecs](searching-in-vecs/) - Searching inside vectors
* [maps-collection-fun](maps-collection-fun/) - HashMap
* [maps-mutable-collections](maps-mutable-collections/) - Mutating maps in place
* [sets-collections-fun](sets-collections-fun/) - HashSet and BTreeSet
* [linked_hash_set_fun](linked_hash_set_fun/) - Insertion-ordered sets with linked_hash_set
* [heap_priority_queue_fun](heap_priority_queue_fun/) - Priority queues
* [dashmap_fun](dashmap_fun/) - Concurrent hash map with DashMap
* [ring-buffer](ring-buffer/) - A ring buffer from scratch
* [in-memory-stock-engine](in-memory-stock-engine/) - In-memory stock order matching engine

## 🧠 Memory & Unsafe

Allocators, arenas, raw pointers and unsafe code.

* [allocator](allocator/) - A custom global allocator
* [arena-allocator](arena-allocator/) - An arena allocator
* [boxing-arena-fun](boxing-arena-fun/) - boxing-arena crate
* [raw-unsafe-pointers](raw-unsafe-pointers/) - Raw pointers and unsafe
* [pointer-arithmetic-fun](pointer-arithmetic-fun/) - Pointer arithmetic with libc

## ❗ Error Handling

Result, the ? operator, custom errors and panics.

* [result-fun](result-fun/) - Result basics
* [error-handling-rust](error-handling-rust/) - Error handling patterns
* [error-handling-custom-types](error-handling-custom-types/) - Custom error types
* [rust-simple-error](rust-simple-error/) - simple-error crate
* [anyhow-fun](anyhow-fun/) - anyhow
* [error-chain-rust](error-chain-rust/) - error-chain
* [error-chain-pattern-matcher-rust](error-chain-pattern-matcher-rust/) - Matching on error-chain errors
* [error-chain-pattern-matcher-explicit-rust](error-chain-pattern-matcher-explicit-rust/) - Explicit matching on error-chain errors
* [throw-fun](throw-fun/) - Errors with a stack trace using throw
* [panic-attack-fun](panic-attack-fun/) - Panics and how to handle them

## 🧵 Concurrency & Threads

Threads, channels, locks and data parallelism.
Plus loom to test every interleaving of concurrent code.

* [thread-simple](thread-simple/) - Spawning a thread
* [threads-rust-fun](threads-rust-fun/) - Threads and join handles
* [threads-dont-borrow](threads-dont-borrow/) - Why threads can't borrow from the stack
* [thread-channel-fun](thread-channel-fun/) - Threads talking over channels
* [channels-fun-rust](channels-fun-rust/) - mpsc channels
* [thread-pool-fun](thread-pool-fun/) - A thread pool
* [mutex-fun](mutex-fun/) - Mutex
* [mutex-fun-2](mutex-fun-2/) - Mutex with Arc
* [crossbeam-fun](crossbeam-fun/) - crossbeam scoped threads and channels
* [rayon-fun](rayon-fun/) - Data parallelism with rayon
* [futures_cpupool_fun](futures_cpupool_fun/) - futures-cpupool
* [loom-fun](loom-fun/) - Model checking concurrency with loom

## ⚡ Async Runtimes

Async Rust needs a runtime to poll futures.
Tokio is the default, others trade features for thread-per-core speed.

* [tokyo-fun](tokyo-fun/) - Tokio
* [futures-fun](futures-fun/) - futures crate
* [async_std_fun](async_std_fun/) - async-std
* [streams-fun](streams-fun/) - Async streams
* [glommio-fun](glommio-fun/) - Glommio thread-per-core with io_uring
* [monoio-fun](monoio-fun/) - Monoio thread-per-core runtime
* [scipio-fun](scipio-fun/) - Scipio, the old name of Glommio
* [dial9-fun](dial9-fun/) - Tokio runtime telemetry and traces with dial9

## 🎭 Actors

Actors own their state and talk only by messages.

* [ractor-fun](ractor-fun/) - ractor
* [riker-fun](riker-fun/) - riker
* [xactor_fun](xactor_fun/) - xactor
* [bastion-fun](bastion-fun/) - bastion supervision trees
* [lunatic-fun](lunatic-fun/) - lunatic, Erlang-like processes on WebAssembly

## 🌐 Web Frameworks & HTTP

HTTP servers, clients and full web frameworks.

* [hello-world-actix](hello-world-actix/) - Actix Web hello world
* [actix-web-tokio-rest](actix-web-tokio-rest/) - REST API with Actix Web and Tokio
* [axum-fun](axum-fun/) - Axum
* [hello-rocket](hello-rocket/) - Rocket hello world
* [rocket_fun](rocket_fun/) - Rocket
* [tide-fun](tide-fun/) - Tide
* [salvo-fun](salvo-fun/) - Salvo
* [wrap-fun](wrap-fun/) - Warp
* [nickel-fun](nickel-fun/) - Nickel
* [obsidian-fun](obsidian-fun/) - Obsidian
* [hyper-fun](hyper-fun/) - Hyper
* [reqwest-fun](reqwest-fun/) - HTTP client with reqwest
* [surf-fun](surf-fun/) - HTTP client with surf
* [simple-http-server-nolibs](simple-http-server-nolibs/) - HTTP server with no libraries
* [tcp-server-client-simple](tcp-server-client-simple/) - TCP server and client
* [socket-calculator](socket-calculator/) - Calculator over a TCP socket
* [tonic](tonic/) - gRPC with tonic
* [juniper-graphql-fun](juniper-graphql-fun/) - GraphQL server with Juniper on Actix
* [graphql-rust-client-fun](graphql-rust-client-fun/) - GraphQL client with graphql_client
* [synthetic-data-gen](synthetic-data-gen/) - Axum service generating synthetic test data

## 🏗️ Microservices & Cloud

Services packaged for containers, Kubernetes and observability.

* [rust-microservice](rust-microservice/) - Microservice with contract, DAO, migrations and Postgres
* [rust-docker](rust-docker/) - Rust in Docker and Kubernetes
* [k8s-openapi-fun](k8s-openapi-fun/) - Kubernetes API types with k8s-openapi
* [kube-fun](kube-fun/) - Kubernetes client with kube-rs
* [open-telemetry-rust-fun](open-telemetry-rust-fun/) - Custom OpenTelemetry metrics from Actix Web

## 🗄️ Databases

Drivers, ORMs, proxies and embedded storage engines.

* [pg-fun](pg-fun/) - Postgres driver
* [diesel-postgres](diesel-postgres/) - Diesel ORM on Postgres
* [diesel-mysql](diesel-mysql/) - Diesel ORM on MySQL
* [sqlx-fun](sqlx-fun/) - sqlx with Tide
* [pgdog-fun](pgdog-fun/) - pgdog, a Rust Postgres proxy
* [sqlite-fun](sqlite-fun/) - SQLite
* [sqlite-manager](sqlite-manager/) - SQLite manager CLI: REPL, dumps, manifests and sql-pipe
* [honker-fun](honker-fun/) - honker, a durable queue on SQLite
* [stoolap_fun](stoolap_fun/) - stoolap, an embedded SQL database in Rust
* [cassandra-fun](cassandra-fun/) - Cassandra with cdrs
* [redis-fun](redis-fun/) - Redis
* [rocksdb-fun](rocksdb-fun/) - RocksDB
* [turbokv-fun](turbokv-fun/) - TurboKV embedded key-value store
* [bf-tree-fun](bf-tree-fun/) - Bf-Tree, a read-write optimized index
* [sonic_client-fun](sonic_client-fun/) - Sonic search backend client

## 📨 Messaging

Brokers and queues for sending events between services.

* [kafka-fun](kafka-fun/) - Kafka producer and consumer
* [nats-rs-fun](nats-rs-fun/) - NATS
* [mqtt-mosquitto-local](mqtt-mosquitto-local/) - MQTT with a local Mosquitto

## 📊 Data Processing & Performance

Crunching lots of rows fast, and measuring how fast.

* [100-million-row-challenge-rust](100-million-row-challenge-rust/) - The 100 Million Row Challenge with memmap and ahash
* [arrow-datafusion-fun](arrow-datafusion-fun/) - Apache Arrow and DataFusion SQL
* [criterion-fun](criterion-fun/) - Benchmarks with criterion
* [rust-app-cargo-flamegraph-fun-app](rust-app-cargo-flamegraph-fun-app/) - Profiling with cargo flamegraph

## 🧾 Serialization & Parsing

Turning bytes into data and back: JSON, TOML, RON and custom grammars.

* [serde-fun](serde-fun/) - serde
* [json-simple-serde](json-simple-serde/) - JSON with serde_json
* [json-parser-nolibs](json-parser-nolibs/) - JSON parser with no libraries
* [toml-fun](toml-fun/) - TOML
* [ron_fun](ron_fun/) - RON
* [rust-parsers](rust-parsers/) - XML, JSON and regex parsing
* [nom](nom/) - Parser combinators with nom
* [nom_locate_fun](nom_locate_fun/) - nom with source locations
* [glue-fun](glue-fun/) - Parser combinators with glue
* [lisp-parser](lisp-parser/) - A Lisp parser
* [state-machine-parser](state-machine-parser/) - A parser as a state machine
* [tree-sitter-fun](tree-sitter-fun/) - tree-sitter
* [meval_fun](meval_fun/) - Math expression evaluation with meval
* [url-fun](url-fun/) - URL parsing
* [unicode-xid-fun](unicode-xid-fun/) - Unicode identifiers with unicode-xid
* [case-run](case-run/) - Case conversion with case
* [twoway-fun](twoway-fun/) - Fast substring search with twoway

## 🛡️ Security & Crypto

Hashing, encryption and authorization policies.

* [aes-encryption-fun](aes-encryption-fun/) - AES encryption
* [md5-fun](md5-fun/) - MD5 hashing
* [cedar-policy-fun](cedar-policy-fun/) - Authorization with AWS Cedar policies
* [z3-solver-fun](z3-solver-fun/) - Z3 SMT solver

## 🧰 Utilities & System

Handy crates for files, config, IDs and the machine you run on.

* [fs-fun](fs-fun/) - File system
* [file-processing-struct](file-processing-struct/) - Reading a file into structs
* [tempfile-fun](tempfile-fun/) - Temp files
* [flate2_tfun](flate2_tfun/) - gzip and deflate with flate2
* [chrono-fun](chrono-fun/) - Dates and times with chrono
* [envmnt_fun](envmnt_fun/) - Environment variables with envmnt
* [sonyflake-fun](sonyflake-fun/) - Distributed unique IDs with sonyflake
* [netstat-fun](netstat-fun/) - Network sockets with netstat
* [battery_fun](battery_fun/) - Battery info
* [rust_info_fun](rust_info_fun/) - Rust toolchain info
* [tabwriter_fun](tabwriter_fun/) - Aligned text columns with tabwriter
* [css-style-fun](css-style-fun/) - CSS styles with css-style

## 📝 Logging

* [log_fun](log_fun/) - log with simplelog
* [log4rs_fun](log4rs_fun/) - log4rs
* [fern_fun](fern_fun/) - fern
* [flexi_logger_fun](flexi_logger_fun/) - flexi_logger
* [stlog_fun](stlog_fun/) - stlog

## 💻 CLI & Terminal UI

Argument parsers and full-screen terminal apps.

* [clap-fun](clap-fun/) - clap
* [argh-fun](argh-fun/) - argh
* [abscissa_cool_app](abscissa_cool_app/) - Abscissa app framework
* [cursive-fun](cursive-fun/) - TUI with cursive
* [tui-fun](tui-fun/) - TUI with tui-rs
* [rust-prettier-build](rust-prettier-build/) - Tetris in the terminal with crossterm

## 🖥️ Desktop GUI & Graphics

Native windows, GPU UIs, games and creative coding.

* [iced-fun](iced-fun/) - iced
* [gtk_fun](gtk_fun/) - GTK
* [gpui_fun](gpui_fun/) - GPUI, the Zed UI framework
* [azul_fun](azul_fun/) - Azul
* [tauri-simple-fun](tauri-simple-fun/) - Tauri desktop app
* [nannou-fun](nannou-fun/) - Creative coding with nannou
* [bevy-pong](bevy-pong/) - Pong with Bevy

## 🕸️ WebAssembly & Frontend

Rust compiled to WebAssembly, running in the browser or in a host.

* [yew-app](yew-app/) - Yew
* [Leptos-Fun](Leptos-Fun/) - Leptos with trunk
* [sauron-fun](sauron-fun/) - Sauron
* [perseus-fun](perseus-fun/) - Perseus
* [wasi-fun-rs](wasi-fun-rs/) - WASI target
* [extism-fun](extism-fun/) - Wasm plugins with Extism

## 🔗 Interop & FFI

Rust calling other languages and other languages calling Rust.

* [c-rust-interop](c-rust-interop/) - C calling Rust
* [rust-c-interop](rust-c-interop/) - Rust calling C
* [rust-calling-zig-fun](rust-calling-zig-fun/) - Rust calling Zig
* [jni-fun](jni-fun/) - Java calling Rust with JNI
* [PyO3-Fun](PyO3-Fun/) - Python calling Rust with PyO3
* [rust-python-fun](rust-python-fun/) - Python inside Rust with RustPython
* [neon-fun](neon-fun/) - Node.js calling Rust with Neon
* [embed](embed/) - Rust library called from Python, Ruby and Node

## 📦 Cargo & Tooling

Libraries, workspaces, scripts and build tooling.

* [adder](adder/) - Library crate with integration tests
* [cargo-lib-simple-fun](cargo-lib-simple-fun/) - A library and its user crate
* [simple-lib-rs](simple-lib-rs/) - A library crate
* [rust-lib](rust-lib/) - A library and its user crate
* [cargo-make-fun](cargo-make-fun/) - Task runner with cargo-make
* [cargo-scripts](cargo-scripts/) - Rust scripts with cargo-script

## ✅ Testing

Unit tests, property tests, snapshots, BDD, mutation testing and mocks.

* [rust-cargo-nexttest-fun](rust-cargo-nexttest-fun/) - cargo-nextest runner
* [rspec-fun](rspec-fun/) - RSpec-style tests
* [test_especulate_fun](test_especulate_fun/) - speculate
* [test_cucumber_fun](test_cucumber_fun/) - BDD with cucumber
* [test_insta_fun](test_insta_fun/) - Snapshot tests with insta
* [test_mutagen_fun](test_mutagen_fun/) - Mutation testing with mutagen
* [test_proptest_fun](test_proptest_fun/) - Property tests with proptest
* [testing-proptest-fun](testing-proptest-fun/) - More property tests with proptest
* [quickcheck_fun](quickcheck_fun/) - Property tests with quickcheck
* [test_quickcheck](test_quickcheck/) - quickcheck with macros
* [mock_mockall_fun](mock_mockall_fun/) - Mocks with mockall
* [mock_derive_fun](mock_derive_fun/) - Mocks with mock_derive
* [mock_mockit_fun](mock_mockit_fun/) - Mocks with mock-it
* [mock_mocktopus_fun](mock_mocktopus_fun/) - Mocks with mocktopus
* [mock_mockiato_fun](mock_mockiato_fun/) - Mocks with mockiato
