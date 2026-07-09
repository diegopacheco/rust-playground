# Rust Cargo Nextest Fun

Small Rust 1.9x Cargo project using `cargo-nextest`.

## Requirements

- Rust 1.90 or newer
- Cargo
- `cargo-nextest`

## Build

```bash
./run.sh
```

## Test

```bash
./test.sh
```

`test.sh` installs `cargo-nextest` with Cargo when it is not already installed, then runs:

```bash
cargo nextest run
```

The project has 30 tests in `tests/basic.rs`.

## Result

```
❯ ./test.sh
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
────────────
 Nextest run ID ecad281d-1e2a-4071-9f62-e36b841f9b1e with nextest profile: default
    Starting 30 tests across 2 binaries
        PASS [   0.013s] rust-cargo-nexttest-fun::basic adds_negative_numbers
        PASS [   0.013s] rust-cargo-nexttest-fun::basic detects_even_number
        PASS [   0.013s] rust-cargo-nexttest-fun::basic divides_evenly
        PASS [   0.013s] rust-cargo-nexttest-fun::basic clamps_high_value
        PASS [   0.013s] rust-cargo-nexttest-fun::basic factorial_of_one
        PASS [   0.014s] rust-cargo-nexttest-fun::basic counts_empty_words
        PASS [   0.014s] rust-cargo-nexttest-fun::basic adds_positive_numbers
        PASS [   0.014s] rust-cargo-nexttest-fun::basic adds_mixed_numbers
        PASS [   0.014s] rust-cargo-nexttest-fun::basic counts_single_word
        PASS [   0.015s] rust-cargo-nexttest-fun::basic clamps_low_value
        PASS [   0.015s] rust-cargo-nexttest-fun::basic factorial_of_zero
        PASS [   0.015s] rust-cargo-nexttest-fun::basic detects_negative_even_number
        PASS [   0.016s] rust-cargo-nexttest-fun::basic detects_odd_number
        PASS [   0.016s] rust-cargo-nexttest-fun::basic counts_many_words
        PASS [   0.016s] rust-cargo-nexttest-fun::basic factorial_of_five
        PASS [   0.017s] rust-cargo-nexttest-fun::basic divides_with_integer_result
        PASS [   0.010s] rust-cargo-nexttest-fun::basic multiplies_by_zero
        PASS [   0.011s] rust-cargo-nexttest-fun::basic keeps_value_inside_bounds
        PASS [   0.011s] rust-cargo-nexttest-fun::basic multiplies_positive_numbers
        PASS [   0.011s] rust-cargo-nexttest-fun::basic multiplies_negative_numbers
        PASS [   0.010s] rust-cargo-nexttest-fun::basic subtracts_positive_numbers
        PASS [   0.011s] rust-cargo-nexttest-fun::basic subtracts_negative_numbers
        PASS [   0.010s] rust-cargo-nexttest-fun::basic title_cases_mixed_case
        PASS [   0.010s] rust-cargo-nexttest-fun::basic subtracts_to_negative
        PASS [   0.012s] rust-cargo-nexttest-fun::basic reverses_empty_text
        PASS [   0.012s] rust-cargo-nexttest-fun::basic reverses_words_with_space
        PASS [   0.013s] rust-cargo-nexttest-fun::basic reverses_ascii_text
        PASS [   0.011s] rust-cargo-nexttest-fun::basic title_cases_multiple_words
        PASS [   0.013s] rust-cargo-nexttest-fun::basic skips_division_by_zero
        PASS [   0.011s] rust-cargo-nexttest-fun::basic title_cases_single_word
────────────
     Summary [   0.028s] 30 tests run: 30 passed, 0 skipped
```