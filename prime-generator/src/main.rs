mod prime;
use prime::{Sieve,PrimeGenerator};

fn main() {
    let mut sieve = Sieve::new();
    
    println!("First 10 prime numbers:");
    for _ in 0..10 {
        let prime = sieve.next_prime();
        println!("Found prime: {}", prime);
    }

    println!("\nChecking some numbers for primality:");
    for n in 90..100 {
        if sieve.is_prime(n) {
            println!("{} is prime", n);
        }
    }

    println!("\nAll generated primes so far:");
    let primes = sieve.get_primes();
    println!("{:?}", primes);
}