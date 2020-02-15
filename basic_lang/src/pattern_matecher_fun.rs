pub fn execute(){

    fn have_some(i:i32) -> &'static str{
        match i {
            0             => "no",
            1 | 2         => "some",
            10..=12       => "lots of",
            _ if (i%2==0) => "ah ah some",
            _             => "A few",
        }
    }

    for i in 1..13{
        println!("{}: I have {} apples.",i,have_some(i));
    }
}