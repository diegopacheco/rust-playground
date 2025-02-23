struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.saturating_add(self.next);
        let current = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(current)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

fn main() {
    let fib = Fibonacci::default()
        .take(10)
        .collect::<Vec<_>>();
    println!("First 10 Fibonacci numbers: {:?}", fib);
}