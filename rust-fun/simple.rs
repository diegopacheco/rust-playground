struct Point {
    x: int,
    y: int,
}

fn main() {
	let x: int;
	x = 5i;
    println!("Hello, world! x: {}", x);

    let origin = Point { x: 0i, y: 0i };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let y = 5i;
	match y {
	    1 => println!("one"),
	    2 => println!("two"),
	    3 => println!("three"),
	    4 => println!("four"),
	    5 => println!("five"),
	    _ => println!("something else"),
	}

	let input = std::io::stdin().read_line().ok().expect("Failed to read line");
    println!("You inputed: {}", input);

    hello::print_hello();

    let add_one = |x| { 1i + x };
	println!("The sum of 5 plus 1 is {}.", add_one(5i));

	for x in range(0i, 10i) {
    	println!("{:d}", x);
	}

}

mod hello {
    pub fn print_hello() {
        println!("Hello, world!");
    }
}