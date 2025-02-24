### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

```
❯ cargo test
   Compiling main v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/prime-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/main.rs (target/debug/deps/main-d15d8e36614875e0)

running 3 tests
test prime::tests::test_first_few_primes ... ok
test prime::tests::test_is_prime ... ok
test prime::tests::test_get_primes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
❯ cargo run
   Compiling main v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/prime-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/main`
First 10 prime numbers:
Found prime: 2
Found prime: 3
Found prime: 5
Found prime: 7
Found prime: 11
Found prime: 13
Found prime: 17
Found prime: 19
Found prime: 23
Found prime: 29

Checking some numbers for primality:
97 is prime

All generated primes so far:
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997]
```