fn main() {
    slice_patterns_1(&["","",""]); 
    slice_patterns_1(&[" ","ok"]); 
    slice_patterns_1(&[]); 

    slice_patterns_2(&[]); 
    slice_patterns_2(&["Hello","World","!","Rust"]); 
    slice_patterns_2(&["Foo","Bar","Rust","Rust"]); 
    
    slice_patterns_3(&["Foo","Bar","Rust","!"]); 
    slice_patterns_3(&["asASAASADGDFADSZZZ","zzz"]); 
}

fn slice_patterns_1(words: &[&str]) {
    match words {
        [] => println!("empty slice!"),
        [one] => println!("one element: {:?}", one),
        [one, two] => println!("two elements: {:?} {:?}", one, two),
        _ => println!("I'm not sure how many elements!"),
    }
}

fn slice_patterns_2(words: &[&str]) {
    match words {
        ["Hello", "World", "!", ..] => println!("Hello World!"),
        ["Foo", "Bar", ..] => println!("Baz"),
        rest => println!("{:?}", rest),
    }
}

fn slice_patterns_3(words: &[&str]) {
    match words {
        // Ignore everything but the last element, which must be "!".
        [.., "!"] => println!("!!!"),
        // `start` is a slice of everything except the last element, which must be "z".
        [start @ .., "z"] => println!("starts with: {:?}", start),
        // `end` is a slice of everything but the first element, which must be "a".
        ["a", end @ ..] => println!("ends with: {:?}", end),
        rest => println!("{:?}", rest),
    }
}