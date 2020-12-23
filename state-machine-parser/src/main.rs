use crate::State::{Upper, Comment, Lower, Normal};

#[derive(Copy,Clone)]
enum State{
    Normal,
    Comment,
    Upper,
    Lower
}

fn state_machine(state:&State,c:char) -> (Option<char>,State){
    match (state,c) {
        (Normal,'#') => (None,Comment),
        (Normal,'^') => (None,Upper),
        (Normal,'_') => (None,Lower),
        (Normal,other) => (Some(other),Normal),
        (Comment,'#') => (None,Normal),
        (Comment,_) => (None,Comment),
        (Upper,'^') => (None,Normal),
        (Upper,other) => (Some(other.to_ascii_uppercase()),Upper),
        (Lower,'_') => (None,Normal),
        (Lower,other) => (Some(other.to_ascii_lowercase()),Lower),
    }
}

fn main() {
    let mut state = State::Normal;
    let mut processed_string = String::new();
    let input = "This _IS_ some ^input^. #comment should not appear";
    for character in input.chars(){
        let (output,new_state) = state_machine(&state,character);
        if let Some(c) = output {
            processed_string.push(c);
        }
        state = new_state;
    }
    println!("{}",processed_string);
}