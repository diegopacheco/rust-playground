pub trait PrimeGenerator {
    fn is_prime(&self, n: u64) -> bool;
    fn next_prime(&mut self) -> u64;
    fn get_primes(&self) -> &Vec<u64>;
}

// Sieve
// concept of "sieve of Eratosthenes", which is a 
// classic algorithm for finding prime numbers.
pub struct Sieve {
    sieve: Vec<bool>,  // true means prime, false means composite
    primes: Vec<u64>,
    limit: u64,
    next_index: usize,
}

impl Sieve {

    fn trial_division(&self, n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }

        let sqrt = (n as f64).sqrt() as u64;
        for i in (3..=sqrt).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn find_next_prime_trial_division(&mut self) -> u64 {
        let mut candidate = if self.next_index == 0 {
            2
        } else {
            self.primes.last().unwrap() + 1
        };

        while !self.trial_division(candidate) {
            candidate += 1;
        }
        self.primes.push(candidate);
        self.next_index += 1;
        candidate
    }

    pub fn new() -> Self {
        let limit = 1000;
        let mut sieve = vec![true; limit as usize];
        sieve[0] = false;
        sieve[1] = false;
        
        // Apply the sieve
        for i in 2..((limit as f64).sqrt() as usize + 1) {
            if sieve[i] {
                let mut j = i * i;
                while j < limit as usize {
                    sieve[j] = false;
                    j += i;
                }
            }
        }
        
        // Initialize primes vector
        let mut primes = Vec::new();
        for i in 0..limit as usize {
            if sieve[i] {
                primes.push(i as u64);
            }
        }

        Sieve {
            sieve,
            primes,
            limit,
            next_index: 0,
        }
    }
}

impl PrimeGenerator for Sieve {
    fn is_prime(&self, n: u64) -> bool {
        if n >= self.limit {
            // Fall back to trial division for numbers beyond our sieve
            return self.trial_division(n);
        }
        self.sieve[n as usize]
    }

    fn next_prime(&mut self) -> u64 {
        if self.next_index < self.primes.len() {
            let prime = self.primes[self.next_index];
            self.next_index += 1;
            prime
        } else {
            // Fall back to trial division for numbers beyond our sieve
            self.find_next_prime_trial_division()
        }
    }

    fn get_primes(&self) -> &Vec<u64> {
        &self.primes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_few_primes() {
        let mut sieve = Sieve::new();
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19];
        
        let generated: Vec<u64> = (0..8).map(|_| sieve.next_prime()).collect();
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_is_prime() {
        let mut sieve = Sieve::new();
        // Generate first few primes
        for _ in 0..5 {
            sieve.next_prime();
        }

        assert!(sieve.is_prime(11));
        assert!(sieve.is_prime(13));
        assert!(!sieve.is_prime(12));
        assert!(!sieve.is_prime(1));
        assert!(!sieve.is_prime(0));
    }

    #[test]
    fn test_get_primes() {
        let mut sieve = Sieve::new();
        // Generate first 4 primes
        for _ in 0..4 {
            sieve.next_prime();
        }

        let primes = sieve.get_primes();
        assert!(primes.contains(&2));
        assert!(primes.contains(&3));
        assert!(primes.contains(&5));
        assert!(primes.contains(&7));
        assert!(!primes.contains(&4));
    }
}