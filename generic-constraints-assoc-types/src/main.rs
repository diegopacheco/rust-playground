trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;
    
    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }
    
    fn get(&self) -> Option<&Self::Item> {
        self.items.last()
    }
}

fn main() {
    let mut stack = Stack { items: Vec::new() };
    stack.add(42);
    println!("Last item: {:?}", stack.get());
}