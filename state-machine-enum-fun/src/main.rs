#[derive(Debug)]
enum State {
    Green,
    Yellow,
    Red,
}

impl State {
    pub fn new() -> State {
        Self::Green
    }
    pub fn next(self) -> Self {
        match self {
            Self::Green => Self::Yellow,
            Self::Yellow => Self::Red,
            Self::Red => Self::Green,
        }
    }
}

fn main() {
    let mut state = State::new(); 
    println!("{:?}", &state);

    for _  in 1..3{
        state = state.next();
        println!("{:?}", &state);         
    }
}
