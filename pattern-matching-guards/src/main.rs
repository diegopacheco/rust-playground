fn main() {
    let point = (10, -5);
    let complex_condition = true;

    match point {
        (x, y) if x > 0 && y < 0 && complex_condition => {
            println!("Point in quadrant IV with complex condition");
        }
        (x, _) if x > 0 => println!("Point has positive x"),
        (_, y) if y < 0 => println!("Point has negative y"),
        _ => println!("Point in other quadrants"),
    }
}